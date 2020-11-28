import logging
import pathlib
import sqlite3
from typing import Iterator, List

from readstor import errors
from readstor.config import GlobalConfig
from readstor.stor.models import AnnotationKeys, SourceKeys, StorItemKeys


logger = logging.getLogger(__name__)


class AppleBooksDatabase:
    def serialize(self) -> dict:
        """Converts database's lists of dictionaries to a single dictionary.

        Sources:

            [
                {
                    "id": "SOURCE-ID000",
                    "title": "Source-000 Title",
                    "author": "Source-000 Author",
                    "date_last_opened": 000000000.000000,
                    "path_epub": "/path/to/source-000.epub",
                    ...
                },
                {
                    "id": "SOURCE-ID001",
                    "title": "Source-001 Title",
                    "author": "Source-001 Author",
                    "date_last_opened": 000000000.000000,
                    "path_epub": "/path/to/source-001.epub",
                    ...
                },
                ...
            ]

        Annotations:

            [
                {
                    "source_id": "SOURCE-ID000",
                    "id": "ANNOTATION-ID",
                    "body": "Lorem ipsum...",
                    ...
                },
                {
                    "source_id": "SOURCE-ID001",
                    "id": "ANNOTATION-ID",
                    "body": "Lorem ipsum...",
                    ...
                },
                ...
            ]

        Returns:

            {
                "SOURCE-ID000": {
                    "date_last_opened": 000000000.000000,
                    "path_epub": "/path/to/source-000.epub",
                    "source": {
                        "id": "SOURCE-ID000",
                        "title": "Source-000 Title",
                        "author": "Source-000 Author",
                    }
                    "annotations": [
                        {
                            "source_id": "SOURCE-ID000",
                            "id": "ANNOTATION-ID",
                            "body": "Lorem ipsum...",
                            ...
                        },
                        ...
                    ],
                },
                "SOURCE-ID001": {
                    "date_last_opened": 000000000.000000,
                    "path_epub": "/path/to/source-001.epub",
                    "source": {
                        "id": "SOURCE-ID001",
                        "title": "Source-001 Title",
                        "author": "Source-001 Author",
                    }
                    "annotations": [
                        {
                            "source_id": "SOURCE-ID001",
                            "id": "ANNOTATION-ID",
                            "body": "Lorem ipsum...",
                            ...
                        },
                        ...
                    ],
                },
            }

        """

        data: dict = {}

        sources = self._query_sources()
        annotations = self._query_annotations()

        for source in sources:

            source_id = source.get(SourceKeys.ID)
            source_title = source.get(SourceKeys.TITLE)
            source_author = source.get(SourceKeys.AUTHOR)

            date_last_opened = source.get(StorItemKeys.DATE_LAST_OPENED)
            path_epub = source.get(StorItemKeys.PATH_EPUB)

            data[source_id] = {}
            data[source_id][StorItemKeys.SOURCE] = {
                SourceKeys.ID: source_id,
                SourceKeys.TITLE: source_title,
                SourceKeys.AUTHOR: source_author,
            }
            data[source_id][StorItemKeys.ANNOTATIONS] = []
            data[source_id][StorItemKeys.DATE_LAST_OPENED] = date_last_opened
            data[source_id][StorItemKeys.PATH_EPUB] = path_epub

            for annotation in annotations:

                # fmt:off
                annotation_source_id = annotation.get(AnnotationKeys.SOURCE_ID)

                if annotation_source_id == source_id:
                    data[source_id][StorItemKeys.ANNOTATIONS].append(annotation)
                # fmt:on

            if not data[source_id][StorItemKeys.ANNOTATIONS]:
                del data[source_id]

        return data

    def _query_sources(self) -> List[dict]:

        query = f"""
            SELECT
                ZBKLIBRARYASSET.ZASSETID as {SourceKeys.ID},
                ZBKLIBRARYASSET.ZTITLE as {SourceKeys.TITLE},
                ZBKLIBRARYASSET.ZAUTHOR as {SourceKeys.AUTHOR},
                ZBKLIBRARYASSET.ZLASTOPENDATE as {StorItemKeys.DATE_LAST_OPENED},
                ZBKLIBRARYASSET.ZPATH as {StorItemKeys.PATH_EPUB}
            FROM ZBKLIBRARYASSET

            ORDER BY ZBKLIBRARYASSET.ZTITLE;
        """

        return self._query_database(database=self._sources_database, query=query)

    def _query_annotations(self) -> List[dict]:

        query = f"""
            SELECT
                ZAEANNOTATION.ZANNOTATIONASSETID as {AnnotationKeys.SOURCE_ID},
                ZANNOTATIONUUID as {AnnotationKeys.ID},
                ZANNOTATIONSELECTEDTEXT as {AnnotationKeys.BODY},
                ZANNOTATIONNOTE as {AnnotationKeys.NOTES},
                ZANNOTATIONSTYLE as {AnnotationKeys.STYLE_INDEX},
                ZANNOTATIONLOCATION as {AnnotationKeys.EPUBCFI},
                ZANNOTATIONCREATIONDATE as {AnnotationKeys.DATE_CREATED},
                ZANNOTATIONMODIFICATIONDATE as {AnnotationKeys.DATE_MODIFIED}

            FROM ZAEANNOTATION

            WHERE ZANNOTATIONSELECTEDTEXT IS NOT NULL
                AND ZANNOTATIONDELETED = 0

            ORDER BY ZANNOTATIONASSETID;
        """

        return self._query_database(database=self._annotations_database, query=query)

    @property
    def _sources_database(self) -> pathlib.Path:
        return self._get_database(prefix=GlobalConfig.applebooks.NAME_BKLIBRARY)

    @property
    def _annotations_database(self) -> pathlib.Path:
        return self._get_database(prefix=GlobalConfig.applebooks.NAME_AEANNOTATION)

    def _get_database(self, prefix: str) -> pathlib.Path:
        """Returns the path to an Apple Books database with `prefix` in:
        /[user-stor]/data/databases/[date-today]."""

        glob: str = f"{prefix}*.sqlite"

        path_database_today = GlobalConfig.user.path_database_today
        paths_database: Iterator[pathlib.Path] = path_database_today.glob(glob)

        try:
            return list(paths_database)[0]
        except IndexError:
            # `IndexError` means no database file was found and by extension
            # not in the copied to the `path_database_today` directory. Meaning
            # that Apple Books has changed the some of its directory structure.
            # The currently installed version of Apple Books is therefore
            # unsupported.
            logger.error(
                f"Couldn't find Apple Books database in `{path_database_today}`."
            )
            raise errors.ApplicationError()

    def _query_database(self, database: pathlib.Path, query: str) -> List[dict]:

        connection: sqlite3.Connection = sqlite3.connect(database)
        connection.row_factory = self._dict_factory

        with connection:
            cursor: sqlite3.Cursor = connection.cursor()

            try:
                cursor.execute(query)
            except sqlite3.OperationalError:
                # `OperationalError` the query is no longer valid. The database
                # structure has possibly changed. The currently installed
                # version of Apple Books is therefore unsupported.
                logger.exception(
                    f"Exception raised while attempting to execute query on"
                    f"database: `{database}`."
                )
                raise errors.ApplicationError()

            data: List[dict] = cursor.fetchall()

        return data

    def _dict_factory(self, cursor: sqlite3.Cursor, row: tuple) -> dict:
        """Reconfigures sqlite.Connection.row_factory to return a dictionary
        instead.

        https://docs.python.org/3/library/sqlite3.html#sqlite3.Connection.row_factory
        """

        data: dict = {}

        for index, column in enumerate(cursor.description):
            data[column[0]] = row[index]

        return data
