use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

use crate::schema_parser::Schema;

mod schema_parser;
mod object_definitions;
mod field_definitions;
mod field_kinds;
mod random_values;


fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        process::exit(1);
    }

    let schema_root = args.nth(1).unwrap();
    let dir = fs::read_dir(schema_root).unwrap();

    let mut reference_map = HashMap::new();

    for entry in dir {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() && entry.path().extension().unwrap() == "json" {
            let schema = Schema::from_file(&entry.path()).unwrap();
            let fullpath: String = entry.path().to_string_lossy().to_string();
            reference_map.insert(fullpath, schema);
        }
    }

    eprintln!("reference_map = {:?}", reference_map);
}
