mod arg_parser;
mod file_formatter;

use std::env;
use arg_parser::{ArgParser, AppProps};
use file_formatter::FileFormatter;

// TODO:
//  Implement match pattern (only rename files that conform to pattern)
//  Write comprehensive HELP
fn main() {
    let properties: AppProps = ArgParser::props_from(env::args());

    // Print help/version and return
    if properties.should_print_help {
        let help_text: &str = include_str!("help.txt");
        print!("{}", help_text);
        return;
    }
    else if properties.should_print_version {
        let version_text: &str = include_str!("version.txt");
        print!("{}", version_text);
        return;
    }

    // Check if both path and pattern are present, exit gracefully if they aren't
    if properties.path == None || properties.pattern == None {
        println!("Expected 2 positional arguments:\nbatch_rename <path> <pattern>");
        return;
    }

    // Renaming is handed off to this struct, since it runs last it can crash by using return early
    FileFormatter::perform_format(properties);
}