use std::{env::{self}, fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match command.as_str() {
        "cat" => cat(PathBuf::from(&args[2])),
        "grep" => grep(&args[2],PathBuf::from(&args[3])),
        "cp" => cp(PathBuf::from(&args[2]),PathBuf::from(&args[3])),
        "mv" => mv(PathBuf::from(&args[2]),PathBuf::from(&args[3])),
        "rm" => rm(PathBuf::from(&args[2])),
        _ => println!("{}", command)
    }
}

fn grep(pattern: &String, path: std::path::PathBuf ){
    let f = File::open(&path).unwrap();
    let f = BufReader::new(f);
    for (index,line) in f.lines().enumerate() {
        if line.unwrap().contains(pattern){
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

fn cp(origin: std::path::PathBuf, destination: std::path::PathBuf){
    let content = fs::read_to_string(origin).unwrap();
    let _ = fs::write(destination, content);
 }

 fn mv(origin: std::path::PathBuf, destination: std::path::PathBuf){
    let content = fs::read_to_string(&origin).unwrap();
    let _ = fs::write(destination, content);
    let _ = fs::remove_file(&origin);
 }

 fn rm(origin: std::path::PathBuf){
    let _ = fs::remove_file(&origin);
 }