use bytesize::ByteSize;
use env_logger::Env;
use groch_png::tools::*;
use groch_png::Png;
use log::{debug, error, info, trace, warn};
use std::env;
use std::fs;

fn main() {
    //env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    for filename in env::args().skip(1) {
        trace!("Opening file {filename:?}");
        let file = fs::read(&filename).log_unwrap();
        info!("{filename:?} have {}.", ByteSize(file.len() as u64));
        let mut png = Png::from(file.as_slice());
        for chunk in png {}
        //debug!("d as");
        //error!("e as");
        //info!("i as");
        //trace!("t as");
        //warn!("w as");
    }

    println!("asd");
}
