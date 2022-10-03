pub mod utils;

mod kconfig;
mod secureboot;
mod selinux;
mod sysctl;
mod tainted;

mod options;

fn main() {
    use colored::Colorize;

    let args: options::Args = clap::Parser::parse();
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
    if args.selinux {
        println!();
        crate::selinux::check();
    }
    if args.tainted {
        println!();
        crate::tainted::check();
    }
    if args.secureboot {
        println!();
        crate::secureboot::check();
    }
}
