import datetime
import json
import logging
import logging.handlers
import pathlib
import sys

import PySide2
import PySide2.QtCore

from . import __version__, errors, helpers
from .applebooks.config import AppleBooksConfig


logger = logging.getLogger(__name__)


class GlobalConfigKeys:
    ENV = "env"


class GlobalConfig(PySide2.QtCore.QObject):

    modified = PySide2.QtCore.Signal()

    ENV_DEVELOPMENT: str = "DEVELOPMENT"
    ENV_PRODUCTION: str = "PRODUCTION"
    ENV_TESTING: str = "TESTING"
    ENV_DEFAULT: str = ENV_DEVELOPMENT

    TODAY: datetime.datetime = datetime.datetime.today()
    TODAY_PRETTY: str = TODAY.strftime("%Y-%m-%d")

    def __init__(self) -> None:
        super().__init__(parent=None)

        self.__data: dict = {
            GlobalConfigKeys.ENV: self.ENV_DEFAULT,
        }

        self.app: AppConfig = AppConfig(global_config=self)
        self.user: UserConfig = UserConfig(global_config=self)
        self.applebooks: AppleBooksConfig = AppleBooksConfig(global_config=self)

    @property
    def env(self) -> str:
        return self.__data[GlobalConfigKeys.ENV]

    @env.setter
    def env(self, value: str) -> None:
        self.__data[GlobalConfigKeys.ENV] = value

    @property
    def is_production(self) -> bool:
        """ See src.applebooks.config.AppleBooksConfig.SOURCE_DATABASES """

        if self.env == self.ENV_PRODUCTION:
            return True

        return False


class AppConfigKeys:
    pass


class AppConfig:

    VERSION: str = __version__

    NAME: str = "readstor"
    NAME_PRETTY: str = "ReadStor"
    NAME_PRETTY_VERSION: str = f"{NAME_PRETTY} v{VERSION}"

    COPYRIGHT_INFO: str = "Copyright © 2020 Shant Ergenian. All Rights Reserved"

    FILENAME_CONFIG: str = "config.json"
    FILENAME_LOGS: str = "logs.log"
    FILENAME_MANIFEST: str = "manifest.json"
    FILENAME_DATA: str = "data.json"

    DIRECTORY_NAME_DATA: str = "data"
    DIRECTORY_NAME_ITEMS_DATA: str = "items"
    DIRECTORY_NAME_DATABASES: str = "databases"
    DIRECTORY_NAME_MEDIA: str = "media"
    DIRECTORY_NAME_EXPORTS: str = "exports"
    DIRECTORY_NAME_TEXT: str = "text"
    DIRECTORY_NAME_MARKDOWN: str = "markdown"

    """
    SYSTEM_RESOURCES = pathlib.Path(
        "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources"
    )

    ALERT_STOP_ICON = SYSTEM_RESOURCES / "AlertStopIcon.icns"
    ALERT_CAUTION_ICON = SYSTEM_RESOURCES / "AlertCautionIcon.icns"
    ALERT_CAUTION_BADGE_ICON = SYSTEM_RESOURCES / "AlertCautionBadgeIcon.icns"
    ALERT_NOTE_ICON = SYSTEM_RESOURCES / "AlertNoteIcon.icns"
    """

    def __init__(self, global_config: GlobalConfig) -> None:

        self.__global_config = global_config

        self.PATH_SRC: pathlib.Path = pathlib.Path(__file__).parent.parent

        # /[application]/src/tests
        self.PATH_TESTS: pathlib.Path = self.PATH_SRC / "tests"

        # /[application]/src/data
        self.PATH_DATA: pathlib.Path = self.PATH_SRC / "data"

        # /[application]/src/readstor
        self.PATH_APPLICATION: pathlib.Path = self.PATH_SRC / "readstor"

        # /[application]/src/resources
        self.PATH_RESOURCES: pathlib.Path = self.PATH_APPLICATION / "resources"

        # /[application]/src/resources/templates
        self.PATH_TEMPLATES: pathlib.Path = self.PATH_RESOURCES / "templates"

        # /[application]/src/resources/images
        self.PATH_IMAGES: pathlib.Path = self.PATH_RESOURCES / "images"
        self.LOGO: pathlib.Path = self.PATH_IMAGES / "logo.png"
        self.MENUBAR_ICON_IDLE: pathlib.Path = self.PATH_IMAGES / "menubar-icon-idle.png"
        self.MENUBAR_ICON_BUSY: pathlib.Path = self.PATH_IMAGES / "menubar-icon-busy.png"

        #

        # /Users/[user]/.readstor/
        self.PATH_HOME: pathlib.Path = pathlib.Path.home() / f".{self.NAME}"

        # /Users/[user]/.readstor/config.json
        self.FILE_CONFIG: pathlib.Path = self.PATH_HOME / self.FILENAME_CONFIG

        # /Users/[user]/.readstor/logs.log
        self.FILE_LOGS: pathlib.Path = self.PATH_HOME / self.FILENAME_LOGS

    def setup(self) -> None:
        """ Creates required directories for running the application.

        /Users/[user]/.readstor (AppConfig.PATH_HOME)
        │ Path to application
        │
        ├── config.json
        └── logs.log
        """

        helpers.shell.make(path=self.PATH_HOME)

    def init_logger(self, log_level: str) -> None:

        stream_formatter = logging.Formatter(
            fmt="{threadName} {name} {levelname}: {message}", style="{",
        )
        stream_handler = logging.StreamHandler()
        stream_handler.setFormatter(stream_formatter)
        stream_handler.setLevel(log_level)

        #

        file_formatter = logging.Formatter(
            fmt="{asctime} {threadName} {name} {levelname}: {message}",
            datefmt="%Y-%m-%d %H:%M:%S",
            style="{",
        )
        file_handler = logging.handlers.RotatingFileHandler(
            self.FILE_LOGS, mode="a", maxBytes=1024 * 1024
        )
        file_handler.setFormatter(file_formatter)
        file_handler.setLevel(logging.DEBUG)

        #

        logging.basicConfig(
            level=log_level, handlers=[stream_handler, file_handler],
        )


class UserConfigKeys:
    PATH_STOR: str = "path_stor"
    EXPORT_MARKDOWN: str = "export_markdown"
    EXPORT_TEXT: str = "export_text"


class UserConfig:
    def __init__(self, global_config: GlobalConfig) -> None:

        self.__global_config = global_config

        self.__data: dict = {
            # Defaults to `/Users/[user]/.readstor`.
            UserConfigKeys.PATH_STOR: self.__global_config.app.PATH_HOME,
            UserConfigKeys.EXPORT_MARKDOWN: True,
            UserConfigKeys.EXPORT_TEXT: True,
        }

    def setup(self) -> None:
        """ Creates all required directories for saving user data.

        /path/to/stor
        │ Defaults to the the application root `/Users/[user]/.readstor`. When
        │ `UserConfig.load()` is called the value is over-written with the
        │ contents from `config.json`.
        │
        ├── data
        │   ├── manifest.json
        │   ├── items
        │   │   ├── item-01
        │   │   ├── item-02
        │   │   └── ...
        │   └── databases
        │       ├── 2020-01-01
        │       │   ├── AEAnnotation
        │       │   │   └── AEAnnotation###.sqlite
        │       │   └── BKLibrary
        │       │       └── BKLibrary###.sqlite
        │       ├── 2020-01-01
        │       ├── 2020-01-02
        │       └── ...
        └── exports
            ├── markdown
            │   ├── item-01
            │   ├── item-02
            │   └── ...
            └── text
                ├── item-01
                ├── item-02
                └── ...
        """

        directories = [
            self.path_stor,
            self.path_data,
            self.path_databases,
            self.path_exports_text,
            self.path_exports_markdown,
        ]

        for path in directories:
            helpers.shell.make(path=path)

    def load(self) -> None:

        logger.debug(
            f"Loading `{self.__global_config.app.FILE_CONFIG.name}` from "
            f"`{self.__global_config.app.FILE_CONFIG}`."
        )

        try:

            with open(self.__global_config.app.FILE_CONFIG, "r") as f:
                data: dict = json.load(f)

        except FileNotFoundError:

            logger.warning(
                f"Creating new `{self.__global_config.app.FILE_CONFIG.name}` in "
                f"`{self.__global_config.app.FILE_CONFIG.parent}`."
            )
            self._save()

        except json.JSONDecodeError:

            logger.error(
                f"Error reading `{self.__global_config.app.FILE_CONFIG.name}` in "
                f"`{self.__global_config.app.FILE_CONFIG.parent}`."
            )
            self._save()

        else:

            self.__data = self._deserialize(data=data)

            logger.debug(f"Configuration loaded as: {self.__data}")

    @property
    def path_stor(self) -> pathlib.Path:
        return self.__data[UserConfigKeys.PATH_STOR]

    @path_stor.setter
    def path_stor(self, path: pathlib.Path) -> None:

        path_previous: pathlib.Path = self.path_stor

        # Raises an ConfigurationError if the directory does not exists.
        try:
            path = pathlib.Path(path).resolve(strict=True)
        except FileNotFoundError:
            raise errors.ConfigurationError("Directory does not exist.")

        if path == path_previous:
            return

        # Raises an ConfigurationError if the directory is not empty.
        if any(path.iterdir()):
            raise errors.ConfigurationError("Directory must be empty.")

        self.__data[UserConfigKeys.PATH_STOR] = path
        self._save()

        self._move_stor(source=path_previous, destination=self.path_stor)

    @property
    def path_data(self) -> pathlib.Path:
        # /[user-stor]/data
        return self.path_stor / self.__global_config.app.DIRECTORY_NAME_DATA

    @property
    def path_items_data(self) -> pathlib.Path:
        # /[user-stor]/data/items
        return self.path_data / self.__global_config.app.DIRECTORY_NAME_ITEMS_DATA

    @property
    def file_manifest(self) -> pathlib.Path:
        # /[user-stor]/data/manifest.json
        return self.path_data / self.__global_config.app.FILENAME_MANIFEST

    @property
    def path_databases(self) -> pathlib.Path:
        # /[user-stor]/data/databases
        return self.path_data / self.__global_config.app.DIRECTORY_NAME_DATABASES

    @property
    def path_database_today(self) -> pathlib.Path:
        # /[user-stor]/data/databases/[date-today]
        return self.path_databases / self.__global_config.TODAY_PRETTY

    @property
    def path_exports(self) -> pathlib.Path:
        # /[user-stor]/exports
        return self.path_stor / self.__global_config.app.DIRECTORY_NAME_EXPORTS

    @property
    def path_exports_text(self) -> pathlib.Path:
        # /[user-stor]/exports/text
        return self.path_exports / self.__global_config.app.DIRECTORY_NAME_TEXT

    @property
    def path_exports_markdown(self) -> pathlib.Path:
        # /[user-stor]/exports/markdown
        return self.path_exports / self.__global_config.app.DIRECTORY_NAME_MARKDOWN

    @property
    def export_markdown(self) -> bool:
        return self.__data[UserConfigKeys.EXPORT_MARKDOWN]

    @export_markdown.setter
    def export_markdown(self, value: bool) -> None:
        self.__data[UserConfigKeys.EXPORT_MARKDOWN] = value
        self._save()

    @property
    def export_text(self) -> bool:
        return self.__data[UserConfigKeys.EXPORT_TEXT]

    @export_text.setter
    def export_text(self, value: bool) -> None:
        self.__data[UserConfigKeys.EXPORT_TEXT] = value
        self._save()

    def _save(self) -> None:

        logger.debug(
            f"Saving `{self.__global_config.app.FILE_CONFIG.name}` to "
            f"`{self.__global_config.app.FILE_CONFIG}`."
        )

        # Write /Users/[user]/.readstor/config.json
        with open(self.__global_config.app.FILE_CONFIG, "w") as f:
            json.dump(self._serialize(), f, sort_keys=False, indent=4)

        logger.debug(f"Configuration saved as: {self.__data}")

        self.__global_config.modified.emit()

    def _move_stor(self, source: pathlib.Path, destination: pathlib.Path) -> None:

        logger.debug(f"Moving stor from: `{source}` to `{destination}`.")

        for item in source.iterdir():

            if item.name == self.__global_config.app.FILENAME_CONFIG:
                continue

            if item.name == self.__global_config.app.FILENAME_LOGS:
                continue

            helpers.shell.move(source=item, destination=destination)

    def _serialize(self) -> dict:

        return {
            UserConfigKeys.PATH_STOR: str(self.path_stor),
            UserConfigKeys.EXPORT_MARKDOWN: self.export_markdown,
            UserConfigKeys.EXPORT_TEXT: self.export_text,
        }

    def _deserialize(self, data: dict) -> dict:

        # fmt: off
        data_root: str = data.get(UserConfigKeys.PATH_STOR, self.__global_config.app.PATH_HOME)
        export_markdown: bool = data.get(UserConfigKeys.EXPORT_MARKDOWN, True)
        export_text: bool = data.get(UserConfigKeys.EXPORT_TEXT, True)
        # fmt: on

        return {
            UserConfigKeys.PATH_STOR: pathlib.Path(data_root),
            UserConfigKeys.EXPORT_MARKDOWN: export_markdown,
            UserConfigKeys.EXPORT_TEXT: export_text,
        }


config = GlobalConfig()
