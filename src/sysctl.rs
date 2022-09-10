use crate::utils::*;

pub fn check() {
    let aslr = get_ctl("kernel.randomize_va_space").unwrap();
    echo!(
        "ASLR",
        if aslr == 2 {
            "Full".green().bold()
        } else if aslr == 1 {
            "Partial".yellow().bold()
        } else {
            "None".red().bold()
        },
        "kernel.randomize_va_space".blue().bold()
    );
    let yama = get_ctl("kernel.yama.ptrace_scope");
    echo!(
        "YAMA",
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
    echo!(
        "Exec shield",
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
        ("Protected symlinks", "fs.protected_symlinks"),
        ("Protected hardlinks", "fs.protected_hardlinks"),
    ];
    for (msg, ctl) in links {
        let ans = get_ctl(ctl).unwrap();
        echo!(
            msg,
            if ans == 1 {
                "Enabled".green().bold()
            } else {
                "Disabled".red().bold()
            },
            ctl.blue().bold()
        );
    }

    let files = vec![
        ("Protected fifos", "fs.protected_fifos"),
        ("Protected regular", "fs.protected_regular"),
    ];
    for (msg, ctl) in files {
        let ans = get_ctl(ctl);
        echo!(
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
