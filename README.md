# Jisort

A **J**avaScript / Typescript **i**mport **sort**er.  
Originally built specifically for [JVS](https://www.groupe-jvs.fr/).

# Usage

## Basic usage

To use, just run `jisort` at the root of your project.

## Options

You can use the `--globs` option to override the default ones.
The default globs match every `js`, `jsx`, `ts` or `tsx` file.

## Included files

jisort will include:

- Hidden files
- Files matching the given globs (except if they are in the following category)

jisort will **not** include:

- Files listed in `.gitignore` (see full list [here](https://docs.rs/ignore/0.4.20/ignore/struct.WalkBuilder.html))
- Files under hidden dirs

**NOTE:** List included files with `jisort --list`.

## Help

See `jisort --help` for all options and documentation.

# Installation

## From source

```sh
cargo install --git https://github.com/Valentin271/jisort
```

## Pre-compiled binaries

You can download the appropriate binary for your platform [on the release page](https://github.com/Valentin271/jisort/releases/latest).
