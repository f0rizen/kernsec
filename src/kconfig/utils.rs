pub fn contains(config: &String, chk: &str) -> bool {
    return config.contains(format!("CONFIG_{}=y", chk).as_str());
}

pub fn kernel_version(config: &String) -> Result<Vec<i32>, &str> {
    use regex::Regex;
    let re = Regex::new(r"# Linux/.* Kernel Configuration").unwrap();
    if !re.is_match(config) {
        return Err("Failed to detect kernel version");
    }
    let v = re.captures(config).unwrap().get(0).unwrap();
    let matches: Vec<&str> = v.as_str().split(' ').collect();
    let version: Vec<&str> = matches[2].split('-').collect();

    let splitted: Vec<&str> = version[0].split('.').collect();
    if splitted.len() < 3 {
        return Err("Kernel version does not match x.y.z");
    }
    fn not_num(x: &str) -> bool {
        return x.parse::<i32>().is_err();
    }
    let mut ans: Vec<i32> = Vec::new();
    for i in splitted {
        if not_num(i) {
            return Err("Failed to parse kernel version");
        }
        ans.push(i.parse().unwrap());
    }
    return Ok(ans);
}

#[cfg(test)]
mod kversion {
    use super::kernel_version;

    #[test]
    fn simple() {
        let test = String::from("# Linux/x86 5.15.0 Kernel Configuration");
        assert_eq!(kernel_version(&test), Ok(vec![5, 15, 0]));
    }

    #[test]
    fn build() {
        let test = String::from("# Linux/x86 5.15.0-arch1 Kernel Configuration");
        assert_eq!(kernel_version(&test), Ok(vec![5, 15, 0]));
    }

    #[test]
    fn failed_detect() {
        let test = String::from("abcdef");
        assert_eq!(
            kernel_version(&test),
            Err("Failed to detect kernel version")
        );
    }

    #[test]
    fn not_match() {
        let test = String::from("# Linux/x86 6.0 Kernel Configuration"); // valid is 6.0.0
        assert_eq!(
            kernel_version(&test),
            Err("Kernel version does not match x.y.z")
        );
    }

    #[test]
    fn failed_parse() {
        let test = String::from("# Linux/x86 abc.abc.abc Kernel Configuration");
        assert_eq!(kernel_version(&test), Err("Failed to parse kernel version"));
    }
}
