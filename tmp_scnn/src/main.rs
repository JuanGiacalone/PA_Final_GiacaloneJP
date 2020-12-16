#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(unused_assignments)]

extern crate structopt;
#[macro_use] extern crate text_io;
use structopt::StructOpt;
mod scnn_perf;
mod scnn_sec;
// TP FINAL INTEGRADOR  - PROGRAMACION AVANZADA - UNIVERSIDAD ATLANTIDAD ARGENTINA 2020
// GIACALONE JUAN PABLO 40.666.502

// Escaner de archivos temporales y basura. Diseñado y creado para UNIX
#[structopt(name = "tmp_scnn")]
#[derive(StructOpt)]
struct Cli {

    // Modo de operacion performance(uso de threads) o secuencial.
    #[structopt(long = "fast", short = "f", default_value = "sec")]
    pattern: String,

}
fn main() {

    let opc = Cli::from_args();  ///No logré hacer funcionar CLI
                                ///cargo run --bin scnn_perf  - ejecuta el escaneo con hilos
                                ///cargo run --bin scnn_sec   - ejecuta el escaneo secuencial
    if opc.pattern == "fast"||opc.pattern =="f"
    {
        scnn_perf::perf();
    }else{
        scnn_sec::sec();
    }
        
    
}
