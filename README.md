# csvlint (Rust)

A fast CSV linter written in Rust that validates CSV files according to RFC 4180.

## Installation

### From source

```bash
git clone https://github.com/blackstar257/csvlint-rs
cd csvlint-rs
cargo build --release
```

The binary will be available at `target/release/csvlint`.

### Using Cargo

```bash
cargo install csvlint
```

## Usage

```bash
csvlint [OPTIONS] <FILE>
```

### Arguments

- `<FILE>` - The CSV file to validate

### Options

- `-d, --delimiter <DELIMITER>` - Field delimiter in the file (default: ",")
  - Supports: `,` (comma), `\t` (tab), `|` (pipe), `:` (colon), `;` (semicolon)
- `-l, --lazyquotes` - Try to parse improperly escaped quotes
- `--rfc4180` - Strict RFC 4180 compliance mode (enforces comma delimiter and CRLF line endings)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Examples

```bash
# Validate a standard CSV file
csvlint data.csv

# Validate with strict RFC 4180 compliance (comma delimiter, CRLF line endings)
csvlint --rfc4180 data.csv

# Validate a tab-separated file
csvlint --delimiter '\t' data.tsv

# Validate a pipe-separated file
csvlint --delimiter '|' data.txt

# Validate with lazy quote parsing (more lenient)
csvlint --lazyquotes data.csv
```

## Exit Codes

- `0` - File is valid
- `1` - File does not exist or parsing was halted due to fatal errors
- `2` - File contains validation errors

## Features

- **Full RFC 4180 Compliance**: Validates CSV files according to the RFC 4180 standard
  - **Strict mode**: Enforces CRLF line endings and comma delimiters
  - **Line ending validation**: Checks for proper CRLF (`\r\n`) line endings
  - **Quote escaping validation**: Ensures proper quote doubling for escapes
- **Multiple Delimiters**: Supports comma, tab, pipe, colon, and semicolon delimiters
- **Detailed Error Reports**: Provides specific error messages with record numbers and error categories
- **Field Count Validation**: Ensures all records have the same number of fields as the header
- **Quote Validation**: Detects improperly quoted fields and bare quotes
- **Lazy Quote Mode**: Optional mode to parse files with improperly escaped quotes
- **Fast Performance**: Built with Rust for maximum performance using the csv crate
- **Memory Efficient**: Processes files without loading everything into memory

## Error Types

The linter detects several types of CSV format violations:

- **Field Count Errors**: Records with different number of fields than the header
- **Line Ending Errors**: Invalid line endings (RFC 4180 requires CRLF)
- **Quote Errors**: Improperly quoted fields, bare quotes, unterminated quotes
- **Unescaped Special Characters**: Special characters not properly escaped or quoted
- **Encoding Errors**: Invalid UTF-8 sequences
- **I/O Errors**: File reading errors

## Library Usage

This tool can also be used as a Rust library:

```toml
[dependencies]
csvlint = "0.1.0"
```

```rust
use csvlint::validate;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data.csv")?;
    let reader = BufReader::new(file);

    let result = validate(reader, b',', false)?;

    if result.errors.is_empty() {
        println!("File is valid!");
    } else {
        for error in result.errors {
            println!("{}", error);
        }
    }

    Ok(())
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running integration tests

```bash
cargo test integration_tests
```

## RFC 4180 Compliance

This implementation provides comprehensive support for [RFC 4180](https://www.rfc-editor.org/rfc/rfc4180) compliance:

### Strict Mode (`--rfc4180`)
When using the `--rfc4180` flag, the linter enforces:
- **Line Endings**: Must be CRLF (`\r\n`) as specified in RFC 4180
- **Delimiter**: Must be comma (`,`) only
- **Quote Escaping**: Strict validation of quote doubling (e.g., `"He said ""Hello""."`)
- **Field Structure**: Consistent field count across all records

### Standard Mode (default)
In standard mode, the linter is more lenient and accepts:
- Various line endings (LF, CRLF, CR)
- Multiple delimiter types
- More flexible quote handling with `--lazyquotes`

### Compliance Features
- ✅ **CRLF Line Endings**: Validates proper `\r\n` line endings
- ✅ **Field Count Consistency**: Ensures all records have same field count
- ✅ **Quote Escaping**: Validates doubled quotes within quoted fields
- ✅ **Special Character Handling**: Validates proper quoting of fields containing commas, quotes, or line breaks
- ✅ **Header Support**: Optional header row support
- ✅ **MIME Type Compliance**: Follows `text/csv` MIME type specification

## Performance

This Rust implementation leverages the excellent [csv crate](https://docs.rs/csv/) for parsing, which provides:

- Zero-copy parsing where possible
- Efficient memory usage
- Fast field iteration
- Robust error handling

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
