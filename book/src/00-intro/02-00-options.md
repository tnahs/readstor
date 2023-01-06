# Options

Each of the three currently available [Commands][commands] has its own pipeline
for processing Apple Books' data before writing it to disk. And by extension,
each pipeline has its own set of applicable options.

The pipelines and options for these three [Commands][commands] are as follows:

```plaintext
╭─────────╮           Export Pipeline
│ Global* │          ╭────────╮ ╭─────────────╮ ╔════════╗
╰────┬────╯        ┌─┤ Filter ├─┤ Pre-process ├─╢ export ╟────────────────────┐
     │             │ ╰────────╯ ╰─────────────╯ ╚════════╝                    │
     │             │                                                          │
     │             │                                                          │
     │             │  Render Pipeline                                         │
     │             │ ╭──────────╮                                             │
     │             │ │ Template ├───────────────┐                             │
     │             │ ╰──────────╯               │                             │
     │ ┌╌╌╌╌╌╌╌╌╌┐ │ ╭────────╮ ╭─────────────╮ │ ╔════════╗ ╭──────────────╮ │ ┌╌╌╌╌╌╌╌┐
     █─┤ Extract ├─┴─┤ Filter ├─┤ Pre-process ├─┴─╢ render ╟─┤ Post-process ├─┼─┤ Write │
     │ └╌╌╌╌╌╌╌╌╌┘   ╰────────╯ ╰─────────────╯   ╚════════╝ ╰──────────────╯ │ └╌╌╌╌╌╌╌┘
     │                                                                        │
     │                                                                        │
     │  Backup Pipeline                                                       │
     │ ╔════════╗                                                             │
     └─╢ backup ╟─────────────────────────────────────────────────────────────┘
       ╚════════╝
```

| Name         | Affects Commands  | Options For                        |
| ------------ | ----------------- | ---------------------------------- |
| Global       | All               | -                                  |
| Filter       | `render` `export` | Filtering down books/annotations.  |
| Template     | `render`          | Configuring templates.             |
| Pre-process  | `render` `export` | Processing before running Command. |
| Post-process | `render`          | Processing after running Command.  |

[commands]: ./01-commands.md
[filter]: ./02-02-filter.md
[global]: ./02-01-global.md
[template]: ./02-03-template.md
[pre-process]: ./04-01-preprocessor.md
[post-process]: ./05-01-postprocessor.md
