//! V language grammar for [tree-sitter](https://tree-sitter.github.io/tree-sitter/).
///
/// Maintained fork of [nedpals/tree-sitter-v](https://github.com/nedpals/tree-sitter-v).
use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_v() -> *const ();
}

/// Tree-sitter language for V.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_v) };

/// Static node types for this grammar.
pub const NODE_TYPES: &str = include_str!("src/node-types.json");

/// Highlight queries for V syntax.
pub const HIGHLIGHTS_QUERY: &str = include_str!("highlights.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading V language");
    }

    #[test]
    fn test_highlights_query_compiles() {
        tree_sitter::Query::new(&super::LANGUAGE.into(), super::HIGHLIGHTS_QUERY)
            .expect("Error loading V highlight query");
    }

    #[test]
    fn highlights_literals_and_builtin_types() {
        use tree_sitter::StreamingIterator;

        let source =
            "fn f(x int) { a := [1, 2]\n n := 1.5\n println(r'raw')\n println(c'hello')\n }";
        let language = super::LANGUAGE.into();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        let tree = parser.parse(source, None).unwrap();
        assert!(!tree.root_node().has_error());
        let query = tree_sitter::Query::new(&language, super::HIGHLIGHTS_QUERY).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());
        let mut seen = Vec::new();
        while let Some((matched, index)) = captures.next() {
            let capture = matched.captures[*index];
            seen.push((
                query.capture_names()[capture.index as usize],
                capture.node.utf8_text(source.as_bytes()).unwrap(),
            ));
        }
        for expected in [
            ("type", "int"),
            ("number", "1.5"),
            ("string", "r'raw'"),
            ("string", "c'hello'"),
            ("punctuation.bracket", "["),
        ] {
            assert!(seen.contains(&expected), "missing {expected:?} in {seen:?}");
        }
        assert!(!seen.contains(&("punctuation.bracket", "[1, 2]")));
    }

    #[test]
    fn parses_string_interpolation() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&super::LANGUAGE.into()).unwrap();
        let tree = parser
            .parse("fn main() { println(\"Hello ${name}!\") }", None)
            .unwrap();
        let root = tree.root_node();
        assert!(!root.has_error(), "{}", root.to_sexp());
        assert!(root.to_sexp().contains("(string_interpolation"));
    }
}
