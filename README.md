# kernsec
```
kernsec 0.2.2
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
```
$ ./target/release/kernsec --sysctl
* Kernel config: /proc/config.gz

  GCC stack protector support             Enabled       CONFIG_STACKPROTECTOR
  GCC stack protector strong              Enabled       CONFIG_STACKPROTECTOR_STRONG
  Kernel heap randomization               Enabled       CONFIG_COMPAT_BRK
  Strict /dev/mem access                  Enabled       CONFIG_STRICT_DEVMEM
  Strict I/O access to /dev/mem           Enabled       CONFIG_IO_STRICT_DEVMEM
  Randomize SLAB freelist                 Enabled       CONFIG_SLAB_FREELIST_RANDOM
  Use a virtually-mapped stack            Enabled       CONFIG_VMAP_STACK
  Full reference count validation         Disabled      CONFIG_REFCOUNT_FULL
  Hardened usercopy                       Enabled       CONFIG_HARDENED_USERCOPY
  Harden common str/mem functions         Enabled       CONFIG_FORTIFY_SOURCE
  Randomize position of kernel            Enabled       CONFIG_RANDOMIZE_BASE
  Randomize position of memory            Enabled       CONFIG_RANDOMIZE_MEMORY

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

```
$ cargo build --release
```
