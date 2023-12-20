use std::ffi::{OsStr, OsString};
use std::fs;
use tree_sitter::Tree;

pub fn walk_tree(tree: Tree, source: &str) {
    let node = tree.root_node();
    let mut cursor = tree.walk();
    let mut node_stack = vec![node];
    while !node_stack.is_empty() {
        let current_node = node_stack.pop().unwrap();
        for child_node in current_node.children(&mut cursor) {
            node_stack.push(child_node);
        }

        if current_node.is_named() {
            println!("{:?}", current_node.utf8_text(source.as_bytes()));
            println!("{:?}", current_node.to_sexp());
        } else {
            println!("{:?}", current_node.to_sexp());
        }
    }
}
pub fn find_by_extensions_in_dir(dir: &OsStr, extensions: &Vec<&OsStr>) -> Vec<OsString> {
    let mut paths: Vec<OsString> = vec![];
    match fs::metadata(dir) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let entries = fs::read_dir(dir).unwrap();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            if let Ok(entry_metadata) = fs::metadata(&entry.path()) {
                                if entry_metadata.is_dir() && entry.path().file_name().unwrap() != "node_modules" {
                                    let next_depth = find_by_extensions_in_dir(entry.path().as_os_str(), &extensions);
                                    for path in next_depth {
                                        paths.push(path)
                                    }
                                } else if let Some(ext) = entry.path().extension() {
                                    if extensions.contains(&ext) {
                                        paths.push(entry.path().as_os_str().to_owned())
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    paths
}