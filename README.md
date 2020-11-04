# Mult Panic

This is a minimal reproducer for a bug where the compiler inserts a call to a non-existent
panic function. The following things need to be true to reproduce the bug:

* target must be wasm32-unknown-unknown
* overflow checks must be enabled
* link time optimization must be enabled
* core must be recompiled with these flags using cargo xbuild or the std aware cargo
* A compiler extrinsic must be able to trigger a panic

## Steps to reproduce

I tested these steps with `rustc 1.49.0-nightly (ffa2e7ae8 2020-10-24)`:

```shell
cargo +nightly build --release --target wasm32-unknown-unknown --no-default-features -Z build-std -Z build-std-features=panic_immediate_abort
wasm2wat target/wasm32-unknown-unknown/release/mult_panic.wasm | less
```

If you inspect the resulting wasm file you see that it imports some none existing panic function:

```wat
(import "env" "_ZN4core9panicking5panic17h80a3410ec4f43255E" (func $_ZN4core9panicking5panic17h80a3410ec4f43255E (type 0)))
```

This function is called from the compiler intrinsic on integer overflow. If either lto or
integer overflow is disabled, the error goes away.
