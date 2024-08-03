use std::env;
use std::process::exit;
use std::fs;
use std::io::{self, BufRead};

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path){
        Err(msg) => return Err(msg.to_string()), Ok(contents) => run(&contents),
    }
}

fn run(_contents: &str) -> Result<(), String>{
    return Err("Not Implemented".to_string());
}

fn run_prompt() -> Result<(), String> {
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Couldnt Read Line".to_string()),
    }
    println!("You Wrote {}", buffer);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: blitz [script]");
        exit(64);
    } else if args.len() == 2 {
       match run_file(&args[1]){
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
       }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n {}", msg)
            }
        }
    }

    dbg!(args);
}
