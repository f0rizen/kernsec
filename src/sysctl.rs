use crate::utils::*;
use sysctl::{Ctl, Sysctl, SysctlError};

fn get_ctl(name: &str) -> Result<i32, SysctlError> {
    let ctl = Ctl::new(name)?;
    let val = ctl.value_string()?.parse().unwrap();
    return Ok(val);
}

pub fn check() {
    let aslr = get_ctl("kernel.randomize_va_space").unwrap();
    println!(
        "{:SIZE1$}{:SIZE2$}{}",
        "ASLR:",
        if aslr == 2 {
            "Full".green().bold()
        } else if aslr == 1 {
            "Partial".yellow().bold()
        } else {
            "None".red().bold()
        },
        "kernel.randomize_va_space".blue().bold(),
    );
    let yama = get_ctl("kernel.yama.ptrace_scope");
    println!(
        "{:SIZE1$}{:SIZE2$}{}",
        "YAMA:",
        match yama {
            Ok(k) => {
                if k == 1 {
                    "Active".green().bold()
                } else {
                    "Inactive".yellow().bold()
                }
            }
            Err(_) => "Disabled".red().bold(),
        },
        "kernel.yama.ptrace_scope".blue().bold()
    );
    let exec_shield = get_ctl("kernel.exec-shield");
    println!(
        "{:SIZE1$}{:SIZE2$}{}",
        "Exec shield:",
        match exec_shield {
            Ok(k) => {
                if k == 2 {
                    "Enabled".green().bold()
                } else if k == 1 {
                    "Partial".yellow().bold()
                } else {
                    "Disabled".red().bold()
                }
            }
            Err(_) => "Unsupported".blue().bold(),
        },
        "kernel.exec-shield".blue().bold()
    );

    let links = vec![
        ("Protected symlinks:", "fs.protected_symlinks"),
        ("Protected hardlinks:", "fs.protected_hardlinks"),
    ];
    for (msg, ctl) in links {
        let ans = get_ctl(ctl).unwrap();
        println!(
            "{:SIZE1$}{:SIZE2$}{}",
            msg,
            if ans == 1 {
                "Enabled".green().bold()
            } else {
                "Disabled".red().bold()
            },
            ctl.blue().bold(),
        );
    }

    let files = vec![
        ("Protected fifos:", "fs.protected_fifos"),
        ("Protected regular:", "fs.protected_regular"),
    ];
    for (msg, ctl) in files {
        let ans = get_ctl(ctl);
        println!(
            "{:SIZE1$}{:SIZE2$}{}",
            msg,
            match ans {
                Ok(k) => {
                    if k == 2 {
                        "Enabled".green().bold()
                    } else if k == 1 {
                        "Partial".yellow().bold()
                    } else {
                        "Disabled".red().bold()
                    }
                }
                Err(_) => "Unsupported".blue().bold(),
            },
            ctl.blue().bold()
        );
    }
}