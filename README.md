# Template-rust workspace

## Project with complete structure
1. Create a workspace hands-on
```bash
mkdir hands-on
cd hands-on
```

2. About the target <b>host<b>
```bash
rustc -vV
>> rustc 1.86.0 (05f9846f8 2025-03-31)
>> binary: rustc
>> commit-hash: 05f9846f893b09a1be1fc8560e33fc3c815cfecb
>> commit-date: 2025-03-31
>> host: x86_64-unknown-linux-gnu
>> release: 1.86.0
>> LLVM version: 19.1.7
```

3. Add file with parameters of build .cargo/config.toml
```toml
[build]
#target = "x86_64-unknown-linux-gnu"
rustflags = ["-C", "link-dead-code"]

# ref: https://github.com/johnthagen/min-sized-rust
[profile.release]
opt-level = "z"   # Optimize for size.
strip = true      # Automatically strip symbols from the binary.
lto = true        # Link Time Optimization
codegen-units = 1
panic = "abort"
```

4. Add workspace base file (./Cargo.toml)
```toml
[workspace]
members = []
resolver = "1"
```

1. Add file gitignore: (.gitignore)
```
target/
debug/
Cargo.lock
**/*.rs.bk
*.pdb
```

1. Final structure files:
```bash
user@thinkp:hands-on$ tree -a
.
├── .cargo
│   └── config.toml
├── Cargo.toml
├── .gitignore
├── README.md
└── workspace.code-workspace
```

1. Create program exec01 and library lib01
```bash
# create program exec01 in workspace
cargo new --bin exec01

# create library lib01 in workspace
create new --lib lib01
```

8. Add ref of lib01 in cargo exec01 (./exec01/Cargo.toml)
```toml
[dependencies]
# add line bellow
lib01 = { path = "../lib01" }
```

1. Add ref of lib01 in  main.rs of  exec01 (./exec01/src/main.rs)
```rs
use lib01::add
```
