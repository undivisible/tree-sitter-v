# tree-sitter-v

[![crates.io](https://img.shields.io/crates/v/tree-sitter-v.svg)](https://crates.io/crates/tree-sitter-v)
[![GitHub](https://img.shields.io/github/license/undivisible/tree-sitter-v)](LICENSE)

V language grammar for [tree-sitter](https://tree-sitter.github.io/tree-sitter/).

Generated parser — 244 node types, C99, Rust bindings.

## Usage

```rust
use tree_sitter_v::LANGUAGE;

let mut parser = tree_sitter::Parser::new();
parser.set_language(&LANGUAGE.into())?;

let tree = parser.parse("fn main() { println(\"hello\") }", None)?;
```

## Install

```bash
cargo add tree-sitter-v
```

## Node Types

244 node types covering V's syntax: functions, structs, generics, interfaces,
enums, arrays, slices, maps, closures, or-patterns, sum types, comptime blocks,
and more.

## License

MIT
