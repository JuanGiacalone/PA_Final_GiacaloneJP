#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(unused_assignments)]

extern crate walkdir;
use std::env;
use std::time::{Duration, Instant};
use dirs::home_dir;
use walkdir::WalkDir;

pub fn sec() {

 
    let start = Instant::now();

    let _home = home_dir();
    let homee = env::var("HOME").unwrap_or("none".to_string());
    println!("{}",homee);

        ///Or, if you'd like to iterate over all entries and ignore any errors that may arise, 
        //use filter_map. (e.g., This code below will silently skip directories that the owner 
        //of the running process does not have permission to access.)
    for e in WalkDir::new("/tmp/").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            println!("{}", e.path().display());
        }
    }
    for x in WalkDir::new("/var/tmp/").into_iter().filter_map(|x| x.ok()) {
        if x.metadata().unwrap().is_file() {
            println!("{}", x.path().display());
        }
    }
    
    for y in WalkDir::new("{%homee}/.local/share/Trash/").into_iter().filter_map(|y| y.ok()) {
        if y.metadata().unwrap().is_file() {
            println!("{}", y.path().display());
        }
    }
    println!("Escaneo finalizado");
    
    println!("Tiempo total: {:#?}", start.elapsed());
    
}
