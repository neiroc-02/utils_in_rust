#![allow(unused_imports)]   //To allow unused libraries
#![allow(dead_code)]        //To allow stubs
use std::env::args;
use regex::Regex;                                
use std::fs::File;
use colored::*;
use walkdir::WalkDir;
use clap::{Arg, App};

/*
 Tasks to reimplement grep:
 0. Be able to handle command line arguments
 1. Be able to find a file in current directory and dig through the tree
 2. Load that file
 3. Read line by line through the file to find regex matches
 4. Print the matches with colored matches
*/
fn find_directory(){
    print!("Hello World")
}


fn main() {
    let args: Vec<String> = args().skip(1).collect();
    for elem in args {
        print!("{} \n", elem)
    }
    
}

