import datetime
import logging
import pathlib
from typing import List, Optional

from readstor import helpers
from readstor.applebooks.mixins import AppleBooksUtilsMixin
from readstor.config import config

from .mixins import DateTimeUtilsMixin, EPUBUtilsMixin


logger = logging.getLogger(__name__)


class AnnotationKeys:

    SOURCE_ID = "source_id"
    ID = "id"
    BODY = "body"
    NOTES = "notes"
    METADATA = "metadata"
    STYLE = "style"
    STYLE_INDEX = "style_index"
    EPUBCFI = "epubcfi"
    LOCATION = "location"
    DATE_CREATED = "date_created"
    DATE_MODIFIED = "date_modified"


class Annotation(AppleBooksUtilsMixin, EPUBUtilsMixin, DateTimeUtilsMixin):
    def __init__(
        self,
        id: str,
        body: Optional[str],
        notes: Optional[str],
        source_id: str,
        date_created: Optional[float],
        date_modified: Optional[float],
        style_index: Optional[int],
        epubcfi: Optional[str],
    ) -> None:

        self._id = id
        self._body = self._process_body(data=body)
        self._notes = self._process_notes(data=notes)
        self._source_id = source_id
        self._date_created = self.datetime_from_epoch(epoch=date_created)
        self._date_modified = self.datetime_from_epoch(epoch=date_modified)
        self._style_index = style_index
        self._epubcfi = epubcfi

        self._style = self.style_from_index(index=style_index)
        self._location = self.location_from_epubcfi(epubcfi=epubcfi)

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}: {self.id}>"

    def __str__(self) -> str:
        return f"<{self.__class__.__name__}: `{self.body[:50].strip()}...`>"

    @classmethod
    def deserialize(cls, data: dict) -> Optional["Annotation"]:

        id_ = data.get(AnnotationKeys.ID)
        body = data.get(AnnotationKeys.BODY)
        notes = data.get(AnnotationKeys.NOTES)
        source_id = data.get(AnnotationKeys.SOURCE_ID)
        date_created = data.get(AnnotationKeys.DATE_CREATED)
        date_modified = data.get(AnnotationKeys.DATE_MODIFIED)
        style_index = data.get(AnnotationKeys.STYLE_INDEX)
        epubcfi = data.get(AnnotationKeys.EPUBCFI)

        if id_ is None or source_id is None:
            logger.error(
                f"{cls.__name__}.deserialize() received invalid data. "
                f"Missing `id` or `source_id`.\n{data}"
            )
            return None

        return cls(
            id=id_,
            body=body,
            notes=notes,
            source_id=source_id,
            date_created=date_created,
            date_modified=date_modified,
            style_index=style_index,
            epubcfi=epubcfi,
        )

    def _process_body(self, data: Optional[str]) -> str:

        if data is None:
            return ""

        # Strip trailing new lines and split into paragraphs.
        data = data.splitlines(keepends=False)  # type:ignore

        # Strip trailing whitespace.
        data = [p.strip() for p in data]  # type:ignore

        # Remove any empty items in list.
        data = list(filter(None, data))  # type:ignore

        return "\n".join(data)

    def _process_notes(self, data: Optional[str]) -> str:

        if data is None:
            return ""

        data = data.strip()

        return data

    @property
    def id(self) -> str:
        return self._id

    @property
    def body(self) -> str:
        return self._body

    @property
    def source_id(self) -> str:
        return self._source_id

    @property
    def notes(self) -> str:
        return self._notes

    @property
    def date_created(self) -> datetime.datetime:
        return self._date_created

    @property
    def date_modified(self) -> datetime.datetime:
        return self._date_modified

    @property
    def style(self) -> str:
        return self._style

    @property
    def style_index(self) -> Optional[int]:
        return self._style_index

    @property
    def epubcfi(self) -> Optional[str]:
        return self._epubcfi

    @property
    def location(self) -> str:
        return self._location

    def serialize(self) -> dict:
        return {
            AnnotationKeys.ID: self._id,
            AnnotationKeys.BODY: self._body,
            AnnotationKeys.NOTES: self._notes,
            AnnotationKeys.SOURCE_ID: self._source_id,
            AnnotationKeys.METADATA: {
                AnnotationKeys.DATE_CREATED: self._date_created.isoformat(),
                AnnotationKeys.DATE_MODIFIED: self._date_modified.isoformat(),
                AnnotationKeys.STYLE: self._style,
                AnnotationKeys.STYLE_INDEX: self._style_index,
                AnnotationKeys.EPUBCFI: self._epubcfi,
                AnnotationKeys.LOCATION: self._location,
            },
        }


class SourceKeys:

    ID = "id"
    TITLE = "title"
    AUTHOR = "author"


class Source(DateTimeUtilsMixin):
    def __init__(self, id: str, title: Optional[str], author: Optional[str]) -> None:

        self._id = id
        self._title = title if title is not None else ""
        self._author = author if author is not None else ""

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}: {self.id}>"

    def __str__(self) -> str:
        return f"<{self.__class__.__name__}: `{self.name_pretty}`>"

    @classmethod
    def deserialize(cls, data: dict) -> Optional["Source"]:

        id_ = data.get(SourceKeys.ID)
        title = data.get(SourceKeys.TITLE)
        author = data.get(SourceKeys.AUTHOR)

        if id_ is None:
            logger.error(
                f"{cls.__name__}.deserialize() received invalid data. "
                f"Missing `id`.\n{data}"
            )
            return None

        return cls(id=id_, title=title, author=author)

    @property
    def id(self) -> str:
        return self._id

    @property
    def title(self) -> str:
        return self._title

    @property
    def author(self) -> str:
        return self._author

    @property
    def name_pretty(self) -> str:
        return f"{self.title} by {self.author}"

    def serialize(self) -> dict:
        return {
            SourceKeys.ID: self._id,
            SourceKeys.TITLE: self._title,
            SourceKeys.AUTHOR: self._author,
        }


class StorItemKeys:

    NAME = "name"
    SOURCE = "source"
    ANNOTATIONS = "annotations"
    DATE_LAST_OPENED = "date_last_opened"
    PATH_EPUB = "path_epub"


class StorItem(DateTimeUtilsMixin):
    def __init__(
        self,
        source: Source,
        annotations: List[Annotation],
        date_last_opened: Optional[float],
        path_epub: Optional[str],
    ) -> None:

        self._source = source
        self._annotations = annotations
        self._date_last_opened = self.datetime_from_epoch(epoch=date_last_opened)
        self._path_epub = (
            pathlib.Path(path_epub) if path_epub is not None else pathlib.Path("")
        )

    @classmethod
    def deserialize(cls, data: dict) -> Optional["StorItem"]:

        source = data.get(StorItemKeys.SOURCE)
        annotations = data.get(StorItemKeys.ANNOTATIONS)
        date_last_opened = data.get(StorItemKeys.DATE_LAST_OPENED)
        path_epub = data.get(StorItemKeys.PATH_EPUB)

        if not source or not annotations:
            return None

        source_obj: Optional[Source] = Source.deserialize(source)

        if source_obj is None:
            return None

        #

        annotation_objs: List[Annotation] = []

        for annotation in annotations:
            annotation_obj = Annotation.deserialize(annotation)

            if annotation_obj is None:
                continue

            annotation_objs.append(annotation_obj)

        #

        return cls(
            source=source_obj,
            annotations=annotation_objs,
            date_last_opened=date_last_opened,
            path_epub=path_epub,
        )

    def __repr__(self) -> str:
        return (
            f"<{self.__class__.__name__}: source:{self._source.id} "
            f"annotation_count:{len(self._annotations):04}>"
        )

    def __str__(self) -> str:
        return (
            f"<{self.__class__.__name__}: source:`{self._source.name_pretty}` "
            f"annotation_count:{len(self._annotations):04}>"
        )

    def add_annotation(self, annotation: Annotation) -> None:
        self._annotations.append(annotation)

    @property
    def date_last_opened(self) -> datetime.datetime:
        return self._date_last_opened

    @property
    def path_epub(self) -> pathlib.Path:
        return self._path_epub

    @property
    def source(self) -> Source:
        return self._source

    @property
    def annotations(self) -> List[Annotation]:
        return sorted(self._annotations, key=lambda a: a.location)

    @property
    def name(self) -> str:
        # Title by Author XXXXXX
        return f"{self.source.name_pretty} {self.source.id[:6]}"

    @property
    def name_safe(self) -> str:
        # title-by-author-xxxxxx
        return helpers.misc.slugify(string=self.name)

    @property
    def name_safe_pretty(self) -> str:
        # Title by Author XXXXXX
        return helpers.misc.slugify(string=self.name, delimiter=" ", lowercase=False)

    @property
    def path_item_data(self) -> pathlib.Path:
        # /[user-stor]/data/items/[title-by-author-xxxxxx]
        return config.user.path_items_data / self.name_safe

    @property
    def file_data(self) -> pathlib.Path:
        # /[user-stor]/data/items/[title-by-author-xxxxxx]/data.json
        return self.path_item_data / config.app.FILENAME_DATA

    @property
    def path_media(self) -> pathlib.Path:
        # /[user-stor]/data/items/[title-by-author-xxxxxx]/media
        return self.path_item_data / config.app.DIRECTORY_NAME_MEDIA

    @property
    def file_export_text(self) -> pathlib.Path:
        # /[user-stor]/exports/text/[Title by Author XXXXXX.txt]
        return config.user.path_exports_text / f"{self.name_safe_pretty}.txt"

    @property
    def file_export_markdown(self) -> pathlib.Path:
        # /[user-stor]/exports/markdown/[Title by Author XXXXXX.md]
        return config.user.path_exports_markdown / f"{self.name_safe_pretty}.md"

    def serialize_source(self) -> dict:
        return self.source.serialize()

    def serialize_annotations(self) -> List[dict]:
        return [a.serialize() for a in self.annotations]

    def serialize(self) -> dict:
        return {
            StorItemKeys.DATE_LAST_OPENED: self._date_last_opened.isoformat(),
            StorItemKeys.SOURCE: self.serialize_source(),
            StorItemKeys.ANNOTATIONS: self.serialize_annotations(),
        }
