use clap::Parser;
use core::panic;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_delimiter = ' ', num_args=1..)]
    path: Vec<PathBuf>,
}

fn trim_end(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();

        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn run(s: String) -> String {
    // let Ok(new_s) = String::from_str("asfdasdfasdf");
    // new_s
    String::new()
}

fn run_prompt() {
    loop {
        print!(">> ");
        let _ = stdout().flush();
        let mut s = String::new();
        let r = std::io::stdin().read_line(&mut s);

        match r {
            Ok(_) => {
                // print!("{}", s)
            }
            Err(_) => println!("Something went wrong while reading from prompt"),
        };
        trim_end(&mut s);
        let Ok(exit) = String::from_str("exit");
        if s == exit {
            break;
        }
        let output: String = run(s.clone());

        if !output.is_empty() {
            println!("{}", output)
        } else {
            println!("{}", s)
        }
    }
}

fn run_file(path: &PathBuf) {
    let mut contents = String::new();

    if let Ok(mut file) = File::open(path) {
        let _ = file.read_to_string(&mut contents);
    } else {
        panic!("Couldn't open file or file doesn't not exist")
    }
    run(contents);
}

fn main() {
    let args = Args::parse();

    if args.path.is_empty() {
        run_prompt();
    } else if args.path.len() == 1 {
        let path = &args.path[0];
        run_file(path);
    } else {
        panic!("Multiple file parsing not supported yet");
    }
}
