use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

mod kconfig;
mod secureboot;
mod selinux;
mod sysctl;
pub mod utils;

#[derive(Parser)]
#[clap(
    version,
    about = "Check kernel protection mechanisms\nFor more see https://kernsec.org/"
)]
struct Args {
    /// Path to kernel config
    #[clap(long, value_parser, default_value = "/proc/config.gz")]
    config: PathBuf,

    /// sysctl checks
    #[clap(long)]
    sysctl: bool,

    /// SELinux checks
    #[clap(long)]
    selinux: bool,

    /// Secure Boot checks
    #[clap(long)]
    secureboot: bool,
}

fn main() {
    let args = Args::parse();
    println!(
        "{} Kernel config: {}\n",
        "*".yellow().bold(),
        args.config.to_str().unwrap().green().bold()
    );
    crate::kconfig::check(args.config.clone());
    if args.sysctl {
        println!("\n{} sysctl checks\n", "*".yellow().bold());
        crate::sysctl::check();
    }
    if args.selinux {
        println!();
        crate::selinux::check();
    }
    if args.secureboot {
        println!();
        crate::secureboot::check();
    }
}
