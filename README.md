# dotenv-tool

`dotenv-tool` is a command-line utility for managing and configuring `.env` files. It provides a convenient way to view, add, and update environment variables in `.env` files, with support for specifying file paths and handling values interactively.

## Features

- **View all variables** in a `.env` file or display the value of a specific variable.
- **Set or update variables** without having to open the file in a text editor.
- **Run within .env directory or specify a custom path** using the `--path` flag.
- **Support regular text input** allows you to enter input containing whitespace, it will automatically get wrapped in quotation marks when using `set FIELD_NAME` without adding a value.

## Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd dotenv-tool
   ```
2. **Build the tool**:
   ```bash
   cargo build --release
   ```
3. **Install the binary**:
   ```bash
   sudo mv target/release/dotenv-tool /usr/local/bin/
   ```
   
## Usage
```bash
dotenv-tool <COMMAND> [OPTIONS] [ARGS]
```

### Commands

```
show [KEY] [--path <directory>]
```

Display the contents of the .env file, or show only the value of the specified KEY if provided.

- If no path is specified, the current directory is used by default.

Examples:
  - `dotenv-tool show`
  - `dotenv-tool show DB_NAME --path /path/to/dir`


```
set <KEY> [VALUE] [--path <directory>]
```

Set or update a property in the .env file. If VALUE is omitted, you will be prompted to enter it interactively.

- Single or double-quoted values will retain their quotes.
- If prompted interactively, the value will be saved in double quotes by default.
- Aliases: config, update

Examples:
  - `dotenv-tool set DB_USER my_user --path /path/to/dir`
  - `dotenv-tool set API_KEY --path /path/to/.env`
  - `dotenv-tool config MY_KEY "quoted value"`

### Options
`--path <directory>` - specify the directory containing the .env file. If not provided, the current directory is used by default.

`--help, -h` - displays help information.

## Examples
```bash
#Show all variables in the .env file
dotenv-tool show

# Show a specific variable’s value
dotenv-tool show SOME_KEY

# Set a new key with a value
dotenv-tool set NEW_KEY "new_value"

# Set or update a key, specifying a custom directory for .env
dotenv-tool set ANOTHER_KEY --path ./specific/dir

# Use the ‘config’ alias to set a key with a quoted value
dotenv-tool config MY_KEY 'quoted value'
```