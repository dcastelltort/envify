extern crate serde_json;

use serde_json::{Error, Value};

#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    //#[structopt(long = "count", short = "n", default_value = "3")]
    //count: usize,
    // Add a positional argument that the user has to supply:
    /// The file to read
    file: String,
    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(&args.file)?;

    // Parse the string of data into serde_json::Value.
    let json_file: Value = serde_json::from_str(&content)?;

    let mut acc = vec![];
    let prefix = "";
    iter(&mut acc, prefix, &json_file);
    println!("{:#?}", acc);
});

fn iter<'a>(acc: &'a mut Vec<String>, current_prefix: &str, json: &Value) -> &'a mut Vec<String> {
    match json.as_object() {
        Some(m) => {
            m.iter().for_each(|(key, val)| {
                let separator = if current_prefix.is_empty() { "" } else { "_" };
                if val.is_object() {
                    iter(
                        acc,
                        format!("{}{}{}", current_prefix, separator, key).as_str(),
                        val,
                    );
                } else {
                    let target_salt = 1;

                    let new_decl = if target_salt == 0 {
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
                    acc.push(new_decl);
                }
            });
            return acc;
        }
        None => {
            return acc;
        }
    }
}
