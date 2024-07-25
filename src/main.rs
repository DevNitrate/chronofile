use std::{fs::{self, ReadDir}, process::exit};
use rayon::prelude::*;
use std::env;

fn search(item: &str, start_dir: &str) {
    let dir_err = fs::read_dir(start_dir);
    if dir_err.is_err() {
        return;
    }
    let dir: ReadDir = dir_err.unwrap();
    
    dir.par_bridge().for_each(|f| {
        let list = f.unwrap();
        let curr = list.path().to_str().unwrap().to_string();
        if list.path().is_file() {
            if list.path().to_str().unwrap().contains(item) {
                println!("found in: {}", list.path().to_str().unwrap());
            }
        } else if list.path().is_dir() {
            search(item, curr.as_str());
        }
    });
}

fn main() {
    if env::args().len() != 2 {
        println!("Command use: chronofile <file name/keyword>");
        exit(1);
    }

    let item: String = env::args().nth(1).unwrap().clone();

    search(item.as_str(), "C:/");
}
