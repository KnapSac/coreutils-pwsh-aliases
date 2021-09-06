use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::{fs::OpenOptions, io::Write, process::Command};

const ALIAS_SECTION_IDENT: &str = "### coreutils aliases ###";

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

    let mut file = OpenOptions::new().append(true).open(aliases_file)?;

    let coreutils_help_output = Command::new("coreutils").arg("-h").output()?.stdout;
    let output = String::from_utf8(coreutils_help_output)?;

    // These commands have readonly or constant pwsh aliases, as such they need a postfix
    let functions_to_postfix = vec!["sleep", "sort", "tee"];

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

        if functions_to_postfix.contains(&function) {
            functions_to_alias.push(format!("{}-uu", function));
        } else {
            functions_to_alias.push(function.to_string());
        }
    }

    write!(file, "\r\n")?;
    write!(file, "{}\r\n", ALIAS_SECTION_IDENT)?;

    for alias in functions_to_alias.iter().map(|function| {
        format!(
            "Set-Alias -Name {} -Value Get-{} -Option AllScope",
            function, function
        )
    }) {
        write!(file, "{}\r\n", alias)?;
    }

    write!(file, "\r\n")?;

    for function in functions_to_alias.iter().map(|function| {
        format!(
            "function Get-{} {{ coreutils {} $args }}",
            function, function
        )
    }) {
        write!(file, "{}\r\n", function)?;
    }

    write!(file, "\r\n")?;
    write!(file, "{}\r\n", ALIAS_SECTION_IDENT)?;

    Ok(())
}
