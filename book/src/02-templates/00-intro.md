# Templates

ReadStor's most powerful feature is its templating interface. Templates can be
designed to output markdown, html, CSV or any kind of text-based file format.
(PDFs are possible but require an extra step to convert HTML > PDF.)

Templates consist of two main sections, the configuration block written in YAML
and the template body written in [Tera](https://tera.netlify.app/), a
Jinja-flavored templating language. The configuration describes how the template
will be rendered primarily the output structure and the names of the ouput files
and directories while the template body describes what will be rendered.
