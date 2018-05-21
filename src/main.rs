extern crate serde_json;

use serde_json::{Value};

use std::io::prelude::*;
use std::fs::File;

#[macro_use]
extern crate quicli;
use quicli::prelude::*;

use std::path::{Path};

#[derive(Debug, StructOpt)]
enum Target {
    /// generate a shell file
    #[structopt(name = "shell")]
    Shell,
    /// generate a salt file
    #[structopt(name = "salt")]
    Salt,
}

#[derive(Debug, StructOpt)]
struct Cli {
    /// The file to read
    file: String,
    /// Output file name
    #[structopt(short = "o", long = "output", help = "name of the file with extension, otherwise it uses input with appropriate extension")]
    output_file: Option<String>,
    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
    /// Pass many times for more log output
    #[structopt(subcommand)]
    target: Target,
}

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(&args.file)?;

    // Parse the string of data into serde_json::Value.
    let json_file: Value = serde_json::from_str(&content)?;

    let mut decls = vec![];
    let prefix = "";

    generate_decls(&mut decls, prefix, &json_file, &args.target);

    let file_name = build_file_name(&args.file, &args.output_file, &args.target);

    write_file(&file_name, &decls, &args.target)?;
});

fn write_file(output_file_name: &String, decls: &Vec<String>, target: &Target) -> Result<()> {
    let mut output = File::create(output_file_name)?;

    match target {
        &Target::Shell => {
            writeln!(output, "#!/bin/sh")?;
        }
        _ => {}
    };

    for decl in decls {
        writeln!(output, "{}", decl)?;
    }
    output.sync_all()?;
    Ok(())
}

fn build_file_name(
    input_file_name: &String,
    output_file_name: &Option<String>,
    target: &Target,
) -> String {
    match output_file_name {
        &Some(ref fname) => fname.clone(),
        &None => {
            let path = Path::new(input_file_name);
            let extension = match target {
                &Target::Shell => "sh",
                &Target::Salt => "sls",
            };
            path.with_extension(extension)
                .to_str()
                .expect("invalid output file name")
                .to_string()
                .clone()
        }
    }
}

fn generate_decls<'a>(
    declarations: &'a mut Vec<String>,
    current_prefix: &str,
    json: &Value,
    target: &Target,
) -> &'a mut Vec<String> {
    match json.as_object() {
        Some(m) => {
            m.iter().for_each(|(key, val)| {
                let separator = if current_prefix.is_empty() { "" } else { "_" };
                if val.is_object() {
                    generate_decls(
                        declarations,
                        format!("{}{}{}", current_prefix, separator, key).as_str(),
                        val,
                        target,
                    );
                } else {
                    let new_decl = match target {
                        &Target::Shell => format!(
                            "export {}{}{}={}",
                            current_prefix.to_uppercase(),
                            separator,
                            key.to_uppercase(),
                            val
                        ),
                        &Target::Salt => format!(
                            "salt '*' environ.setval {}{}{} {}",
                            current_prefix.to_uppercase(),
                            separator,
                            key.to_uppercase(),
                            val
                        ),
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
