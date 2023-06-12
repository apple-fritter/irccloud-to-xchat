use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use regex::Regex;

// Structure to represent an XChat line
struct XChatLine {
    timestamp: String,
    sender: String,
    message: String,
}

// Function to parse an IRCcloud log line into an XChat line
fn parse_irccloud_line(line: &str) -> Option<XChatLine> {
    let re = Regex::new(r"^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\t<([^>]+)>\t(.*)$").unwrap();
    if let Some(captures) = re.captures(line) {
        let timestamp = captures[1].to_string();
        let sender = captures[2].to_string();
        let message = captures[3].to_string();

        Some(XChatLine {
            timestamp,
            sender,
            message,
        })
    } else {
        None
    }
}

// Function to convert an IRCcloud log file to XChat log format
fn convert_to_xchat(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(xchat_line) = parse_irccloud_line(&line) {
                let formatted_line = format!(
                    "[{}] {}\t{}",
                    xchat_line.timestamp, xchat_line.sender, xchat_line.message
                );
                writer.write_all(formatted_line.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }
    }

    writer.flush()?;

    Ok(())
}

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: irccloud-to-xchat <input_file> <output_file>");
        return;
    }

    // Parse input and output file paths
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    // Convert IRCcloud log to XChat log
    if let Err(err) = convert_to_xchat(input_path, output_path) {
        println!("Error: {}", err);
    } else {
        println!("Conversion complete.");
    }
}
