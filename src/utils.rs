pub use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;
pub use std::path::PathBuf;

pub static TAB:   usize =  2;
pub static SIZE1: usize = 40;
pub static SIZE2: usize = 14;

fn decode_gz(file: File) -> String {
    use flate2::read::GzDecoder;
    let mut decoder = GzDecoder::new(file);
    let mut ans = String::new();
    decoder.read_to_string(&mut ans).unwrap();
    return ans;
}

pub fn read_config(path: PathBuf) -> String {
    let mut file = File::open(&path).unwrap();
    let mut text = String::new();
    if path.extension().unwrap() == "gz" {
        text = decode_gz(file);
    } else {
        file.read_to_string(&mut text).unwrap();
    }
    return text;
}

macro_rules! echo {
    ($msg:expr, $ans:expr, $line:expr) => {
        println!("{:TAB$}{:SIZE1$}{:SIZE2$}{}", "", $msg, $ans, $line);
    };
}

pub(crate) use echo;
