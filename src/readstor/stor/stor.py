import datetime
import json
import logging
from typing import Dict, Optional

from readstor import helpers
from readstor.applebooks import database
from readstor.config import config

from . import exporter, models
from .mixins import DateTimeUtilsMixin


logger = logging.getLogger(__name__)


class Stor(DateTimeUtilsMixin):
    """ The basic structure of this module.

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
       formats. """

    __manifest: Dict[str, datetime.datetime] = {}

    def __init__(self) -> None:

        self._exporter = exporter.Exporter()

    def stor(self) -> None:
        """ Primary public method. Runs a series of functions to
        backup/query/save data from the Apple Books databases. """

        self._remake_directory_database_today()
        self._copy_source_applebooks_databases()

        self._load_manifest()
        self._process_applebooks_database()
        self._save_manifest()

    def _remake_directory_database_today(self) -> None:
        """ Re-makes today's database backup directory in case
        `AppleBooks.stor()` is run more than once a day. """

        helpers.shell.remove(path=config.user.path_database_today)
        helpers.shell.make(path=config.user.path_database_today)

    def _copy_source_applebooks_databases(self) -> None:
        """ Copies both BKLibrary###.sqlite and AEAnnotation###.sqlite to
        /[user-stor]/data/databases/[date-today]. """

        for item in config.applebooks.PATH_BKLIBRARY.iterdir():

            if not item.is_file():
                continue

            if item.name.startswith(config.applebooks.NAME_BKLIBRARY):
                helpers.shell.copy(
                    sources=[item], destination=config.user.path_database_today,
                )

        for item in config.applebooks.PATH_AEANNOTATION.iterdir():

            if not item.is_file():
                continue

            if item.name.startswith(config.applebooks.NAME_AEANNOTATION):
                helpers.shell.copy(
                    sources=[item], destination=config.user.path_database_today,
                )

    def _load_manifest(self) -> None:
        """ Loads the manifest file from /[user-stor]/data/manifest.json.

        This file contains a dictionary of `id:date` key-value pairs referring
        to a `AppleBooksStoreItem.source.id` the last time its respective book
        was opened. This information is used to determine wheather add a new
        book, update an existing one or skip it.

        NOTE: `AppleBooksStoreItem.source.id` is the unique identifier given to
        a book in Apple Books. """

        logger.debug(
            f"Loading `{config.user.file_manifest.name}` from `{config.user.path_data}`."
        )

        try:

            with open(config.user.file_manifest, "r") as f:
                data = json.load(f)

        except FileNotFoundError:

            logger.warning(
                f"Creating new `{config.user.file_manifest.name}` in "
                f"`{config.user.path_data}`."
            )
            self._save_manifest()

        except json.JSONDecodeError:

            logger.error(
                f"Error reading `{config.user.file_manifest.name}` in "
                f"`{config.user.path_data}`."
            )
            self._save_manifest()

        else:

            self.__manifest = self._deserialize_manifest(data=data)

        logger.debug(f"Manifest contains {len(self.__manifest)} items.")

    def _save_manifest(self) -> None:
        """ Saves the manifest file to /[user-stor]/data/manifest.json. """

        logger.debug(
            f"Saving `{config.user.file_manifest.name}` to `{config.user.path_data}`."
        )

        with open(config.user.file_manifest, "w") as f:
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
                continue

            date_last_updated = self.__manifest[item.source.id]

            # Update items found in manifest that have been opened
            # (modified/read) since their last refresh.
            if item.date_last_opened > date_last_updated:
                logger.info(f"Updated {item.source.name_pretty}.")
                self._add_update_item(item=item)
                continue

            # Skip items that are already in the manifest and have not been
            # opened (modified/read) since their last refresh.

    def _add_update_item(self, item) -> None:

        # Add/update the item in the manifest with the current datetime.
        self.__manifest[item.source.id] = datetime.datetime.now()

        # Make /[user-stor]/data/items/[title-by-author-xxxxxx]
        helpers.shell.make(path=item.path_item_data)

        # Make /[user-stor]/data/items/[title-by-author-xxxxxx]/media
        helpers.shell.make(path=item.path_media)

        # Write /[user-stor]/data/items/[title-by-author-xxxxxx]/data.json
        with open(item.file_data, "w", encoding="utf-8") as f:
            json.dump(item.serialize(), f, sort_keys=False, indent=4)

        self._exporter.export(item=item)

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
        return helpers.shell.process_is_running(process_names=config.applebooks.NAMES)

    def quit(self) -> None:
        """ Kindly asks Apple Books to quit. """
        helpers.shell.run(
            ["osascript", "-e", f'tell application "{config.applebooks.NAME}" to quit',]
        )
