use crate::arg_parser::AppProps;
use std::{fs::{self, DirEntry}, ffi::OsStr};

#[derive(Debug)]
pub struct FileFormatter;

impl FileFormatter {
    pub fn perform_format(properties: AppProps) {
        let path: &String = properties.path.as_ref().unwrap();
        let file_paths = match fs::read_dir(path) {
            Ok(i) => i,
            Err(e) => {
                println!("Error reading directory: {}", e);
                return; // Gracefully crash app while giving error reasoning
            }
        };

        // for $num variable
        let mut i: usize = 1;

        for file_path in file_paths {
            let dir_entry: DirEntry = match file_path {
                Ok(i) => i,
                Err(e) => {
                    println!("Error retrieving file: {}", e);
                    return;
                }
            };

            let mut file_name: String = String::from(properties.pattern.as_ref().unwrap());

            // Big block of replacement
            file_name = file_name.replace("$num", &i.to_string()[..]);
            file_name = file_name.replace("$name", &Self::get_stem(&dir_entry)[..]);

            let mut final_file_path: String = format!("{}{}{}", path, "/", file_name);

            // Block of options
            if properties.should_preserve_extensions {
                final_file_path = format!("{}.{}", final_file_path, Self::get_extension(&dir_entry));
            }

            // Print verbose information
            if properties.is_verbose {
                println!("{} => {}", Self::get_path(&dir_entry), final_file_path);
            }

            // Finally rename file, exiting on error
            match fs::rename(dir_entry.path().to_str().unwrap(), final_file_path) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error writing file name: {}", e);
                    return;
                }
            };

            // Update $num variable
            i += 1;
        }
    }

    fn get_extension(dir_entry: &DirEntry) -> String {
        return dir_entry.path().extension().and_then(OsStr::to_str).unwrap_or("").to_owned();
    }

    fn get_stem(dir_entry: &DirEntry) -> String {
        return dir_entry.path().file_stem().and_then(OsStr::to_str).unwrap_or("").to_owned();
    }

    fn get_path(dir_entry: &DirEntry) -> String {
        return dir_entry.path().as_os_str().to_str().unwrap_or("").to_owned();
    }
}