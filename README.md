# kernsec
```txt
kernsec 1.0.1
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
$ ./target/release/kernsec
* Kernel config: /proc/config.gz

* Defconfig checks
  GCC stack protector support             Enabled       STACKPROTECTOR
  GCC stack protector strong              Enabled       STACKPROTECTOR_STRONG
  Kernel heap randomization               Enabled       COMPAT_BRK
  Strict kernel RWX                       Enabled       STRICT_KERNEL_RWX
  Strict module RWX                       Enabled       STRICT_MODULE_RWX
  Full reference count validation         Disabled      REFCOUNT_FULL
  Thread info in task                     Enabled       THREAD_INFO_IN_TASK
  IOMMU Hardware Support                  Enabled       IOMMU_SUPPORT
  Randomize position of kernel            Enabled       RANDOMIZE_BASE
  Use a virtually-mapped stack            Enabled       VMAP_STACK
  CPU microcode loading support           Enabled       MICROCODE
  Avoid speculative indirect branches     Disabled      CONFIG_RETPOLINE
  Supervisor Mode Access Prevention       Disabled      X86_SMAP
  TCP syncookie support                   Enabled       SYN_COOKIES
  User Mode Instruction Prevention        Enabled       X86_UMIP

* KSPP checks
  Trigger a BUG on corruption             Disabled      BUG_ON_DATA_CORRUPTION
  Warn on W+X mappings on boot            Enabled       DEBUG_WX
  Detect stack corruption on schedule()   Enabled       SCHED_STACK_END_CHECK
  Harden SLAB freelist metadata           Enabled       SLAB_FREELIST_HARDENED
  Randomize SLAB freelist                 Enabled       SLAB_FREELIST_RANDOM
  Page allocator randomization            Enabled       SHUFFLE_PAGE_ALLOCATOR
  Harden common str/mem functions         Enabled       FORTIFY_SOURCE
  Kernel Electric-Fence                   Enabled       KFENCE
  Hardened usercopy                       Enabled       HARDENED_USERCOPY
  Strict /dev/mem access                  Enabled       STRICT_DEVMEM
  Strict I/O access to /dev/mem           Enabled       IO_STRICT_DEVMEM
```
## Build
Install cargo and [libselinux](https://aur.archlinux.org/packages/libselinux)

```sh
cargo build --release
```
