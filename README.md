# Qas

![version number](https://img.shields.io/badge/version-0.1.0-blue.svg)
[![Actions Status](https://github.com/easbarba/qas_rs/workflows/ci/badge.svg)](https://github.com/easbarba/qas/actions)
[<img src="https://img.shields.io/crates/v/qas.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/qas)

Easily manage multiple FLOSS repositories

# Installation

TODO

## Usage

`qas` consumes configuration in the following manners:

By default it looks for configuration files at `$XDG_CONFIG/qas` or in the
directory set in the `$QAS_CONFIG_HOME` environment variable.

```shell
qas --grab
```

Of course, a `JSON` configuration file can provide projects;

```shell
qas --grab --json ~/Downloads/misc.json
```

or it consumes even a REST API `JSON` resource providing all the projects.

```shell
qas --grab --api localhost:5000/configs
```

PS: an API example is at: https://github.com/easbarba/qas_api.

## Configuration file

A `qas` single configuration file:

```json
[
  {
    "name": "awesomewm",
    "branch": "master",
    "url": "https://github.com/awesomeWM/awesome"
  },
  {
    "name": "nuxt",
    "branch": "main",
    "url": "https://github.com/nuxt/framework"
  }
]
```

More examples of configuration files are at `docs/examples`.

## Options

Consult `qas --help` for more options.

## GNU Guix

In a system with GNU Guix binary installed, its even easier to grab all
dependencies: `guix shell`.

## TODO

Check the `TODO.md` for more information.

## LICENSE

[GPL-v3](https://www.gnu.org/licenses/gpl-3.0.en.html)
