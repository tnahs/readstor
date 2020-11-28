import logging
import pathlib

import jinja2

from readstor import helpers
from readstor.config import GlobalConfig

from . import models


logger = logging.getLogger(__name__)


class Exporter:
    """Exporter Folder Structure

    /path/to/exports
    ├── flat
    │   ├── item-01.md
    │   ├── item-02.md
    │   └── ...
    └── nested
        ├── item-01
        │   ├── source.md
        │   ├── annotations
        │   │   ├── annotation-01.md
        │   │   ├── annotation-02.md
        │   │   └── ...
        ├── item-01
        │   ├── source.md
        │   └── annotations
        │       ├── annotation-01.md
        │       ├── annotation-02.md
        │       └── ...
        └── ...
    """

    DIRECTORY_NAME_FLAT: str = "flat"
    DIRECTORY_NAME_NESTED: str = "nested"
    DIRECTORY_NAME_ANNOTATIONS: str = "annotations"

    FILE_NAME_SOURCE: str = "source.md"

    ENVIRONMENT = jinja2.Environment(
        loader=jinja2.FileSystemLoader(GlobalConfig.app.PATH_TEMPLATES),
        # trim_blocks=True,
        # lstrip_blocks=True,
    )

    TEMPLATE_FLAT = ENVIRONMENT.get_template("flat.jinja")
    TEMPLATE_NESTED_SOURCE = ENVIRONMENT.get_template("nested-source.jinja")
    TEMPLATE_NESTED_ANNOTATION = ENVIRONMENT.get_template("nested-annotation.jinja")

    def __init__(self):

        helpers.shell.make(path=self.path_nested)
        helpers.shell.make(path=self.path_flat)

    @property
    def path_flat(self) -> pathlib.Path:
        # /[user-stor]/exports/flat
        return GlobalConfig.user.path_exports / self.DIRECTORY_NAME_FLAT

    @property
    def path_nested(self) -> pathlib.Path:
        # /[user-stor]/exports/nested
        return GlobalConfig.user.path_exports / self.DIRECTORY_NAME_NESTED

    def export(self, item: models.StorItem) -> None:

        if GlobalConfig.user.export_flat:
            self._export_flat(item=item)

        if GlobalConfig.user.export_nested:
            self._export_nested(item=item)

    def _export_flat(self, item: models.StorItem) -> None:

        render: str = self.TEMPLATE_FLAT.render(
            item=item,
            date_exported=GlobalConfig.TODAY_PRETTY,
        )

        # /[user-stor]/exports/flat/[Title-by-Author].md
        file_export = self.path_flat / f"{item.name}.md"

        with open(file_export, "w") as f:
            logger.debug(f"Writing {file_export}.")
            f.write(render)

    def _export_nested(self, item: models.StorItem) -> None:
        # TODO: Document and clean-up.

        # /[user-stor]/exports/nested/[Title-by-Author]
        path_export_root = self.path_nested / item.name

        # /[user-stor]/exports/nested/[Title-by-Author]/source.md
        file_source = path_export_root / self.FILE_NAME_SOURCE

        # /[user-stor]/exports/nested/[Title-by-Author]/annotations
        path_annotations = path_export_root / self.DIRECTORY_NAME_ANNOTATIONS

        """The exported directory is re-made in case the annotations or source
        have been modified. For example, when an annotation is inserted
        between two previous ones, the order of the annoations will have to
        be changed."""

        helpers.shell.remove(path=path_export_root)
        helpers.shell.make(path=path_export_root)
        helpers.shell.make(path=path_annotations)

        render_source: str = self.TEMPLATE_NESTED_SOURCE.render(
            item=item,
            date_exported=GlobalConfig.TODAY_PRETTY,
        )

        with open(file_source, "w") as f:
            logger.debug(f"Writing {file_source}.")
            f.write(render_source)

        for count, annotation in enumerate(item.annotations):

            file_annotation = path_annotations / f"{count:04}_{annotation.id}.md"

            render_annotation: str = self.TEMPLATE_NESTED_ANNOTATION.render(
                item=item,
                annotation=annotation,
                date_exported=GlobalConfig.TODAY_PRETTY,
            )

            with open(file_annotation, "w") as f:
                logger.debug(f"Writing {file_annotation}.")
                f.write(render_annotation)
