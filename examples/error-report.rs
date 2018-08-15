extern crate rnix;

use rnix::Error as NixError;
use std::{env, fs, io::{self, Write}};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let file = match env::args().skip(1).next() {
        Some(file) => file,
        None => {
            eprintln!("Usage: dump-ast <file>");
            return;
        }
    };
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("error reading file: {}", err);
            return;
        }
    };
    let span = match rnix::parse(&content) {
        Ok(_) => { println!("success"); return; },
        Err(err) => {
            writeln!(stdout, "error: {}", err).unwrap();
            match err {
                NixError::TokenizeError(span, _) => span,
                NixError::ParseError(Some(span), _) => span,
                NixError::ParseError(None, _) => return
            }
        }
    };

    writeln!(stdout, "{:?}", span);
    writeln!(stdout).unwrap();

    let start = span.start;
    let end = span.end.unwrap_or(start+1);

    let prev_line_end = content[..start].rfind('\n');
    let prev_line = content[..prev_line_end.unwrap_or(0)].rfind('\n').map(|n| n+1).unwrap_or(0);
    let next_line = content[end..].find('\n').map(|n| n + 1 + end).unwrap_or(content.len());
    let next_line_end = content[next_line..].find('\n').map(|n| n + next_line).unwrap_or(content.len());

    let mut pos = prev_line;
    loop {
        let line = content[pos..].find('\n').map(|n| n + pos).unwrap_or(content.len() - 1);

        writeln!(stdout, "{}", &content[pos..line]).unwrap();
        if pos >= prev_line_end.map(|n| n + 1).unwrap_or(0) && line < next_line {
            for i in pos..line {
                if i >= start && i < end {
                    stdout.write_all(&[b'^']).unwrap();
                } else {
                    stdout.write_all(&[b' ']).unwrap();
                }
            }
            writeln!(stdout).unwrap();
        }

        pos = line+1;
        if pos >= next_line_end {
            break;
        }
    }
}