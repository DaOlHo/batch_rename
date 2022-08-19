use std::env::Args;

#[derive(Debug)]
pub struct ArgParser;

impl ArgParser {
    // Consumes, is not meant to be used again
    pub fn props_from(args: Args) -> AppProps {
        let args: Vec<String> = args.collect();

        // Arguments gleaned from cmd
        let path: Option<String> = match args.get(1) {
            Some(i) => Some(i.to_string()),
            None => None
        };
        let pattern: Option<String> = match args.get(2) {
            Some(i) => Some(i.to_string()),
            None => None
        };
        let arguments: &Option<&[String]> = &args.get(3..);

        // App properties, set by arguments after path and pattern
        let mut is_verbose: bool = false;
        let mut should_print_help: bool = false;
        let mut should_preserve_extensions: bool = true;
        let mut should_print_version: bool = false;
        let mut match_pattern: String = String::from("");

        // Just print help if user needs it before screaming (arg 0 is always program path)
        if args.len() < 2 || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
            should_print_help = true;
        }
        else if args.contains(&String::from("--version")) || args.contains(&String::from("-v")) {
            should_print_version = true;
        }
        else if arguments.is_some() {
            let arguments: &[String] = arguments.unwrap();

            // For loop isn't used here so that arguments can be skip over if they are 2 entries long
            let mut i: usize = 0;
            while i < arguments.len() {
                let arg: &str = &arguments.get(i).unwrap()[..];

                match arg {
                    "--verbose" | "-V" => is_verbose = true,
                    "--match-pattern" | "-m" => { match_pattern = Self::get_prop_val(arguments, &i); i += 1; },
                    "--no-preserve" | "-n" => should_preserve_extensions = false,
                    &_ => println!("Argument '{}' not recognized", arg)
                }

                // Increment
                i += 1;
            }
        }

        return AppProps {
            path,
            pattern,
            is_verbose,
            should_print_help,
            should_preserve_extensions,
            should_print_version,
            match_pattern,
        }
    }

    // Quick func to safely unwrap property
    fn get_prop_val(arguments: &[String], index: &usize) -> String {
        return String::from(arguments
            .get(index + 1)
            .unwrap_or(&String::from(""))
        );
    }
}

// Holds properties of app
#[derive(Debug)]
pub struct AppProps {
    pub path: Option<String>,
    pub pattern: Option<String>,
    pub is_verbose: bool,
    pub should_print_help: bool,
    pub should_preserve_extensions: bool,
    pub should_print_version: bool,
    pub match_pattern: String,
}