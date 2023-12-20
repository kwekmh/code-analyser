use std::ffi::OsStr;
use libcode_analyser::backends::typescript::backend::TypeScriptBackend;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    directory: String,
}

fn main() {
    let args = Args::parse();
    let mut typescript_backend = TypeScriptBackend::new();
    match typescript_backend.parse_directory(OsStr::new(&args.directory)) {
        Ok(parsed_directory) => {
            for parsed_file in parsed_directory.get_parsed_files() {
                println!("==================");
                println!("{:?}", parsed_file);
                if let Some(&ref tree) = parsed_file.get_parse_tree().as_ref() {
                    TypeScriptBackend::get_function_calls_in_tree(&tree, &parsed_file.get_source_code(), "replaceMeHere");
                    let imports = TypeScriptBackend::get_imports_in_tree(&tree, &parsed_file.get_source_code());
                    for import in &imports {
                        println!("Import: {:?}", import);
                    }
                }
            }
        },
        Err(e) => { println!("{}", e); }
    }
}