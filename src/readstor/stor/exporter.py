import jinja2

from readstor.config import config

from . import models


class Exporter:

    ENVIRONMENT = jinja2.Environment(
        loader=jinja2.FileSystemLoader(config.app.PATH_TEMPLATES),
        trim_blocks=True,
        lstrip_blocks=True,
    )

    TEMPLATE_TEXT = ENVIRONMENT.get_template("export.txt.jinja")
    TEMPLATE_MARKDOWN = ENVIRONMENT.get_template("export.md.jinja")

    def export(self, item: models.StorItem) -> None:

        if config.user.export_markdown:
            self._export_markdown(item=item)

        if config.user.export_text:
            self._export_text(item=item)

    def _export_markdown(self, item: models.StorItem) -> None:

        render: str = self.TEMPLATE_MARKDOWN.render(
            date_exported=config.TODAY_PRETTY,
            source=item.source,
            annotations=item.annotations,
        )

        with open(item.file_export_markdown, "w") as f:
            f.write(render)

    def _export_text(self, item: models.StorItem) -> None:

        render: str = self.TEMPLATE_TEXT.render(
            date_exported=config.TODAY_PRETTY,
            source=item.source,
            annotations=item.annotations,
        )

        with open(item.file_export_text, "w") as f:
            f.write(render)
