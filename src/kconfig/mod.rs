use crate::utils::*;

mod utils;
use crate::kconfig::utils::*;

fn check_stackprotector(config: &str, version: &Vec<i32>) {
    let stackprotector = if *version < vec![4, 18] {
        "CC_STACKPROTECTOR"
    } else {
        "STACKPROTECTOR"
    };
    let stackprotector_strong = if *version < vec![4, 18] {
        "CC_STACKPROTECTOR_STRONG"
    } else {
        "STACKPROTECTOR_STRONG"
    };

    let spf = contains(config, stackprotector);
    let spf_strong = contains(config, stackprotector_strong);
    echo!(
        "GCC stack protector support",
        match spf {
            true => "Enabled".green().bold(),
            false => "Disabled".red().bold(),
        },
        stackprotector.blue().bold()
    );
    if spf {
        echo!(
            "GCC stack protector strong",
            match spf_strong {
                true => "Enabled".green().bold(),
                false => "Disabled".red().bold(),
            },
            stackprotector_strong.blue().bold()
        );
    }
}

pub fn check(path: PathBuf) {
    let config = read_config(path);
    let ver = match kernel_version(&config) {
        Ok(v) => v,
        Err(e) => {
            echo!(e.on_red().bold());
            std::process::exit(1);
        }
    };
    let chk_config = |lines: Vec<(&str, &str)>| {
        for (msg, line) in lines {
            echo!(
                msg,
                match contains(&config, line) {
                    true => "Enabled".green().bold(),
                    false => "Disabled".red().bold(),
                },
                line.blue().bold()
            );
        }
    };

    echoy!("Defconfig checks");

    check_stackprotector(&config, &ver);
    echo!(
        "Kernel heap randomization",
        match !contains(&config, "COMPAT_BRK") {
            true => "Enabled".green().bold(),
            false => "Disabled".red().bold(),
        },
        "COMPAT_BRK".blue().bold()
    );
    let defconfig = vec![
        ("Strict kernel RWX", "STRICT_KERNEL_RWX"),
        ("Strict module RWX", "STRICT_MODULE_RWX"),
        ("Full reference count validation", "REFCOUNT_FULL"),
        ("Thread info in task", "THREAD_INFO_IN_TASK"),
        ("IOMMU Hardware Support", "IOMMU_SUPPORT"),
        ("Randomize position of kernel", "RANDOMIZE_BASE"),
        ("Use a virtually-mapped stack", "VMAP_STACK"),
        ("CPU microcode loading support", "MICROCODE"),
        ("Avoid speculative indirect branches", "CONFIG_RETPOLINE"),
        ("Supervisor Mode Access Prevention", "X86_SMAP"),
        ("TCP syncookie support", "SYN_COOKIES"),
        ("User Mode Instruction Prevention", "X86_UMIP"),
    ];
    let kspp = vec![
        ("Trigger a BUG on corruption", "BUG_ON_DATA_CORRUPTION"),
        ("Warn on W+X mappings on boot", "DEBUG_WX"),
        (
            "Detect stack corruption on schedule()",
            "SCHED_STACK_END_CHECK",
        ),
        ("Harden SLAB freelist metadata", "SLAB_FREELIST_HARDENED"),
        ("Randomize SLAB freelist", "SLAB_FREELIST_RANDOM"),
        ("Page allocator randomization", "SHUFFLE_PAGE_ALLOCATOR"),
        ("Harden common str/mem functions", "FORTIFY_SOURCE"),
        ("Kernel Electric-Fence", "KFENCE"),
        ("Hardened usercopy", "HARDENED_USERCOPY"),
        ("Strict /dev/mem access", "STRICT_DEVMEM"),
        ("Strict I/O access to /dev/mem", "IO_STRICT_DEVMEM"),
    ];
    chk_config(defconfig);

    println!();
    echoy!("KSPP checks");
    chk_config(kspp);
}
