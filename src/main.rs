use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

mod kconfig;
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
}

fn main() {
    let args = Args::parse();
    println!(
        "{} Kernel config: {}\n",
        "*".yellow().bold(),
        args.config.to_str().unwrap().green().bold()
    );
    crate::kconfig::check(args.config);
    if args.sysctl {
        println!("\n{} sysctl checks\n", "*".yellow().bold());
        crate::sysctl::check();
    }
}
