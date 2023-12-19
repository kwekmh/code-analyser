use std::ffi::OsStr;
use std::fs;
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
        Some(parsed) => {
            for (k, v) in parsed.iter() {
                println!("{:?} => {:?}", k, v);
                TypeScriptBackend::find_function_calls_in_tree(&v, &fs::read_to_string(k).unwrap(), "replaceMeHere");
                TypeScriptBackend::find_imports_in_tree(&v, &fs::read_to_string(k).unwrap());
            }
        },
        None => {
            println!("Nothing was parsed");
        }
    }
}