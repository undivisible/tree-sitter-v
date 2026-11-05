# Changelog

## 0.0.5

- Fix the defective 0.0.4 release: the imported V grammar and corpus were shipped with an older, mismatched parser and without the external scanner needed to regenerate the grammar. Restore the upstream scanner, regenerate the parser, and compile both into the Rust binding. The restored parser passes the existing corpus without changing its expected trees.
- Restore highlighting for built-in types and add captures for floating-point numbers, raw strings, and C strings. Array contents are no longer captured as bracket punctuation.
- The regenerated parser changes node types and tree shapes and uses Tree-sitter language ABI 15. Consumers should check their queries and ensure their Tree-sitter runtime supports ABI 15 before upgrading.
