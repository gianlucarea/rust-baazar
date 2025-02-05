use std::{fs::File, io::BufReader, io::BufRead};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    pattern: String,
    path: std::path::PathBuf,

}

fn main() {
    let args = Cli::parse();
    match args.command.as_str() {
        "cat" => cat(args.path),
        "grep" => grep(args.path,args.pattern),
        _ => panic!()
    }
}

fn grep(path: std::path::PathBuf, pattern: String){
    let f = File::open(&path).unwrap();
    let f = BufReader::new(f);
    for (index,line) in f.lines().enumerate() {
        if line.unwrap().contains(&pattern){
            println!("Found the word {} in line {}",pattern,index);
        }   
    }
}

fn cat(path: std::path::PathBuf){
    let f = File::open(&path).unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
        println!("{}",line.unwrap());
     }
}

