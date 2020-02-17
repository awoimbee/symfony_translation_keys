# symfony_translation_keys
### What is it
A program that parses translation files (only yaml format as of now) and searches for occurences in the whole code base, returning unused keys each with a 'trust' factor.
- trust 0: you need to investigate, nothing is certain
- trust 1: you still need to investigate, but YOLOing is possible
- trust 2 & more: just remove that key

```
translations checker 0.2
Arthur W. <arthur.woimbee@gmail.com>
Find unused translations in symfony project

USAGE:
    symfony_translations_checker [OPTIONS] --project_root <FOLDER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --project_root <FOLDER>        Where to work
    -s, --src <FILE|FOLDER>...         where to search for translation keys usage (rel. to p. root)
    -t, --trans_fd <FILE|FOLDER>...    Where to load translation keys (rel. to p. root)
```
