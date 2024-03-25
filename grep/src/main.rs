#![allow(unused_imports)]   //To allow unused libraries
#![allow(dead_code)]        //To allow stubs
#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::env::args;
use std::io::Read;
use std::path::Path;
use regex::Regex;                                
use std::fs::File;
use walkdir::WalkDir;
use colored::*;

fn handle_grep(regex: String, path: String) -> std::io::Result<()> {
    let re = Regex::new(&regex).unwrap();
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    for line in contents.lines() {
        if let Some(captures) = re.captures(line) {
            if let Some(pattern_match) = captures.get(0) {
                let (start, end) = (pattern_match.start(), pattern_match.end());
                let (before, matched, after) = (&line[..start], &line[start..end], &line[end..]);
                println!("{}{}{}", before, matched.yellow(), after); 
            }
        }
    }
    Ok(())
}

fn main() {
    let mut regex_pattern: String = "".to_string();
    let mut files: Vec<String> = Vec::new(); 
    let mut is_extended: bool = false;
    let mut recurse_dirs: bool = false;
    let mut track_counts: bool = false;
    let args: Vec<String> = args().skip(1).collect();
    for arg in args {
        if arg.starts_with("-") {
            match &arg[1..] {
            "e" => is_extended = true,
            "r" => recurse_dirs = true,
            "c" => track_counts = true,
            _ => panic!("Bad Argument!"),
            }
        }
        else {
            if regex_pattern == "" {
                match Regex::new(&arg) {
                    Ok(_) => regex_pattern = arg,
                    Err(_) => panic!("Invalid Regex Pattern"),
                }
            }
            else {
                if Path::new(&arg).exists() { files.push(arg); }
                else { panic!("Invalid file: {}", arg) }
            } 
        }
    }
    for file in files {
        let _ = handle_grep(regex_pattern.clone(), file);
    }
}

