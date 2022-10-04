# kernsec
```txt
kernsec 1.0.0
Check kernel protection mechanisms
For more see https://kernsec.org/

USAGE:
    kernsec [OPTIONS]

OPTIONS:
        --config <CONFIG>    Path to kernel config [default: /proc/config.gz]
    -h, --help               Print help information
        --secureboot         Secure Boot checks
        --selinux            SELinux checks
        --sysctl             sysctl checks
        --tainted            Tainted kernel checks
    -V, --version            Print version information
```
## Example output
```txt
$ ./target/release/kernsec --sysctl
* Kernel config: /proc/config.gz

* Kernel version                          5.19.12
  GCC stack protector support             Enabled       STACKPROTECTOR
  GCC stack protector strong              Enabled       STACKPROTECTOR_STRONG
  Kernel heap randomization               Enabled       COMPAT_BRK
  Strict /dev/mem access                  Enabled       STRICT_DEVMEM
  Strict I/O access to /dev/mem           Enabled       IO_STRICT_DEVMEM
  Randomize SLAB freelist                 Enabled       SLAB_FREELIST_RANDOM
  Use a virtually-mapped stack            Enabled       VMAP_STACK
  Full reference count validation         Disabled      REFCOUNT_FULL
  Hardened usercopy                       Enabled       HARDENED_USERCOPY
  Harden common str/mem functions         Enabled       FORTIFY_SOURCE
  Randomize position of kernel            Enabled       RANDOMIZE_BASE
  Randomize position of memory            Enabled       RANDOMIZE_MEMORY

* sysctl checks

  ASLR                                    Full          kernel.randomize_va_space
  YAMA                                    Active        kernel.yama.ptrace_scope
  Exec shield                             Unsupported   kernel.exec-shield
  Protected symlinks                      Enabled       fs.protected_symlinks
  Protected hardlinks                     Enabled       fs.protected_hardlinks
  Protected fifos                         Partial       fs.protected_fifos
  Protected regular                       Partial       fs.protected_regular
```
## Build
Install cargo and [libselinux](https://aur.archlinux.org/packages/libselinux)

```sh
cargo build --release
```
