# Rust `PartialEq` silent correctness bug on `armebv7r-none-eabi` (big-endian ARM)

Minimal reproducer for a silent correctness bug in the Rust compiler where derived
`PartialEq` on enums returns incorrect results on big-endian ARM targets when built
at `opt-level=0` with `-Z build-std`.

**Affected target:** `armebv7r-none-eabi`  
**Trigger:** `opt-level=0` (the default for `cargo build` / `cargo run`) + `-Z build-std=core`  
**Symptom:** `a == b` returns `true`. No panic, no warning — silent wrong behavior.  
**Workaround:** `opt-level >= 1`, or replace `==` with a `match` expression.

---

## The bug

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MyEnum { A, B, C, D }

let a = MyEnum::A;

// BUG: returns true at opt-level=0 on armebv7r-none-eabi
if a == MyEnum::B {
    // reached
}

// OK: match always works
match a {
    MyEnum::A => { /* reached correctly */ }
    MyEnum::B => { /* not reached correctly */ }
    _ => {}
}
```

Note that `#[repr(u8)]` does **not** mitigate the bug — a single-byte discriminant
still triggers it.

---

## Environment

- **Target:** `armebv7r-none-eabi` (big-endian, `no_std`, bare-metal)
- **Runner:** QEMU `virt` machine, `cortex-a15` CPU, semihosting
- **Build flag:** `-Z build-std=core` (nightly required)
- **Rust channel:** nightly

---

## Prerequisites

- Rust nightly toolchain
- `rust-src` component: `rustup component add rust-src --toolchain nightly`
- QEMU with ARM support: `qemu-system-arm`
- `rust-lld` linker (ships with the nightly toolchain)

---

## Reproducing the bug

Clone the repo and run with the default `opt-level=0`:

```sh
cargo +nightly run
```

Expected (correct) exit code: **1**  
Actual (buggy) exit code: **0**

The program reaches `semihosting_exit(0)` — even though
`a == MyEnum::B` should be `false`.

---

## Confirming the workaround

Set `opt-level = 1` in `Cargo.toml` (already included as a commented-out profile),
then run again:

```sh
cargo +nightly run
```

Exit code is now **0** — the bug does not trigger.

Alternatively, replace the `==` comparison with a `match` expression (see
`src/main.rs`) and run at any opt-level.

---

## Project structure

```
.
├── .cargo/
│   └── config.toml       # target runner (QEMU), linker, rustflags
├── src/
│   └── main.rs           # reproducer: enum PartialEq vs match
├── link.x                # linker script for the virt machine
└── Cargo.toml
```
