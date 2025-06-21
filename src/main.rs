use clap::Parser;
use csvlint::validate;
use std::fs::File;
use std::io::{self, BufReader};
use std::process;

/// A CSV linter that validates CSV files according to RFC 4180
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Field delimiter in the file (e.g., ',' '\t' '|' ':' ';')
    #[arg(short, long, default_value = ",")]
    delimiter: String,

    /// Try to parse improperly escaped quotes
    #[arg(short, long, default_value_t = false)]
    lazyquotes: bool,

    /// Strict RFC 4180 compliance mode (implies comma delimiter and CRLF line endings)
    #[arg(long, default_value_t = false)]
    rfc4180: bool,

    /// CSV file to validate
    file: String,
}

fn main() {
    let args = Args::parse();

    // Handle RFC 4180 strict mode
    let (delimiter_byte, lazy_quotes) = if args.rfc4180 {
        if args.delimiter != "," {
            eprintln!("Warning: --rfc4180 mode requires comma delimiter, ignoring --delimiter option");
        }
        if args.lazyquotes {
            eprintln!("Warning: --rfc4180 mode disables lazy quotes, ignoring --lazyquotes option");
        }
        (b',', false)
    } else {
        // Validate and convert delimiter
        let delimiter_byte = match parse_delimiter(&args.delimiter) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        (delimiter_byte, args.lazyquotes)
    };

    // Warn if not using defaults (unless in RFC 4180 mode)
    if !args.rfc4180 && (args.delimiter != "," || args.lazyquotes) {
        eprintln!("Warning: not using defaults, may not validate CSV to RFC 4180");
    }

    if args.rfc4180 {
        println!("Running in strict RFC 4180 compliance mode");
        println!("- Delimiter: comma (,)");
        println!("- Line endings: CRLF required");
        println!("- Quote escaping: strict");
        println!();
    }

    // Open and validate the file
    let file = match File::open(&args.file) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("file '{}' does not exist", args.file);
            } else {
                eprintln!("error opening file '{}': {}", args.file, e);
            }
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    
    let result = match validate(reader, delimiter_byte, lazy_quotes) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("validation error: {}", e);
            process::exit(1);
        }
    };

    // Handle results
    if result.errors.is_empty() {
        if args.rfc4180 {
            println!("file is valid and complies with RFC 4180");
        } else {
            println!("file is valid");
        }
        process::exit(0);
    }

    // Count different types of errors
    let mut field_count_errors = 0;
    let mut line_ending_errors = 0;
    let mut quote_errors = 0;
    let mut other_errors = 0;

    for error in &result.errors {
        match error.error {
            csvlint::CsvErrorKind::FieldCount => field_count_errors += 1,
            csvlint::CsvErrorKind::InvalidLineEnding => line_ending_errors += 1,
            csvlint::CsvErrorKind::BareQuote | csvlint::CsvErrorKind::Quote | 
            csvlint::CsvErrorKind::UnterminatedQuote => quote_errors += 1,
            _ => other_errors += 1,
        }
    }

    // Print summary
    println!("Found {} validation error(s):", result.errors.len());
    if field_count_errors > 0 {
        println!("  - {} field count error(s)", field_count_errors);
    }
    if line_ending_errors > 0 {
        println!("  - {} line ending error(s) (RFC 4180 requires CRLF)", line_ending_errors);
    }
    if quote_errors > 0 {
        println!("  - {} quote/escaping error(s)", quote_errors);
    }
    if other_errors > 0 {
        println!("  - {} other error(s)", other_errors);
    }
    println!();

    // Print all errors
    for error in &result.errors {
        println!("{}", error);
    }

    if result.halted {
        println!("\nunable to parse any further");
        process::exit(1);
    }

    process::exit(2);
}

fn parse_delimiter(delimiter_str: &str) -> Result<u8, String> {
    match delimiter_str {
        "," => Ok(b','),
        "\\t" => Ok(b'\t'),
        "|" => Ok(b'|'),
        ":" => Ok(b':'),
        ";" => Ok(b';'),
        s if s.len() == 1 => Ok(s.as_bytes()[0]),
        _ => Err(format!(
            "error parsing delimiter '{}', note that only one-character delimiters are supported",
            delimiter_str
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_delimiter() {
        assert_eq!(parse_delimiter(",").unwrap(), b',');
        assert_eq!(parse_delimiter("\\t").unwrap(), b'\t');
        assert_eq!(parse_delimiter("|").unwrap(), b'|');
        assert_eq!(parse_delimiter(":").unwrap(), b':');
        assert_eq!(parse_delimiter(";").unwrap(), b';');
        assert_eq!(parse_delimiter("x").unwrap(), b'x');
        
        assert!(parse_delimiter("").is_err());
        assert!(parse_delimiter("ab").is_err());
    }
} 