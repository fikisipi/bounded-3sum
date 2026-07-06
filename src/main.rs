//! Interactive REPL for the 3SUM solver.
//!
//! Type a list of integers like `[1, 2, 3]` (brackets optional; commas or
//! whitespace both work) and it reports whether any three distinct entries
//! sum to zero. `quit` / `exit` / Ctrl-D leaves.

use std::io::{self, BufRead, Write};

use three_sum::Solution;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("3SUM REPL — enter a list like [1, 2, 3]. Ctrl-D or `quit` to exit.");

    loop {
        print!("> ");
        stdout.flush().ok();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("read error: {err}");
                break;
            }
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if matches!(trimmed, "quit" | "exit" | ":q") {
            break;
        }

        match parse_list(trimmed) {
            Ok(nums) => match Solution::has_three_sum(nums) {
                Some(true) => println!("=> true (a zero-sum triple exists)"),
                Some(false) => println!("=> false (no zero-sum triple)"),
                None => println!("=> None (value range too large for this solver)"),
            },
            Err(err) => eprintln!("parse error: {err}"),
        }
    }
}

/// Parses `[1, 2, 3]`, `1 2 3`, `1,2,3`, etc. into a `Vec<i32>`.
fn parse_list(input: &str) -> Result<Vec<i32>, String> {
    input
        .trim_matches(|c| c == '[' || c == ']' || char::is_whitespace(c))
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|token| !token.is_empty())
        .map(|token| {
            token
                .parse::<i32>()
                .map_err(|_| format!("`{token}` is not an integer"))
        })
        .collect()
}
