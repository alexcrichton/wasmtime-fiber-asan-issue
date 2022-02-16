
```
$ RUSTFLAGS='-Zsanitizer=address' cargo +nightly run --target aarch64-unknown-linux-gnu --release
    Finished release [optimized + debuginfo] target(s) in 0.01s
     Running `target/aarch64-unknown-linux-gnu/release/wasmtiem-fiber-asan-issue`
allocated a stack at 0xffff91400000-0xffff91600000
starting fiber
		in-fiber 0xffff915ffe00
fiber exited, destroying fiber
releasing fiber stack
got previous fiber stack as a fresh new mmap allocation
=================================================================
==29724==ERROR: AddressSanitizer: stack-buffer-underflow on address 0xffff915ffbe0 at pc 0xaaaad94974d8 bp 0xffffdc9cfee0 sp 0xffffdc9cfef8
WRITE of size 1 at 0xffff915ffbe0 thread T0
    #0 0xaaaad94974d4 in core::ptr::write_volatile::h47b425cf7cc437f0 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/core/src/ptr/mod.rs:1128:9
    #1 0xaaaad94974d4 in wasmtiem_fiber_asan_issue::main::h08ae4e1365b27113 /home/acrichto/code/wasmtime-fiber-asan-issue/src/main.rs:69:13
    #2 0xaaaad9495ba0 in core::ops::function::FnOnce::call_once::h693e6d092d4ef68a /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/core/src/ops/function.rs:227:5
    #3 0xaaaad9495ba0 in std::sys_common::backtrace::__rust_begin_short_backtrace::h2481c7128b3300b5 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/sys_common/backtrace.rs:122:18
    #4 0xaaaad9497e7c in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd8f972309abaffdf /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/rt.rs:145:18
    #5 0xaaaad94aa5c0 in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h738af5d88ca9c227 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/core/src/ops/function.rs:259:13
    #6 0xaaaad94aa5c0 in std::panicking::try::do_call::h07925d8ec3014f5a /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/panicking.rs:492:40
    #7 0xaaaad94aa5c0 in std::panicking::try::h51eacf1087c31062 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/panicking.rs:456:19
    #8 0xaaaad94aa5c0 in std::panic::catch_unwind::h9edf57fc8780b4b9 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/panic.rs:137:14
    #9 0xaaaad94aa5c0 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h79a479d323b84507 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/rt.rs:128:48
    #10 0xaaaad94aa5c0 in std::panicking::try::do_call::hdeaff87196c43426 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/panicking.rs:492:40
    #11 0xaaaad94aa5c0 in std::panicking::try::h0fe596461dab61b0 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/panicking.rs:456:19
    #12 0xaaaad94aa5c0 in std::panic::catch_unwind::h725f36d8999f5bf4 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/panic.rs:137:14
    #13 0xaaaad94aa5c0 in std::rt::lang_start_internal::hf35cbac5ad16b355 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/rt.rs:128:20
    #14 0xaaaad9497df8 in std::rt::lang_start::hf0ba83e76ae012c9 /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/std/src/rt.rs:144:17
    #15 0xaaaad94978d4 in main (/home/acrichto/code/wasmtime-fiber-asan-issue/target/aarch64-unknown-linux-gnu/release/wasmtiem-fiber-asan-issue+0xbb8d4)
    #16 0xffff943f3d20 in __libc_start_main /build/glibc-pfQmxL/glibc-2.28/csu/../csu/libc-start.c:308:16
    #17 0xaaaad93f2610 in _start (/home/acrichto/code/wasmtime-fiber-asan-issue/target/aarch64-unknown-linux-gnu/release/wasmtiem-fiber-asan-issue+0x16610)

Address 0xffff915ffbe0 is a wild pointer inside of access range of size 0x000000000001.
SUMMARY: AddressSanitizer: stack-buffer-underflow /rustc/09cb29c64c2a0e15debf2d6fca2bc7c71a682033/library/core/src/ptr/mod.rs:1128:9 in core::ptr::write_volatile::h47b425cf7cc437f0
Shadow bytes around the buggy address:
  0x200ff22bff20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x200ff22bff30: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x200ff22bff40: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x200ff22bff50: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x200ff22bff60: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
=>0x200ff22bff70: 00 00 00 00 00 00 00 00 00 00 00 00[f1]f1 f1 f1
  0x200ff22bff80: f8 f8 f8 f2 f2 f2 f2 f2 f8 f8 f8 f2 f2 f2 f2 f2
  0x200ff22bff90: f8 f8 f8 f2 f2 f2 f2 f2 f8 f8 f8 f3 f3 f3 f3 f3
  0x200ff22bffa0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x200ff22bffb0: 00 00 00 00 f1 f1 f1 f1 00 00 00 f2 f2 f2 f2 f2
  0x200ff22bffc0: 00 f3 f3 f3 00 00 00 00 00 00 00 00 00 00 00 00
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==29724==ABORTING
```
