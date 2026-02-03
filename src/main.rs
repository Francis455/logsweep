use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn write_to_file(filename: &str, content: &str) {
    let mut file = File::create(filename)
        .expect("Could not create output file");

    file.write_all(content.as_bytes())
        .expect("Could not write to file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Basic help message
    if args.len() < 2 {
        println!("LogSweep Usage:");
        println!("  logsweep clean <file> [--errors] [--warnings] [--output <file>]");
        println!("  logsweep search <file> <keyword>");
        return;
    }

    let command = &args[1];

    // CLEAN COMMAND
    if command == "clean" {
        if args.len() < 3 {
            println!("Usage: logsweep clean <file>");
            return;
        }

        let file = &args[2];
        let show_errors = args.contains(&"--errors".to_string());
        let show_warnings = args.contains(&"--warnings".to_string());
        let output_position = args.iter().position(|x| x == "--output");

        let content = fs::read_to_string(file)
            .expect("Could not read the file");

        let mut result = String::new();

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if show_errors && !line.contains("ERROR") {
                continue;
            }

            if show_warnings && !line.contains("WARN") {
                continue;
            }

            result.push_str(line);
            result.push('\n');
        }

        if let Some(pos) = output_position {
            if pos + 1 >= args.len() {
                println!("Please provide an output file name");
                return;
            }

            let output_file = &args[pos + 1];
            write_to_file(output_file, &result);
            println!("Output written to {}", output_file);
        } else {
            print!("{}", result);
        }

    // SEARCH COMMAND
    } else if command == "search" {
        if args.len() < 4 {
            println!("Usage: logsweep search <file> <keyword>");
            return;
        }

        let file = &args[2];
        let keyword = &args[3];

        let content = fs::read_to_string(file)
            .expect("Could not read the file");

        for line in content.lines() {
            if line.contains(keyword) {
                println!("{}", line);
            }
        }

    // UNKNOWN COMMAND
    } else {
        println!("Unknown command: {}", command);
    }
}

