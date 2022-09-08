use crate::utils::*;
use selinux::{current_mode, SELinuxMode, protection_checking_mode, undefined_handling};

pub fn check() {
    echoy!(
        "SELinux",
        match current_mode() {
            SELinuxMode::NotRunning => "Disabled".red().bold(),
            SELinuxMode::Permissive => "Permissive".yellow().bold(),
            _ => "Enforcing".green().bold(),
        }
    );
    if current_mode() == SELinuxMode::NotRunning {
        return;
    }
    echo!(
        "Checkreqprot",
        match protection_checking_mode() {
            Ok(_) => "Enabled".green().bold(),
            Err(_) => "Disabled".red().bold(),
        }
    );
    echo!(
        "Deny unknown",
        match undefined_handling() {
            Ok(_) => "Enabled".green().bold(),
            Err(_) => "Disabled".red().bold(),
        }
    );
}
