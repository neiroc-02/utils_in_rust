#![allow(unused_imports)]   //To allow unused libraries
#![allow(dead_code)]        //To allow stubs
#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::env::args;
use regex::Regex;                                
use std::fs::File;
use walkdir::WalkDir;
use colored::*;

fn main() {
    let mut regex_pattern: String = "".to_string();
    let mut files: Vec<String> = Vec::new(); 
    let mut is_extended: bool = false;
    let mut recurse_dirs: bool = false;
    let mut track_counts: bool = false;
    let args: Vec<String> = args().skip(1).collect(); //Gathering command line args
    //Handling command line options
    for arg in args {
        if arg.starts_with("-") {
            match &arg[1..] {
            "e" => {
                is_extended = true;
            }
            "r" => {
                recurse_dirs = true;
            }
            "c" => {
                track_counts = true
            }
            _ => panic!("Bad Argument!")
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
                files.push(arg);
            } 
        }
    }
    println!("Finished argument parsing without errors!");
}

