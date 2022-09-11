pub use colored::Colorize;
pub use std::path::PathBuf;

use std::fs::File;
use std::io::Read;

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

pub fn get_ctl(name: &str) -> Result<i32, sysctl::SysctlError> {
    use sysctl::{Ctl, Sysctl};
    let ctl = Ctl::new(name)?;
    let val = ctl.value_string()?.parse().unwrap();
    return Ok(val);
}

#[allow(non_upper_case_globals)]
mod constants {
    pub const tab:    usize =  2;
    pub const width1: usize = 40;
    pub const width2: usize = 14;
}
pub use constants::*;

macro_rules! echo {
    ($msg:expr) => {
        println!("{:tab$}{}", "", $msg);
    };
    ($msg:expr, $ans:expr) => {
        println!("{:tab$}{:width1$}{}", "", $msg, $ans);
    };
    ($msg:expr, $ans:expr, $line:expr) => {
        println!("{:tab$}{:width1$}{:width2$}{}", "", $msg, $ans, $line);
    };
}

macro_rules! echoy {
    ($msg:expr) => {
        println!("{:tab$} {} {}", "", "*".yellow().bold(), $msg)
    };
    ($msg:expr, $ans:expr) => {
        println!("{} {:width1$}{}", "*".yellow().bold(), $msg, $ans);
    };
}

pub(crate) use echo;
pub(crate) use echoy;
