import pathlib
from typing import TYPE_CHECKING


if TYPE_CHECKING:
    from readstor.config import _GlobalConfig


class _AppleBooksConfig:

    VERSION = "Books 2.0 (1841)"

    NAME = "Books"
    NAMES = [NAME, "iBooks", "Apple Books", "AppleBooks"]

    NAME_BKLIBRARY = "BKLibrary"
    NAME_AEANNOTATION = "AEAnnotation"

    def __init__(self, global_config: "_GlobalConfig") -> None:

        self.__global_config = global_config

    @property
    def PATH_SOURCE_DATABASES(self) -> pathlib.Path:
        """Returns the database directory depending on if the application is
        running in production mode or not.

        Development and production Apple Books data directories. Both containing
        `BKLibrary` and `AEAnnotation` directories with each containing their
        respective `.sqlite` files.

            Development: /[application]/src/data/databases
            Production: /Library/Containers/com.apple.iBooksX/Data/Documents"""

        databases_development = self.__global_config.app.PATH_DATA / "databases"
        databases_production = (
            pathlib.Path.home() / "Library/Containers/com.apple.iBooksX/Data/Documents"
        )

        return (
            databases_production
            if self.__global_config.is_production
            else databases_development
        )

    @property
    def PATH_BKLIBRARY(self) -> pathlib.Path:
        return self.PATH_SOURCE_DATABASES / self.NAME_BKLIBRARY

    @property
    def PATH_AEANNOTATION(self) -> pathlib.Path:
        return self.PATH_SOURCE_DATABASES / self.NAME_AEANNOTATION
