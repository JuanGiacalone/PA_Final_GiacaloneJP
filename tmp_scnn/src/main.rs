#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(unused_assignments)]

extern crate structopt;
use structopt::StructOpt;
mod scnn_perf;
mod scnn_sec;

// Escaner de archivos temporales y basura. Diseñado y creado para UNIX
#[structopt(name = "tmp_scnn")]
#[derive(StructOpt)]
struct Cli {

    // Modo de operacion performance(uso de threads) o secuencial.
    #[structopt(long = "fast", short = "f", default_value = "sec")]
    pattern: String,


}
fn main() {

    let opc = Cli::from_args();

    if opc.pattern == "fast"||opc.pattern =="f"
    {
        scnn_perf::perf();
    }else{
        scnn_sec::sec();
    }
        
    
}
