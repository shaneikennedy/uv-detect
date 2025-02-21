# uv-detect

Detect external package dependencies in your code and add the missing ones to pyproject.toml

A rust port of the excellent package [pipreqs](https://github.com/bndr/pipreqs), but for uv

## Usage

`uv-detect` in the root of a uv-managed project

## Demo

```sh
uv-detect git:main
❯ pwd
/Users/shane.kennedy/dev/shane/uv-detect

uv-detect git:main
❯ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s

uv-detect git:main
❯ target/debug/uv-detect --help
Usage: uv-detect [OPTIONS]

Options:
      --exclude-dirs <EXCLUDE_DIRS>  List of directories to ignore, we ignore .venv and .git by default
  -h, --help                         Print help
  -V, --version                      Print version

uv-detect git:main
❯ cd example_app

uv-detect/example_app git:main
❯ cat pyproject.toml
[project]
name = "example"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.13"
dependencies = []

uv-detect/example_app git:main
❯ ../target/debug/uv-detect
INFO  [uv_detect::writer] Adding: Django~=5.1.6
INFO  [uv_detect::writer] Adding: djangorestframework~=3.15.2
INFO  [uv_detect::writer] Writting dependencies to pyproject.toml
INFO  [uv_detect::writer] Updated pyproject.toml
INFO  [uv_detect::writer] Syncing uv
INFO  [uv_detect::writer] Writting uv lock

uv-detect/example_app git:main
❯ cat pyproject.toml
[project]
name = "example"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.13"
dependencies = [
    "Django~=5.1.6",
    "djangorestframework~=3.15.2"
]
```

### ToDos

- [x] Make it configurable via cli flags
- [x] Resolve against package registries for proper version speccing
- [x] Don't overwrite current contents
- [x] Real logging
- [ ] Add a dotfile for saving configurattion
- [ ] Do better than panicing everywhere
- [ ] Make it fast
