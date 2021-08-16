use clap::{App, AppSettings, Arg, SubCommand};
mod commands;
mod utils;
use anyhow::Result;

#[allow(unused_imports)]
#[macro_use]
extern crate self_update;

fn main() -> Result<()> {
    let version: String = "0.1.0".to_string();
    let app = App::new("udymts")
        .version(&*version)
        .author("Daggy1234 <daggy@daggy.tech>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .about("Python programming by ceaser ciphered. Very esolang")
        .bin_name("udymts")
        .subcommand(
            SubCommand::with_name("decode")
                .about("turn amazing udymts code into python")
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .takes_value(true)
                        .help("path to file to read and decode")
                        .value_name("FILE"),
                )
                .arg(
                    Arg::with_name("text")
                        .long("text")
                        .short("t")
                        .takes_value(true)
                        .help("text to decode")
                        .value_name("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("encode")
                .about("convert boring python code to udymts")
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .takes_value(true)
                        .help("path to file to read and decode")
                        .value_name("FILE"),
                )
                .arg(
                    Arg::with_name("text")
                        .long("text")
                        .short("t")
                        .takes_value(true)
                        .help("text to decode")
                        .value_name("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("eval")
                .about("evaluate your udymys file or line of code")
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .takes_value(true)
                        .help("path to file to read and decode")
                        .value_name("FILE"),
                )
                .arg(
                    Arg::with_name("text")
                        .long("text")
                        .short("t")
                        .takes_value(true)
                        .help("text to decode")
                        .value_name("INPUT"),
                ),
        );

    let matches = app.get_matches();

    if let Some(subcmd) = matches.subcommand_matches("decode") {
        if let Some(file) = subcmd.value_of("file") {
            commands::decode::run_with_file(file.to_string())?;
        } else if let Some(text) = subcmd.value_of("text") {
            commands::decode::run_with_text(text.to_string())?;
        } else {
            println!("No Argument. Provide  `--text <text>` or `--file <path>`")
        }
    } else if let Some(subcmd) = matches.subcommand_matches("encode") {
        println!("{:?}", subcmd);
        if let Some(file) = subcmd.value_of("file") {
            commands::encode::run_with_file(file.to_string())?;
        } else if let Some(text) = subcmd.value_of("text") {
            commands::encode::run_with_text(text.to_string())?;
        } else {
            println!("No Argument. Provide  `--texno turns out tt <text>` or `--file <path>`")
        }
    } else if let Some(subcmd) = matches.subcommand_matches("eval") {
        println!("{:?}", subcmd);
        if let Some(file) = subcmd.value_of("file") {
            commands::eval::run_with_file(file.to_string())?;
        } else if let Some(text) = subcmd.value_of("text") {
            commands::eval::run_with_text(text.to_string())?;
        } else {
            println!("No Argument. Provide  `--texno turns out tt <text>` or `--file <path>`")
        }
    }
    Ok(())
}
