use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::{path::Path, process::Command};

fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("aliases-file")
                .short("a")
                .long("aliases-file")
                .help("The aliases file to append the aliases to")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let aliases_file = matches.value_of("aliases-file").unwrap();
    println!("Appending aliases to {}", aliases_file);

    let file = Path::new(aliases_file);
    if !file.exists() {
        todo!()
    }

    let coreutils_help_output = Command::new("coreutils").arg("-h").output()?.stdout;
    let output = String::from_utf8(coreutils_help_output)?;

    let functions = output
        .lines()
        .filter(|line| !line.is_empty() && line.starts_with(' '))
        .map(|line| line.trim())
        .fold(String::new(), |acc, line| acc + line);

    let mut functions_to_alias = Vec::new();
    for function in functions.split(',') {
        let function = function.trim();
        if 1 >= function.len() {
            continue;
        }

        functions_to_alias.push(function);
    }

    for alias in functions_to_alias
        .iter()
        .map(|function| format!("Set-Alias -Name {} -Value Get-{}", function, function))
    {
        println!("{}", alias);
    }

    for function in functions_to_alias.iter().map(|function| {
        format!(
            "function Get-{} {{ coreutils {} $args }}",
            function, function
        )
    }) {
        println!("{}", function);
    }

    Ok(())
}
