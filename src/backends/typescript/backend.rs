use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor, Tree};
use crate::utils::find_by_extensions_in_dir;

pub struct TypeScriptBackend {
    parser: Parser,
    parsed: Option<Box<HashMap<OsString, Tree>>>,
}

impl TypeScriptBackend {
    pub fn new() -> TypeScriptBackend {
        let mut backend = TypeScriptBackend {
            parser: Parser::new(),
            parsed: None,
        };
        backend.parser.set_language(tree_sitter_typescript::language_typescript()).expect("Error loading TypeScript grammar");
        backend
    }
    pub fn parse_directory(&mut self, directory: &OsStr) -> &Option<Box<HashMap<OsString, Tree>>> {
        let mut parser = &mut self.parser;
        let paths = find_by_extensions_in_dir(directory, &vec![OsStr::new("ts")]);
        let mut map: HashMap<OsString, Tree> = HashMap::new();
        for path in paths {
            match fs::read_to_string(&path) {
                Ok(source_code) => {
                    let tree = parser.parse(&source_code, None).unwrap();
                    map.insert(path, tree);
                },
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        self.parsed = Some(Box::new(map));
        &self.parsed
    }

    pub fn find_function_calls_in_tree(tree: &Tree, source: &str, function_name: &str) {
        let query = Query::new(tree_sitter_typescript::language_typescript(),
                               &format!(r#"
                ((call_expression
                  function: [
                    (identifier) @function
                      (member_expression
                        property: (property_identifier) @method
                      )]
                 (#eq? @function "{}")
                 (#eq? @method "{}")
                ))"#, function_name, function_name)).unwrap();
        let mut query_cursor = QueryCursor::new();
        let all_matches = query_cursor.matches(&query, tree.root_node(), source.as_bytes());
        for each_match in all_matches {
            for capture in each_match.captures {
                let range = capture.node.range();
                let text = &source[range.start_byte..range.end_byte];
                let line = range.start_point.row;
                let col = range.start_point.column;
                println!("[Line: {}, Col: {}] Found: `{}`", line, col, text);
            }
        }
    }

    pub fn find_imports_in_tree(tree: &Tree, source: &str) {
        let query = Query::new(tree_sitter_typescript::language_typescript(),
                               r#"
                           (import_statement
                           "import"
                           (import_clause ((identifier)? @named_import
                           			(namespace_import (identifier) @namespace_import)?
                           			            (named_imports ((((import_specifier [(identifier) (string)]+ @named_import ("as" ([(identifier) (string)] @alias))?)) @import_with_alias) ","?)*)?))
                           (string) @source
                           ) @import"#).unwrap();
        let named_import_idx = query.capture_index_for_name("named_import").unwrap();
        let namespace_import_idx = query.capture_index_for_name("namespace_import").unwrap();
        let source_idx = query.capture_index_for_name("source").unwrap();
        let import_idx = query.capture_index_for_name("import").unwrap();
        let alias_idx = query.capture_index_for_name("alias").unwrap();
        let import_with_alias_idx = query.capture_index_for_name("import_with_alias").unwrap();
        let mut query_cursor = QueryCursor::new();
        let all_matches = query_cursor.matches(&query, tree.root_node(), source.as_bytes());
        for each_match in all_matches {
            for capture in each_match.captures {
                let range = capture.node.range();
                let text = &source[range.start_byte..range.end_byte];
                let line = range.start_point.row;
                let col = range.start_point.column;
                let capture_name = match capture.index {
                    idx if idx == named_import_idx => "named_import",
                    idx if idx == namespace_import_idx => "namespace_import",
                    idx if idx == source_idx => "source",
                    idx if idx == import_idx => "import",
                    idx if idx == alias_idx => "alias",
                    idx if idx == import_with_alias_idx => "import_with_alias",
                    _ => "Unknown",
                };
                println!("[Line: {}, Col: {}, Match: {}, Name: {}] Found: `{}`", line, col, each_match.id(), capture_name, text);
            }
        }
    }
}