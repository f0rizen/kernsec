use crate::utils::*;

fn kernel_version(config: &String) {
    use regex::Regex;
    let re = Regex::new(r"# Linux/.* Kernel Configuration").unwrap();
    if !re.is_match(config) {
        echo!("Failed to detect kernel version".on_red().bold());
        return;
    }
    let v = re.captures(config).unwrap().get(0).unwrap();
    let matches: Vec<&str> = v.as_str().split(' ').collect();
    let version: Vec<&str> = matches[2].split('-').collect();

    let splitted: Vec<&str> = version[0].split('.').collect();
    if splitted.len() < 3
        || splitted[0].parse::<i32>().is_err()
        || splitted[1].parse::<i32>().is_err()
    {
        echo!("Failed to parse kernel version".on_red().bold());
        return;
    }
    echoy!("Kernel version", version[0].blue().bold());
}

fn check_stackprotector(config: &String) {
    let stackprotector = "CONFIG_STACKPROTECTOR";
    let cc_stackprotector = "CONFIG_CC_STACKPROTECTOR";
    let mut line = "";
    if config.contains(format!("{}=y", stackprotector).as_str()) {
        line = stackprotector;
    } else if config.contains(format!("{}=y", cc_stackprotector).as_str()) {
        line = cc_stackprotector;
    }
    echo!(
        "GCC stack protector support",
        if line != "" {
            "Enabled".green().bold()
        } else {
            "Disabled".red().bold()
        },
        if line == "" {
            stackprotector.blue().bold()
        } else {
            line.blue().bold()
        }
    );
    if line == "" {
        return;
    }
    let stackprotector_strong = "CONFIG_STACKPROTECTOR_STRONG";
    let cc_stackprotector_strong = "CONFIG_CC_STACKPROTECTOR_STRONG";
    line = "";
    if config.contains(format!("{}=y", stackprotector_strong).as_str()) {
        line = stackprotector_strong;
    } else if config.contains(format!("{}=y", cc_stackprotector_strong).as_str()) {
        line = cc_stackprotector_strong;
    }
    echo!(
        "GCC stack protector strong",
        if line != "" {
            "Enabled".green().bold()
        } else {
            "Disabled".red().bold()
        },
        if line == "" {
            stackprotector.blue().bold()
        } else {
            line.blue().bold()
        }
    );
}

pub fn check(path: PathBuf) {
    let config = read_config(path);
    kernel_version(&config);

    check_stackprotector(&config);
    echo!(
        "Kernel heap randomization",
        if !config.contains("CONFIG_COMPAT_BRK=y") {
            "Enabled".green().bold()
        } else {
            "Disabled".red().bold()
        },
        "CONFIG_COMPAT_BRK".blue().bold()
    );

    let configs = vec![
        ("Strict /dev/mem access", "CONFIG_STRICT_DEVMEM"),
        ("Strict I/O access to /dev/mem", "CONFIG_IO_STRICT_DEVMEM"),
        ("Randomize SLAB freelist", "CONFIG_SLAB_FREELIST_RANDOM"),
        ("Use a virtually-mapped stack", "CONFIG_VMAP_STACK"),
        ("Full reference count validation", "CONFIG_REFCOUNT_FULL"),
        ("Hardened usercopy", "CONFIG_HARDENED_USERCOPY"),
        ("Harden common str/mem functions", "CONFIG_FORTIFY_SOURCE"),
        ("Randomize position of kernel", "CONFIG_RANDOMIZE_BASE"),
        ("Randomize position of memory", "CONFIG_RANDOMIZE_MEMORY"),
    ];
    for (msg, line) in configs {
        echo!(
            msg,
            if config.contains(format!("{}=y", line).as_str()) {
                "Enabled".green().bold()
            } else {
                "Disabled".red().bold()
            },
            line.blue().bold()
        );
    }
}
