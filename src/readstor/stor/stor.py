import datetime
import json
import logging
import pathlib
from typing import Dict, Optional

from readstor import helpers
from readstor.applebooks import database
from readstor.config import GlobalConfig

from . import exporter, models
from .mixins import DateTimeUtilsMixin


logger = logging.getLogger(__name__)


class Stor(DateTimeUtilsMixin):
    """The basic structure of this module.

    <Stor>
     │  │ Sets up directories, copies databases and contains methods to perform
     │  │ on Apple Books i.e: `is_running()` and `quit()`. Uses
     │  │ <AppleBooksDatabase> class to create and update stor items from the
     │  │ Apple Books databases (BKLibrary & AEAnnotation).
     │  │
     │  ├── <StorItem>
     │  │    │ Container class to manage directories and filenames for each
     │  │    │ <Source> and <Annotation> pair.
     │  │    │
     │  │    ├── <Source>
     │  │    │ Contains and manages data from a single Source (Book).
     │  │    │
     │  │    ├── <Annotation>
     │  │    │ Contains and manages data from a single Annotation.
     │  │    │
     │  │    ├── <Annotation>
     │  │    ├── <Annotation>
     │  │    └── ...
     │  │
     │  ├── <StorItem>
     │  ├── <StorItem>
     │  └── ...
     │
     └── <Exporter>
       Manages Jinja template environment to export data to various file
       formats.

    #

    Data Folder Structure

    /path/to/data
    ├── manifest.json
    └── items
        ├── item-01
        │   └── data.json
        ├── item-02
        │   └── data.json
        └── ...
    """

    DIRECTORY_NAME_ITEMS_DATA: str = "items"

    FILENAME_MANIFEST: str = "manifest.json"
    FILENAME_DATA: str = "data.json"

    __manifest: Dict[str, datetime.datetime] = {}

    def __init__(self) -> None:

        self._exporter = exporter.Exporter()

    def stor(self) -> None:
        """Primary public method. Runs a series of functions to
        backup/query/save data from the Apple Books databases."""

        self._remake_directory_database_today()
        self._copy_source_applebooks_databases()

        self._load_manifest()
        self._process_applebooks_database()
        self._save_manifest()

    @property
    def path_items_data(self) -> pathlib.Path:
        # /[user-stor]/data/items
        return GlobalConfig.user.path_data / self.DIRECTORY_NAME_ITEMS_DATA

    @property
    def file_manifest(self) -> pathlib.Path:
        # /[user-stor]/data/manifest.json
        return GlobalConfig.user.path_data / self.FILENAME_MANIFEST

    def _remake_directory_database_today(self) -> None:
        """Re-makes today's database backup directory in case
        `AppleBooks.stor()` is run more than once a day."""

        helpers.shell.remove(path=GlobalConfig.user.path_database_today)
        helpers.shell.make(path=GlobalConfig.user.path_database_today)

    def _copy_source_applebooks_databases(self) -> None:
        """Copies both BKLibrary###.sqlite and AEAnnotation###.sqlite to
        /[user-stor]/data/databases/[date-today]."""

        for item in GlobalConfig.applebooks.PATH_BKLIBRARY.iterdir():

            if not item.is_file():
                continue

            if item.name.startswith(GlobalConfig.applebooks.NAME_BKLIBRARY):
                helpers.shell.copy(
                    sources=[item],
                    destination=GlobalConfig.user.path_database_today,
                )

        for item in GlobalConfig.applebooks.PATH_AEANNOTATION.iterdir():

            if not item.is_file():
                continue

            if item.name.startswith(GlobalConfig.applebooks.NAME_AEANNOTATION):
                helpers.shell.copy(
                    sources=[item],
                    destination=GlobalConfig.user.path_database_today,
                )

    def _load_manifest(self) -> None:
        """Loads the manifest file from /[user-stor]/data/manifest.json.

        This file contains a dictionary of `id:date` key-value pairs referring
        to a `AppleBooksStoreItem.source.id` the last time its respective book
        was opened. This information is used to determine wheather add a new
        book, update an existing one or skip it.

        NOTE: `AppleBooksStoreItem.source.id` is the unique identifier given to
        a book in Apple Books."""

        logger.debug(
            f"Loading `{self.file_manifest.name}` from `{GlobalConfig.user.path_data}`."
        )

        try:

            with open(self.file_manifest, "r") as f:
                data = json.load(f)

        except FileNotFoundError:

            logger.warning(
                f"Creating new `{self.file_manifest.name}` in "
                f"`{GlobalConfig.user.path_data}`."
            )
            self._save_manifest()

        except json.JSONDecodeError:

            logger.error(
                f"Error reading `{self.file_manifest.name}` in "
                f"`{GlobalConfig.user.path_data}`."
            )
            self._save_manifest()

        else:

            self.__manifest = self._deserialize_manifest(data=data)

        logger.debug(f"Manifest contains {len(self.__manifest)} items.")

    def _save_manifest(self) -> None:
        """ Saves the manifest file to /[user-stor]/data/manifest.json. """

        logger.debug(
            f"Saving `{self.file_manifest.name}` to `{GlobalConfig.user.path_data}`."
        )

        with open(self.file_manifest, "w") as f:
            json.dump(self._serialize_manifest(), f, sort_keys=False, indent=4)

    def _process_applebooks_database(self) -> None:

        database_data: dict = database.AppleBooksDatabase().serialize()

        logger.info(f"Processing {len(database_data)} items from Apple Books database.")

        for data in database_data.values():

            item: Optional[models.StorItem] = models.StorItem.deserialize(data=data)

            if item is None:
                logger.error(f"Error creating models.AppleBooksStorItem with:\n{data}")
                continue

            # Add any items not in manifest.
            if item.source.id not in self.__manifest.keys():
                logger.info(f"Added {item.source.name_pretty}.")
                self._add_update_item(item=item)
                self._exporter.export(item=item)
                continue

            date_last_updated = self.__manifest[item.source.id]

            # Update items found in manifest that have been opened
            # (modified/read) since their last refresh.
            if item.date_last_opened > date_last_updated:
                logger.info(f"Updated {item.source.name_pretty}.")
                self._add_update_item(item=item)
                self._exporter.export(item=item)
                continue

            # Skip items that are already in the manifest and have not been
            # opened (modified/read) since their last refresh.

    def _add_update_item(self, item) -> None:

        # Add/update the item in the manifest with the current datetime.
        self.__manifest[item.source.id] = datetime.datetime.now()

        # /[user-stor]/data/items/[Title-by-Author]
        path_data = self.path_items_data / item.name

        # /[user-stor]/data/items/[Title-by-Author]/data.json
        file_data = path_data / self.FILENAME_DATA

        helpers.shell.make(path=path_data)

        with open(file_data, "w", encoding="utf-8") as f:
            json.dump(item.serialize(), f, sort_keys=False, indent=4)

    def _serialize_manifest(self) -> dict:
        """ Converts `str:datetime.datetime` to `str:str` """

        data: dict = {}

        for item_source_id, date_last_updated in self.__manifest.items():
            data[item_source_id] = date_last_updated.isoformat()

        return data

    def _deserialize_manifest(self, data: dict) -> dict:
        """ Converts `str:str` to `str:datetime.datetime` """

        data_: dict = {}

        for item_source_id, date_last_updated in data.items():
            data_[item_source_id] = self.datetime_from_iso(iso=date_last_updated)

        return data_

    def is_running(self) -> bool:
        """ Checks to see if Apple Books is currently running. """
        return helpers.shell.process_is_running(
            process_names=GlobalConfig.applebooks.NAMES
        )

    def quit(self) -> None:
        """ Kindly asks Apple Books to quit. """
        helpers.shell.run(
            [
                "osascript",
                "-e",
                f'tell application "{GlobalConfig.applebooks.NAME}" to quit',
            ]
        )
