

<h1 align="center">
  
  # Project TOML parser
  [![CI][ci0]][ci1] [![crates][cr0]][cr1] ![MIT][l0] ![RV][rv0] 
 
</h1>

[ci0]: https://github.com/StephanMalan/project_toml_parser/actions/workflows/build.yml/badge.svg
[ci1]: https://github.com/StephanMalan/project_toml_parser/actions/workflows/rust.yml
[cr0]: https://img.shields.io/badge/dynamic/json?color=success&label=crates.io&prefix=v&query=versions%5B0%5D.num&url=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fproject_toml_parser%2Fversions
[cr1]: https://crates.io/crates/project_toml_parser
[l0]: https://img.shields.io/badge/license-MIT-blue.svg
[rv0]: https://img.shields.io/badge/rustc-1.71%2B-lightgrey.svg

Project TOML parser is a small command-line utility that retrieves project name and version from TOML files.

Example output for this project:
```
project_toml_parser:0.1.0
```

Currently supports:
- Python (Poetry)
- Rust
