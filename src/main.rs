extern crate serde_json;

use serde_json::{Error, Value};

#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The file to read
    file: String,
    #[structopt(short = "b", long = "bash", help = "Generate bash script (default)")]
    /// generate bash
    bash: bool,
    #[structopt(short = "s", long = "salt", help = "Generate salt script")]
    /// generate salt
    salt: bool,
    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(&args.file)?;

    // Parse the string of data into serde_json::Value.
    let json_file: Value = serde_json::from_str(&content)?;

    let mut decls = vec![];
    let prefix = "";
    let target : u32 = if args.salt { 1 } else { 0 }; // change to enum

    generate_decls(&mut decls, prefix, &json_file, target);
    println!("{:#?}", decls);
});

fn generate_decls<'a>(declarations: &'a mut Vec<String>, current_prefix: &str, json: &Value, target: u32) -> &'a mut Vec<String> {
    match json.as_object() {
        Some(m) => {
            m.iter().for_each(|(key, val)| {
                let separator = if current_prefix.is_empty() { "" } else { "_" };
                if val.is_object() {
                    generate_decls(
                        declarations,
                        format!("{}{}{}", current_prefix, separator, key).as_str(),
                        val,
                        target
                    );
                } else {

                    let new_decl = if target == 0 {
                        format!(
                            "export {}{}{}={}",
                            current_prefix.to_uppercase(),
                            separator,
                            key.to_uppercase(),
                            val
                        )
                    } else {
                        format!(
                            "salt '*' environ.setval {}{}{} {}",
                            current_prefix.to_uppercase(),
                            separator,
                            key.to_uppercase(),
                            val
                        )
                    };
                    declarations.push(new_decl);
                }
            });
            return declarations;
        }
        None => {
            return declarations;
        }
    }
}
