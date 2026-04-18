//! Ferriprove Export
//!
//! Lean 4 export NDJSON parser.
//!
//! This crate parses the Lean 4 export format (version 3.1.0) as specified in:
//! <https://github.com/leanprover/lean4export/blob/master/format_ndjson.md>
//!
//! The export format is a newline-delimited JSON (NDJSON) format where each line
//! represents either:
//! - A name component (string or numeric)
//! - A universe level
//! - An expression
//! - A declaration (axiom, definition, theorem, opaque, quot, inductive)
//!
//! All references between entities use integer IDs, which are resolved by this parser.

pub mod ast;
pub mod error;
pub mod parser;
pub mod resolver;

pub use ast::*;
pub use error::{ParseError, Result};
pub use parser::{ExportLine, MetaInfo, ParserState};
pub use resolver::{
    DeclarationResolver, DefinitionHints, ExprResolver, LevelResolver, NameResolver, QuotKind,
    ResolvedConstructor, ResolvedDeclaration, ResolvedInductiveType, ResolvedRecursor, Safety,
};

use std::path::Path;

/// Parse an NDJSON export file and return the parser state with all entities.
///
/// This function parses the entire file, storing names, levels, expressions, and
/// declarations with their integer references. To resolve these to ferriprove-types
/// types, use the resolver module.
///
/// # Arguments
///
/// * `path` - Path to the NDJSON export file
///
/// # Returns
///
/// Returns a `ParserState` containing all parsed entities, or a `ParseError` if
/// parsing fails.
///
/// # Example
///
/// ```rust,no_run
/// use ferriprove_export::parse_file;
///
/// let state = parse_file("mathlib.ndjson").unwrap();
/// println!("Parsed {} names, {} levels, {} expressions, {} declarations",
///     state.names.len(), state.levels.len(), state.expressions.len(), state.declarations.len());
/// ```
pub fn parse_file(path: impl AsRef<Path>) -> Result<ParserState> {
    ParserState::parse_file(path)
}

/// Parse an NDJSON export file and return fully resolved declarations.
///
/// This is a convenience function that parses the file and immediately resolves
/// all declarations to ferriprove-types types.
///
/// # Arguments
///
/// * `path` - Path to the NDJSON export file
///
/// # Returns
///
/// Returns a vector of `ResolvedDeclaration`, or a `ParseError` if parsing fails.
///
/// # Example
///
/// ```rust,no_run
/// use ferriprove_export::parse_and_resolve;
///
/// let declarations = parse_and_resolve("mathlib.ndjson").unwrap();
/// for decl in declarations {
///     println!("{:?}", decl);
/// }
/// ```
pub fn parse_and_resolve(path: impl AsRef<Path>) -> Result<Vec<ResolvedDeclaration>> {
    let state = parse_file(path)?;
    let mut resolver = DeclarationResolver::new(&state);
    resolver.resolve_all(&state)
}

/// Get the version of the export format supported by this parser.
pub const EXPORT_FORMAT_VERSION: &str = "3.1.0";

/// Check if a format version is supported by this parser.
pub fn is_version_supported(version: &str) -> bool {
    // For now, we only support 3.1.0
    version == EXPORT_FORMAT_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_export() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        let content = r#"{"meta":{"exporter":{"name":"lean4export","version":"0.1.0"},"lean":{"githash":"abc123","version":"4.8.0"},"format":{"version":"3.1.0"}}}
{"str":{"pre":0,"str":"Nat"},"in":1}
{"succ":0,"il":1}
{"bvar":0,"ie":1}
{"sort":1,"ie":2}
{"axiom":{"name":1,"levelParams":[],"type":2,"isUnsafe":false}}"#;
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_parse_file() {
        let file = create_test_export();
        let state = parse_file(file.path()).unwrap();

        assert!(state.meta.is_some());
        assert_eq!(state.names.len(), 1);
        assert_eq!(state.levels.len(), 1);
        assert_eq!(state.expressions.len(), 2);
        assert_eq!(state.declarations.len(), 1);
    }

    #[test]
    fn test_meta_info() {
        let file = create_test_export();
        let state = parse_file(file.path()).unwrap();

        let meta = state.meta.unwrap();
        assert_eq!(meta.format.version, "3.1.0");
        assert_eq!(meta.exporter.name, "lean4export");
    }

    #[test]
    fn test_version_support() {
        assert!(is_version_supported("3.1.0"));
        assert!(!is_version_supported("3.0.0"));
        assert!(!is_version_supported("4.0.0"));
    }

    #[test]
    fn test_parse_and_resolve() {
        let file = create_test_export();
        let declarations = parse_and_resolve(file.path()).unwrap();

        assert_eq!(declarations.len(), 1);
        match &declarations[0] {
            ResolvedDeclaration::Axiom {
                name, is_unsafe, ..
            } => {
                assert_eq!(name.as_str(), "Nat");
                assert!(!is_unsafe);
            }
            _ => panic!("Expected axiom declaration"),
        }
    }

    /// Test that App expressions parse correctly (regression test for bug #7)
    /// The JSON field is "fn" which is a Rust keyword, requiring serde rename
    #[test]
    fn test_app_expression_parsing() {
        let mut file = NamedTempFile::new().unwrap();
        let content = r#"{"meta":{"exporter":{"name":"lean4export","version":"0.1.0"},"lean":{"githash":"abc123","version":"4.8.0"},"format":{"version":"3.1.0"}}}
{"str":{"pre":0,"str":"Nat"},"in":1}
{"str":{"pre":1,"str":"add"},"in":2}
{"succ":0,"il":1}
{"sort":1,"ie":2}
{"const":{"name":2,"us":[]},"ie":3}
{"const":{"name":1,"us":[]},"ie":4}
{"app":{"fn":3,"arg":4},"ie":5}
{"axiom":{"name":1,"levelParams":[],"type":5,"isUnsafe":false}}"#;
        file.write_all(content.as_bytes()).unwrap();

        let state = parse_file(file.path()).unwrap();

        // Should have parsed: 2 names, 1 level, 4 expressions (sort, 2 consts, app)
        assert_eq!(state.names.len(), 2);
        assert_eq!(state.levels.len(), 1);
        assert_eq!(state.expressions.len(), 4); // sort, const Nat.add, const Nat, app

        // Verify the app expression was parsed correctly
        let app_expr = state
            .expressions
            .get(&5)
            .expect("App expression with ie=5 should exist");
        match app_expr {
            ParsedExpr::App { fun_id, arg_id } => {
                assert_eq!(*fun_id, 3); // Nat.add constant
                assert_eq!(*arg_id, 4); // Nat constant
            }
            _ => panic!("Expected App expression at ie=5, got {:?}", app_expr),
        }
    }

    /// Test that cycle detection works for name resolution (regression test for bug #2)
    #[test]
    fn test_name_cycle_detection() {
        let mut file = NamedTempFile::new().unwrap();
        // Create a cyclic name reference: name 1 points to pre=1 (itself)
        let content = r#"{"meta":{"exporter":{"name":"lean4export","version":"0.1.0"},"lean":{"githash":"abc123","version":"4.8.0"},"format":{"version":"3.1.0"}}}
{"str":{"pre":1,"str":"cyclic"},"in":1}
{"axiom":{"name":1,"levelParams":[],"type":1,"isUnsafe":false}}"#;
        file.write_all(content.as_bytes()).unwrap();

        let state = parse_file(file.path()).unwrap();
        let mut resolver = DeclarationResolver::new(&state);
        let result = resolver.resolve_all(&state);

        // Should detect the cycle and return an error
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("cycle"),
            "Expected cycle error, got: {}",
            err
        );
    }
}
