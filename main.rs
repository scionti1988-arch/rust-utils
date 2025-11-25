use std::env;
use std::process;

use table_qa::generate_qa_from_csv;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: table-qa <path-to-csv>");
        process::exit(1);
    }

    let path = &args[1];

    match generate_qa_from_csv(path) {
        Ok(pairs) => {
            for (idx, qa) in pairs.iter().enumerate() {
                println!("Q{}: {}", idx + 1, qa.question);
                println!("A{}: {}", idx + 1, qa.answer);
                println!();
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}
