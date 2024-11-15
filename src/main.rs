#![allow(unused_imports)]
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

// Import console output styles and styling functions
mod styles;
use styles::{
    PROMPT_STYLE, ITALIC_STYLE, NOTE_STYLE, KEY_STYLE, VALUE_STYLE, RESET_STYLE,
    style_key, style_note, style_italic, style_prompt, style_value
};

fn main() {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // If no arguments are provided, show help by default.
    if args.len() < 2 {
        print_help();
        return;
    }

    // Match the first argument to determine the command.
    match args[1].as_str() {
        "show" | "view" | "find" | "read" => {
            // Look for the --path flag and extract the path if provided.
            let dir = parse_path_argument(&args);
            // Look for an optional key argument (e.g., dotenv-tool show KEY_NAME --path /path)
            let key = args.get(2).filter(|&arg| arg != "--path").map(|s| s.as_str());
            show_env_file(dir, key);
        }
        "set" | "config" | "update" | "edit" => {
            if args.len() < 3 {
                eprintln!("Error: 'config' command requires a property name.");
                print_help();
                return;
            }
            let property = &args[2];
            let dir = parse_path_argument(&args);

            // Determine if a value is provided
            let value = if args.contains(&"--path".to_string()) {
                let path_index = args.iter().position(|x| x == "--path").unwrap();
                if path_index > 3 {
                    Some(args[3].as_str())
                } else {
                    None
                }
            } else {
                args.get(3).map(|val| val.as_str())
            };

            config_env_file(dir, property, value);
        }
        "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

/// Function to parse the path argument if the `--path` flag is present.
/// Defaults to the current directory if `--path` is not provided or no path follows it.
fn parse_path_argument(args: &[String]) -> &str {
    if let Some(index) = args.iter().position(|arg| arg == "--path") {
        if args.len() > index + 1 {
            return &args[index + 1];
        } else {
            eprintln!("Error: '--path' flag provided but no path specified. Using current directory.");
        }
    }
    "."
}

/// Function to read and print the contents of a `.env` file in the given directory,
/// separating each line into key-value pairs, while ignoring blank lines and comments.
/// Function to read and print the contents of a `.env` file in the given directory,
/// showing all properties or only the specified key's value.
fn show_env_file(dir: &str, key: Option<&str>) {
    let env_path = Path::new(dir).join(".env");

    if !env_path.exists() || !env_path.is_file() {
        eprintln!("No .env file found in the specified directory: {}", env_path.display());
        return;
    }

    let file = File::open(&env_path).expect("Failed to open .env file");
    let reader = BufReader::new(file);

    let mut key_found = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line from .env file");
        let trimmed = line.trim();

        // Skip blank lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let env_path: PathBuf = Path::new(dir).join(".env");
        let env_path_str = env_path.to_str().unwrap_or("[Invalid UTF-8 path]");
        if let Some((line_key, line_value)) = parse_env_line(trimmed) {
            if let Some(target_key) = key {
                if line_key == target_key {
                    println!("{}Showing from {}{}", NOTE_STYLE, style_italic(&env_path_str), RESET_STYLE);
                    println!("{}: {}", style_key(&target_key), style_value(&line_value));
                    key_found = true;
                    break;
                }
            } else {
                println!("{}{}{}: {}{}{}", KEY_STYLE, line_key, RESET_STYLE, VALUE_STYLE, line_value, RESET_STYLE);
            }
        }
    }

    // If a specific key was requested but not found, inform the user.
    if let Some(target_key) = key {
        if !key_found {
            eprintln!("Key '{}' not found in .env file.", target_key);
        }
    }
}

/// Function to add or update a property in the .env file.
fn config_env_file(path: &str, property: &str, value: Option<&str>) {
    // Determine if the path is a directory or a specific file
    let env_path = if Path::new(path).is_dir() {
        Path::new(path).join(".env") // Append .env if it's a directory
    } else {
        Path::new(path).to_path_buf() // Use the provided file path directly
    };

    // Check if the .env file exists, if not create it.
    if !env_path.exists() {
        File::create(&env_path).expect("Failed to create .env file");
    }

    // Get the value, prompting the user if none was provided.
    let formatted_value = match value {
        Some(val) => {
            // Preserve quotes if value is wrapped in single or double quotes, otherwise use as-is.
            if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                val.to_string() // Value already has quotes, keep as-is
            } else {
                val.to_string() // No quotes, add value directly
            }
        }
        None => {
            // Prompt the user for input and wrap it in double quotes.
            let user_input = prompt_for_value(property);
            format!("\"{}\"", user_input)
        }
    };

    // Read the current contents of the file and look for the property.
    let file = File::open(&env_path).expect("Failed to open .env file");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut property_found = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line from .env file");
        let trimmed = line.trim();

        // If the line is a comment or empty, keep it as is.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.push(line);
            continue;
        }

        // Check if the line contains the specified property.
        if let Some((key, _)) = parse_env_line(trimmed) {
            if key == property {
                // Update the value for the existing property.
                lines.push(format!("{}={}", property, formatted_value));
                property_found = true;
            } else {
                // Keep the line as is if it’s not the property we’re looking for.
                lines.push(line);
            }
        } else {
            lines.push(line);
        }
    }

    // If the property was not found, add it as a new line.
    if !property_found {
        lines.push(format!("{}={}", property, formatted_value));
    }

    // Write the updated content back to the .env file.
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&env_path)
        .expect("Failed to open .env file for writing");
    for line in lines {
        writeln!(file, "{}", line).expect("Failed to write to .env file");
    }

    // println!("Set {} to {} in .env file at {}", property, formatted_value, env_path.display());
    println!("{}Set {}{}{} to {}{}{} in {}{}{}", ITALIC_STYLE, KEY_STYLE, property, RESET_STYLE.to_string()+ITALIC_STYLE, VALUE_STYLE, formatted_value, RESET_STYLE.to_string()+ITALIC_STYLE, NOTE_STYLE, env_path.display(), RESET_STYLE);
}

/// Prompts the user to enter a value for the specified property.
fn prompt_for_value(property: &str) -> String {
    print!("Enter a new value for {}: ", property);
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string() // Return the trimmed input
}

/// Function to parse a line in KEY=VALUE format and return the key and value as a tuple.
/// Returns None if the line is not in the correct format.
fn parse_env_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() == 2 {
        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
    } else {
        None
    }
}

/// Function to display help information.
/// Function to display help information.
fn print_help() {
    println!(
        "dotenv-tool: A tool to view and modify .env files from the terminal.\n\n\
        USAGE:\n\
        \tdotenv-tool <COMMAND> [OPTIONS] [ARGS]\n\n\
        COMMANDS:\n\
        \tshow [KEY] [--path <directory>]\n\
        \t\tDisplay the contents of the .env file, or only the specified KEY's value if provided.\n\
        \t\tThe --path option can specify the directory containing the .env file.\n\
        \t\tExamples:\n\
        \t\t\tdotenv-tool show\n\
        \t\t\tdotenv-tool show DB_NAME --path /path/to/dir\n\n\
        \tset <KEY> [VALUE] [--path <directory>]\n\
        \t\tSet or update a property in the .env file. If VALUE is omitted, you will be prompted to enter it interactively.\n\
        \t\tValues wrapped in single or double quotes will retain those quotes. If prompted, the value will be saved in double quotes by default.\n\
        \t\tAliases for 'set' include 'config' and 'update'.\n\
        \t\tExamples:\n\
        \t\t\tdotenv-tool set DB_USER my_user --path /path/to/dir\n\
        \t\t\tdotenv-tool set API_KEY --path /path/to/.env\n\
        \t\t\tdotenv-tool config MY_KEY \"quoted value\"\n\n\
        OPTIONS:\n\
        \t--help, -h\tPrints help information.\n\n\
        EXAMPLES:\n\
        \tdotenv-tool show\n\
        \tdotenv-tool show SOME_KEY\n\
        \tdotenv-tool set NEW_KEY \"new_value\"\n\
        \tdotenv-tool set ANOTHER_KEY --path ./specific/dir\n"
    );
}