# Tabler

**Tabler** is a Rust-based command-line tool that converts CSV, Excel, and JSON data into a structured table format. It's designed to help developers and data analysts quickly visualize and format tabular data directly from the terminal.

## Installation

You have two options to install Tabler:

### 1. Build from source

Make sure you have Rust installed. Then clone the repository and build the project:

```bash
git clone https://github.com/AymaneImr/tabler.git
cd tabler
cargo build --release
```

After building, move the binary to a directory that is part of your system’s `PATH`:

```bash
cp target/release/tabler /usr/local/bin
```

You can now use `tabler` from anywhere in your terminal.

### 2. Download the release file (Recommended for most users)

You can download the precompiled binary from the [Releases page](https://github.com/AymaneImr/tabler/releases):

- Look for the latest `.zip` file (e.g., `tabler-v0.1.0-linux-x86_64.zip`).
- Extract it and move the binary to a directory in your `PATH`, such as `/usr/local/bin`.

Example:

```bash
unzip tabler-v0.1.0-linux-x86_64.zip
sudo cp tabler /usr/local/bin
```

That’s it — you can now run `tabler` from anywhere!

## Usage

Once installed, you can use Tabler to convert various data formats into structured tables. Supported formats include `.csv`, `.xlsx`, and `.json`.

```bash
tabler <PATH> [OPTIONS]
```

### Arguments

- `<PATH>`: The path to the file (required).

### Options

- `-r, --rows <NUMBER>`: Number of rows to display.
- `-d, --default-rows`: Display a default of 200 rows (conflicts with `--rows`).
- `-i, --indent`: Enable indentation to improve readability for wide tables.
- `-s, --sheet-name <NAME>`: For Excel files, specify the sheet to load.
- `-n, --nested`: Use for parsing nested JSON structures.

### Subcommands

- `columns <COL1,COL2,...>`: Display only the specified columns (comma-separated).

### Examples

```bash
# Basic usage with a CSV file
tabler data.csv

# Show only 20 rows
tabler data.json --rows 20

# Use default row limit of 200
tabler data.xlsx --default-rows

# Parse a nested JSON file
tabler nested_data.json --nested

# Specify a sheet in an Excel file
tabler data.xlsx --sheet-name Sheet1

# Show specific columns (using the subcommand)
tabler data.csv columns name,email
```

> **Note:** Support for additional file extensions is planned in future releases.
