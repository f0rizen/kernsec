use crate::utils::*;
use std::str::FromStr;

use efivar::{efi::VariableName, VarManager};

const GUID_EFI_GLOBAL: &str = "8be4df61-93ca-11d2-aa0d-00e098032b8c";

fn efivar_read(manager: Box<dyn VarManager>, name: &str) -> (usize, Vec<u8>) {
    let mut buf = vec![0u8; 512];
    let name = VariableName::from_str(name).unwrap();

    match manager.read(&name, &mut buf[..]) {
        Ok((size, _)) => (size, buf),
        Err(error) => panic!("{}", error),
    }
}

pub fn check() {
    let secureboot_var = efivar_read(
        efivar::system(),
        format!("SecureBoot-{}", GUID_EFI_GLOBAL).as_str(),
    );
    if secureboot_var.0 != 1 {
        echo!(format!(
            "Strange data size {} for {} variable",
            secureboot_var.0.to_string().yellow().bold(),
            "SecureBoot".blue().bold()
        ));
    }
    let secureboot = secureboot_var.1[0];

    let setupmode_var = efivar_read(
        efivar::system(),
        format!("SetupMode-{}", GUID_EFI_GLOBAL).as_str(),
    );
    if setupmode_var.0 != 1 {
        echo!(format!(
            "Strange data size {} for {} variable",
            setupmode_var.0.to_string().yellow().bold(),
            "SetupMode".blue().bold()
        ));
    }
    let setupmode = setupmode_var.1[0];

    echoy!(
        "Secure Boot",
        if secureboot == 1 {
            "Enabled".green().bold()
        } else {
            "Disabled".red().bold()
        }
    );
    if setupmode == 1 {
        echo!("Platform is in {}", "Setup Mode".yellow().bold());
    }
}
