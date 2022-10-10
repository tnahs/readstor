# Introduction

ReadStor is a simple CLI for exporting user-generated data from Apple Books. The
goal of this project is to facilitate data-migration from Apple Books to any
other platform. Currently Apple Books provides no simple way to do this.
Exporting is possible but not ideal and often times truncates long annotations.

Version `0.1.x` contained the core functionality: (1) save all annotations and
notes as JSON (2) render them via a custom (or the default) template using the
[Tera][tera] syntax or (3) backup the current Apple Books databases.

Note that this repository is a heavy work-in-progress and things are bound to
change.

[tera]: https://tera.netlify.app/
