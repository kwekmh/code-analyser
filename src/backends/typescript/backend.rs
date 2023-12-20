use std::ffi::{OsStr, OsString};
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor, Tree};
use crate::backends::{ParsedDirectory, ParsedFile, ParserError};
use crate::utils::find_by_extensions_in_dir;

pub struct TypeScriptBackend {
    parser: Parser,
}

#[derive(Debug)]
pub enum TypeScriptImportType {
    Unknown,
    NamedImport,
    NamespaceImport,
}

#[derive(Debug)]
pub struct TypeScriptImport {
    import_source: String,
    import_type: TypeScriptImportType,
    import_name: String,
    import_alias: Option<String>
}

impl TypeScriptBackend {
    pub fn new() -> TypeScriptBackend {
        let mut backend = TypeScriptBackend {
            parser: Parser::new(),
        };
        backend.parser.set_language(tree_sitter_typescript::language_typescript()).expect("Error loading TypeScript grammar");
        backend
    }
    pub fn parse_directory(&mut self, directory: &OsStr) -> Result<ParsedDirectory, ParserError> {
        let parser = &mut self.parser;
        let paths = find_by_extensions_in_dir(directory, &vec![OsStr::new("ts")]);
        let mut parsed_files: Vec<ParsedFile> = vec![];
        for path in paths {
            match fs::read_to_string(&path) {
                Ok(source_code) => {
                    let tree = parser.parse(&source_code, None);
                    parsed_files.push(ParsedFile {
                        tree,
                        source_code,
                        source_path: path
                    });
                },
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        Ok(ParsedDirectory {
            directory: OsString::from(directory),
            parsed_files,
        })
    }

    pub fn get_function_calls_in_tree(tree: &Tree, source: &str, function_name: &str) {
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

    pub fn get_imports_in_tree(tree: &Tree, source: &str) -> Vec<TypeScriptImport> {
        let mut imports: Vec<TypeScriptImport> = vec![];
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
            let mut is_not_first_import = false;
            let mut imports_to_add: Vec<TypeScriptImport> = vec![];
            for capture in each_match.captures {
                let range = capture.node.range();
                let text = &source[range.start_byte..range.end_byte];
                match capture.index {
                    idx if idx == import_idx => {
                        let import = TypeScriptImport {
                            import_source: "".to_string(),
                            import_type: TypeScriptImportType::Unknown,
                            import_name: "".to_string(),
                            import_alias: None,
                        };
                        imports_to_add.push(import);
                    },
                    idx if idx == import_with_alias_idx => {
                        if is_not_first_import {
                            let import = TypeScriptImport {
                                import_source: "".to_string(),
                                import_type: TypeScriptImportType::Unknown,
                                import_name: "".to_string(),
                                import_alias: None,
                            };
                            imports_to_add.push(import);
                        } else {
                            is_not_first_import = true;
                        }
                    },
                    idx if idx == named_import_idx => {
                        if let Some(import) = imports_to_add.last_mut() {
                            import.import_name = String::from(text);
                            import.import_type = TypeScriptImportType::NamedImport;
                            is_not_first_import = true;
                        }
                    },
                    idx if idx == namespace_import_idx => {
                        if let Some(import) = imports_to_add.last_mut() {
                            import.import_name = String::from(text);
                            import.import_type = TypeScriptImportType::NamespaceImport;
                            is_not_first_import = true;
                        }
                    },
                    idx if idx == source_idx =>  {
                        for import in imports_to_add.iter_mut() {
                            import.import_source = String::from(text);
                        }
                    },
                    idx if idx == alias_idx => {
                        if let Some(import) = imports_to_add.last_mut() {
                            import.import_alias = Some(String::from(text));
                        }
                    },
                    _ => {},
                };
            }
            for import in imports_to_add {
                imports.push(import);
            }
        }
        imports
    }
}