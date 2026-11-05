# tree-sitter-v

[![crates.io](https://img.shields.io/crates/v/tree-sitter-v.svg)](https://crates.io/crates/tree-sitter-v)
[![GitHub](https://img.shields.io/github/license/undivisible/tree-sitter-v)](LICENSE)

Tree-sitter grammar for [V language](https://vlang.io).

**Maintained fork** of [nedpals/tree-sitter-v](https://github.com/nedpals/tree-sitter-v).
The original is unmaintained since 2023 — this fork keeps it up to date with modern
tree-sitter releases and fixes.

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

## Features

- V syntax nodes for declarations, expressions, and types
- Functions, structs, generics, interfaces, enums
- Arrays, slices, maps, closures
- Comptime blocks and selectors
- Match expressions, sum types, or-patterns
- String interpolation with format specifiers
- SQL expressions, C/JS interop
- VSH mode
- Syntax highlighting queries

## Compared to upstream

| | nedpals/tree-sitter-v | This fork |
|---|---|---|
| tree-sitter API | 0.17 (legacy) | 0.26+ (`tree-sitter-language`) |
| Rust bindings | `Language` enum | `LanguageFn` |
| Published on crates.io | No | Yes |
| Maintenance | Last updated 2023 | Active |

## License

ISC and MIT — see [LICENSE](LICENSE) and [LICENSE.upstream](LICENSE.upstream).

Original grammar by [Ned Palacios](https://github.com/nedpals) and contributors.
The upstream grammar and scanner are MIT licensed; see `LICENSE.upstream`.

## Development

Use Tree-sitter CLI 0.25.10 to regenerate `src/parser.c` and the node types with
`tree-sitter generate`, then run `tree-sitter test` and `cargo test`. The scanner
is compiled alongside the generated parser by the Rust binding.
