use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(
    version,
    about = "Check kernel protection mechanisms\nFor more see https://kernsec.org/"
)]
pub struct Args {
    /// Path to kernel config
    #[clap(long, value_parser, default_value = "/proc/config.gz")]
    pub config: PathBuf,

    /// sysctl checks
    #[clap(long)]
    pub sysctl: bool,

    /// SELinux checks
    #[clap(long)]
    pub selinux: bool,

    /// Tainted kernel checks
    #[clap(long)]
    pub tainted: bool,

    /// Secure Boot checks
    #[clap(long)]
    pub secureboot: bool,
}
