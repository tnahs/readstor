# Options

Each of the three currently available [Commands][commands] has its own pipeline for processing Apple
Books' data before writing it to disk. And by extension, each pipeline has its own set of applicable
options.

The pipelines and options for these three [Commands][commands] are as follows:

```plaintext
╭─────────╮           Export Pipeline
│ Global* │          ╭────────╮ ╭─────────────╮ ╔════════╗
╰────┬────╯        ┌─┤ Filter ├─┤ Pre-process ├─╢ export ╟────────────────────┐
     │             │ ╰────────╯ ╰─────────────╯ ╚════════╝                    │
     │             │                                                          │
     │             │                                                          │
     │             │  Render Pipeline                                         │
     │             │ ╭───────────╮                                            │
     │             │ │ Templates ├──────────────┐                             │
     │             │ ╰───────────╯              │                             │
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

| Name                         | Affects Commands  | Options For                        |
| ---------------------------- | ----------------- | ---------------------------------- |
| [Global][global]             | All               | -                                  |
| [Render][render]             | `render`          | Configuring renders.               |
| [Export][export]             | `export`          | Configuring exports.               |
| [Backup][backup]             | `backup`          | Configuring backups.               |
| [Filter][filter]             | `render` `export` | Filtering down books/annotations.  |
| [Pre-process][pre-process]   | `render` `export` | Processing before running Command. |
| [Post-process][post-process] | `render`          | Processing after running Command.  |

[backup]: /intro/options/backup.md
[commands]: /intro/commands.md
[export]: /intro/options/export.md
[filter]: /intro/options/filter.md
[global]: /intro/options/global.md
[post-process]: /intro/options/postprocess.md
[pre-process]: /intro/options/preprocess.md
[render]: /intro/options/render.md
