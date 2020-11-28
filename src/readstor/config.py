import datetime
import json
import logging
import logging.handlers
import pathlib

import PySide2
import PySide2.QtCore
import PySide2.QtWidgets

from . import __version__, errors, helpers
from .applebooks.config import _AppleBooksConfig


logger = logging.getLogger(__name__)


class GlobalConfigKeys:
    ENV = "env"


class _GlobalConfig(PySide2.QtCore.QObject):

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

        self.app = _AppConfig(global_config=self)
        self.user = _UserConfig(global_config=self)
        self.applebooks = _AppleBooksConfig(global_config=self)

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


class _AppConfig:

    VERSION: str = __version__

    NAME: str = "readstor"
    NAME_PRETTY: str = "ReadStor"
    NAME_PRETTY_VERSION: str = f"{NAME_PRETTY} v{VERSION}"

    COPYRIGHT_INFO: str = "Copyright © 2020 Shant Ergenian. All Rights Reserved"

    FILENAME_CONFIG: str = "config.json"
    FILENAME_LOGS: str = "logs.log"

    DIRECTORY_NAME_DATA: str = "data"
    DIRECTORY_NAME_DATABASES: str = "databases"
    DIRECTORY_NAME_EXPORTS: str = "exports"

    def __init__(self, global_config: _GlobalConfig) -> None:

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
        self.MENUBAR_ICON_IDLE: pathlib.Path = (
            self.PATH_IMAGES / "menubar-icon-idle.png"
        )
        self.MENUBAR_ICON_BUSY: pathlib.Path = (
            self.PATH_IMAGES / "menubar-icon-busy.png"
        )

        #

        # /Users/[user]/.readstor/
        self.PATH_HOME: pathlib.Path = pathlib.Path.home() / f".{self.NAME}"

        # /Users/[user]/.readstor/config.json
        self.FILE_CONFIG: pathlib.Path = self.PATH_HOME / self.FILENAME_CONFIG

        # /Users/[user]/.readstor/logs.log
        self.FILE_LOGS: pathlib.Path = self.PATH_HOME / self.FILENAME_LOGS

    def setup(self) -> None:
        """Creates required directories for running the application.

        /Users/[user]/.readstor (AppConfig.PATH_HOME)
        │ Path to application
        │
        ├── config.json
        └── logs.log
        """

        helpers.shell.make(path=self.PATH_HOME)

    def init_logger(self, log_level: str) -> None:

        stream_formatter = logging.Formatter(
            fmt="{threadName} {name} {levelname}: {message}",
            style="{",
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
            level=log_level,
            handlers=[stream_handler, file_handler],
        )

    @property
    def pixel_ratio(self):
        return PySide2.QtWidgets.QApplication.instance().devicePixelRatio()


class UserConfigKeys:
    PATH_STOR: str = "path_stor"
    EXPORT_FLAT: str = "export_flat"
    EXPORT_NESTED: str = "export_nested"


class _UserConfig:
    def __init__(self, global_config: _GlobalConfig) -> None:

        self.__global_config = global_config

        self.__data: dict = {
            # Defaults to `/Users/[user]/.readstor`.
            UserConfigKeys.PATH_STOR: self.__global_config.app.PATH_HOME,
            UserConfigKeys.EXPORT_FLAT: True,
            UserConfigKeys.EXPORT_NESTED: True,
        }

    def setup(self) -> None:
        """Creates all required directories for saving user data.

        /path/to/stor
        │ Defaults to the the application root `/Users/[user]/.readstor`. When
        │ `UserConfig.load()` is called the value is over-written with the
        │ contents from `config.json`.
        │
        ├── data
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
        """

        directories = [
            self.path_stor,
            self.path_data,
            self.path_databases,
            self.path_exports,
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
    def export_flat(self) -> bool:
        return self.__data[UserConfigKeys.EXPORT_FLAT]

    @export_flat.setter
    def export_flat(self, value: bool) -> None:
        self.__data[UserConfigKeys.EXPORT_FLAT] = value
        self._save()

    @property
    def export_nested(self) -> bool:
        return self.__data[UserConfigKeys.EXPORT_NESTED]

    @export_nested.setter
    def export_nested(self, value: bool) -> None:
        self.__data[UserConfigKeys.EXPORT_NESTED] = value
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
            UserConfigKeys.EXPORT_FLAT: self.export_flat,
            UserConfigKeys.EXPORT_NESTED: self.export_nested,
        }

    def _deserialize(self, data: dict) -> dict:

        # fmt: off
        data_root: str = data.get(UserConfigKeys.PATH_STOR, self.__global_config.app.PATH_HOME)
        export_flat: bool = data.get(UserConfigKeys.EXPORT_FLAT, True)
        export_nested: bool = data.get(UserConfigKeys.EXPORT_NESTED, True)
        # fmt: on

        return {
            UserConfigKeys.PATH_STOR: pathlib.Path(data_root),
            UserConfigKeys.EXPORT_FLAT: export_flat,
            UserConfigKeys.EXPORT_NESTED: export_nested,
        }


GlobalConfig = _GlobalConfig()
