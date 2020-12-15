
use std::sync::{Arc};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::process::Command;
use std::time::Duration;
use std::path::PathBuf;
use pbr::{ProgressBar, Units};
use gifski::*;
use gifski::progress::ProgressReporter;
const FRAMES: usize = 96;

fn main() {

   let ffmpeg_png = Command::new("ffmpeg")
                .arg("-i")
                .arg("/home/juan/Downloads/vid/vid.mp4")
                .arg("/home/juan/Downloads/vid/frame%d.png")
                .spawn()
                .expect("fallo en la carga del video");

    let count = 1000;
    let mut pb = ProgressBar::new(count);
    let escritor;
    {
        let mut gifski = gifski::new(Settings::default()).unwrap();
        escritor = gifski.1;

        for i in 0..FRAMES
        {
            gifski.0.add_frame_png_file(i, "/home/juan/Downloads/vid/frame"i".png", 0.).unwrap();
            println!("Added frame {}", i);
        }
    }
    
    escritor
    .write(std::fs::File::create("dead_img_new.gif").unwrap(), &mut pb)
    .unwrap();
        
       
        
}