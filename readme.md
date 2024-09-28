# Cargo-tree-mermaid

## Introduction

This is a small POC implementation to generate a Mermaid Tree on the Cargo file. 

The implementation is based on the project https://github.com/sfackler/cargo-tree

Feel free to copy, reuse or build it out, I needed only a short output on the console for my case.

## Sample

```mermaid
flowchart TD
  cargo-tree-mermaid --> structopt
  cargo-tree-mermaid --> serde_json
  cargo-tree-mermaid --> semver
  cargo-tree-mermaid --> petgraph
  cargo-tree-mermaid --> cargo_metadata
  cargo-tree-mermaid --> anyhow
  cargo_metadata --> thiserror
  cargo_metadata --> serde_json
  cargo_metadata --> serde
  cargo_metadata --> semver
  cargo_metadata --> cargo-platform
  cargo_metadata --> camino
  camino --> serde
  serde --> serde_derive
  serde_derive --> syn
  serde_derive --> quote
  serde_derive --> proc-macro2
  proc-macro2 --> unicode-ident
  quote --> proc-macro2
  syn --> unicode-ident
  syn --> quote
  syn --> proc-macro2
  cargo-platform --> serde
  semver --> serde
  serde_json --> serde
  serde_json --> ryu
  serde_json --> memchr
  serde_json --> itoa
  thiserror --> thiserror-impl
  thiserror-impl --> syn
  thiserror-impl --> quote
  thiserror-impl --> proc-macro2
  petgraph --> indexmap
  petgraph --> fixedbitset
  indexmap --> hashbrown
  indexmap --> equivalent
  structopt --> structopt-derive
  structopt --> lazy_static
  structopt --> clap
  clap --> vec_map
  clap --> unicode-width
  clap --> textwrap
  clap --> strsim
  clap --> bitflags
  clap --> atty
  clap --> ansi_term
  atty --> libc
  textwrap --> unicode-width
  structopt-derive --> syn
  structopt-derive --> quote
  structopt-derive --> proc-macro2
  structopt-derive --> proc-macro-error
  structopt-derive --> heck
  heck --> unicode-segmentation
  proc-macro-error --> syn
  proc-macro-error --> quote
  proc-macro-error --> proc-macro2
  proc-macro-error --> proc-macro-error-attr
  proc-macro-error-attr --> quote
  proc-macro-error-attr --> proc-macro2
  syn --> unicode-ident
  syn --> quote
  syn --> proc-macro2

```