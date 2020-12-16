#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(unused_assignments)]

extern crate walkdir;
use std::thread;
use std::time::{Duration, Instant};
use std::env;
use dirs::home_dir;
use walkdir::{WalkDir};


pub fn perf() {

    let start = Instant::now();

    {
    let mut manejador = thread::spawn(||{

        for e in WalkDir::new("/tmp/").into_iter().filter_map(|e| e.ok()) {
            if e.metadata().unwrap().is_file() {
                println!("{}", e.path().display());
            }
        }
    });
    manejador = thread::spawn(||{

        for x in WalkDir::new("/var/tmp/").into_iter().filter_map(|x| x.ok()) {
            if x.metadata().unwrap().is_file() {
                println!("{}", x.path().display());
            }
        }
    });
    
    let _home = home_dir();
    let homee = env::var("HOME").unwrap_or("none".to_string());

    println!("{}",homee);
    

    for y in WalkDir::new("{%homee}/.local/share/Trash/").into_iter().filter_map(|y| y.ok()) {
        if y.metadata().unwrap().is_file() {
            println!("{}", y.path().display());
        }
    }

    manejador.join().unwrap();
    }
    println!("Escaneo finalizado(Usando Hilos)");
    println!("Tiempo total: {:#?}", start.elapsed())
    
}
fn main()
{
    perf();
}