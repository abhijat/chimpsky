use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Result;

use structopt::StructOpt;

use crate::object_definitions::ObjectDefinition;
use crate::schema_parser::Schema;

mod schema_parser;
mod object_definitions;
mod field_definitions;
mod field_kinds;
mod random_values;

#[derive(Debug, StructOpt)]
enum Command {
    /// Print object definitions found and exit
    Report,

    /// Generate random JSON payloads based on supplied object name
    Randomize {
        /// Object name to emit random JSON payloads for
        #[structopt(short, long)]
        object_name: String,

        /// Number of random payloads to emit
        #[structopt(short, long, default_value = "100")]
        emit_count: u64,

        /// Prettify emitted JSON
        #[structopt(short, long)]
        prettify: bool,
    },
}

#[derive(Debug, StructOpt)]
struct Options {
    /// Root path for json schema files
    #[structopt(short, long)]
    schema_dir: String,

    /// Action to perform
    #[structopt(subcommand)]
    command: Command,

    /// Exit after showing analyzed object definitions
    #[structopt(short, long)]
    report_and_exit: bool,
}


fn schema_from_entry(entry: Result<DirEntry>) -> Option<Schema> {
    let entry = entry.ok()?;
    let path = entry.path();

    if entry.file_type().ok()?.is_file() && path.extension()? == "json" {
        Schema::from_file(&path)
    } else {
        None
    }
}

fn report_schemas(reference_map: &HashMap<String, ObjectDefinition>) {
    reference_map.iter()
        .for_each(|(key, object_definition)| {
            println!("{} in file {}", object_definition.name, key);
        });
}

fn randomize_payloads(reference_map: &HashMap<String, ObjectDefinition>, command: &Command) {
    if let Command::Randomize { object_name, emit_count, prettify } = command {
        let definition = &reference_map[object_name];
        for _ in 0..*emit_count {
            let payload = definition.generate_json(Some(&reference_map)).unwrap();
            let s = if *prettify {
                serde_json::to_string_pretty(&payload).unwrap()
            } else {
                serde_json::to_string(&payload).unwrap()
            };
            println!("{}", s);
        }
    }
}


fn main() {
    let options: Options = Options::from_args();

    let schema_root = &options.schema_dir;
    let dir = fs::read_dir(schema_root).unwrap();

    let mut reference_map: HashMap<String, ObjectDefinition> = HashMap::new();

    for entry in dir {
        schema_from_entry(entry).map(|schema| reference_map.extend(schema.export_definitions().unwrap()));
    }

    match &options.command {
        Command::Report =>
            report_schemas(&reference_map),
        Command::Randomize { .. } =>
            randomize_payloads(&reference_map, &options.command),
    }
}
