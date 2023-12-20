use std::ffi::OsString;
use std::fmt;
use tree_sitter::Tree;

pub mod typescript;

#[derive(Debug, Clone)]
pub struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error has occurred while parsing the source file.")
    }
}

#[derive(Debug)]
pub struct ParsedFile {
    tree: Option<Tree>,
    source_code: String,
    source_path: OsString,
}

impl ParsedFile {
    pub fn get_parse_tree(&self) -> &Option<Tree> {
        &self.tree
    }

    pub fn get_source_code(&self) -> &String {
        &self.source_code
    }

    pub fn get_source_path(&self) -> &OsString {
        &self.source_path
    }
}

pub struct ParsedDirectory {
    directory: OsString,
    parsed_files: Vec<ParsedFile>,
}

impl ParsedDirectory {
    pub fn get_directory(&self) -> &OsString {
        &self.directory
    }
    pub fn get_parsed_files(&self) -> &Vec<ParsedFile> {
        &self.parsed_files
    }
}