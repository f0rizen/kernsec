use crate::utils::*;

pub fn check() {
    let tainted_ctl = get_ctl("kernel.tainted").unwrap();
    echoy!(
        "Tainted kernel",
        if tainted_ctl == 0 {
            "No".green().bold()
        } else {
            "Yes".red().bold()
        }
    );
    if tainted_ctl == 0 {
        return;
    }

    let reasons = vec![
        "Proprietary module was loaded",
        "Module was force loaded",
        "Kernel running on an out of specification system",
        "Module was force unloaded",
        "Processor reported a Machine Check Exception (MCE)",
        "Bad page referenced or some unexpected page flags",
        "Taint requested by userspace application",
        "Kernel died recently, i.e. there was an OOPS or BUG",
        "ACPI table overridden by user",
        "Kernel issued warning",
        "Staging driver was loaded",
        "Workaround for bug in platform firmware applied",
        "Externally-built (\"out-of-tree\") module was loaded",
        "Unsigned module was loaded",
        "Soft lockup occurred",
        "Kernel has been live patched",
        "Auxiliary taint, defined for and used by distros",
        "Kernel was built with the struct randomization plugin",
        "An in-kernel test has been run",
    ];
    for i in 0..=18 {
        if (tainted_ctl >> i) & 1 == 1 {
            echos!(format!("{} (#{})", reasons[i], i));
        }
    }
}
