#![allow(unused_imports)]       //To allow unused libraries
#![allow(dead_code)]            //To allow stubs
#![allow(unused_assignments)]   //To allow me to initialize variables
#![allow(unused_variables)]     
use std::{env::args, fs};
use std::io::Read;
use std::path::Path;
use regex::Regex;                                
use std::fs::{File, Metadata};
use walkdir::WalkDir;
use colored::*;
use num_cpus;
use rayon::prelude::*;

fn handle_grep(regex: String, path: String) -> std::io::Result<()> {
    let re = Regex::new(&regex).unwrap();
    let mut f = File::open(&path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    for line in contents.lines() {
        if let Some(captures) = re.captures(line) {
            if let Some(pattern_match) = captures.get(0) {
                let (start, end) = (pattern_match.start(), pattern_match.end());
                let (before, matched, after) = (&line[..start], &line[start..end], &line[end..]);
                println!("{}:{}{}{}", path.purple(), before, matched.blue(), after); 
            }
        }
    }
    Ok(())
}

fn handle_grep_count(regex: String, path: String) -> std::io::Result<()> {
    let re = Regex::new(&regex).unwrap();
    let mut f = File::open(&path)?;
    let mut contents: String = String::new();
    let mut count: i32 = 0;
    f.read_to_string(&mut contents)?;
    for line in contents.lines() {
        if re.is_match(line){
            count = count + 1;
        }
    }
    println!("{}:{}", path.purple(), count);
    Ok(())
}

//Consider adding support for -q, -E, -n
fn main() {
    let mut regex_pattern: String = "".to_string();
    let mut files: Vec<String> = Vec::new(); 
    let mut recurse_dirs: bool = false;
    let mut track_counts: bool = false;
    let args: Vec<String> = args().skip(1).collect();
    for arg in args {
        if arg.starts_with("-") {
            match &arg[1..] {
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
    if recurse_dirs {
        let max_threads = num_cpus::get();
        let num_threads: usize = if files.len() > max_threads { max_threads } else { files.len() };
        //Another thing that must be done... 
        //if a thread encounters a directory, expand it and put it into a list of files
/*
        let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
        pool.install(|| {
            files.par_iter().for_each(|file| {
                if track_counts {
                    let _ = handle_grep_count(regex_pattern.clone(), file.clone());
                } 
                else {
                    let _ = handle_grep(regex_pattern.clone(), file.clone());
                }
            });
        });
*/
    }
    else {
        for file in files {
            let metadata = fs::metadata(&file).unwrap();
            if metadata.is_file() {
                if track_counts {
                    let _ = handle_grep_count(regex_pattern.clone(), file.clone());
                }
                else {
                    let _ = handle_grep(regex_pattern.clone(), file.clone());
                }
            }
        }
    }   
}

