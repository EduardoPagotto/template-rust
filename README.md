# Rust workspace setup
Just a generic template for new projects:

- Workspace setup
- VScode setup (some extentions)
- Update last version of rust (1.88)

## Workspace basic setup

1. Create a workspace name as hands-on (for example)
```bash
mkdir hands-on
cd hands-on
```

2. About the target <b>host<b>
```bash
# updade last version rust
rustup update
# ..

rustc -vV
    rustc 1.88.0 (6b00bc388 2025-06-23)
    binary: rustc
    commit-hash: 6b00bc3880198600130e1cf62b8f8a93494488cc
    commit-date: 2025-06-23
    host: x86_64-unknown-linux-gnu
    release: 1.88.0
    LLVM version: 20.1.5
```

3. Add file with build parameters:[.cargo/config.toml](.cargo/config.toml)
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

4. Add workspace setup file:[./Cargo.toml](./Cargo.toml)
```toml
[workspace]
members = []
resolver = "3" # 2024

[workspace.package]
version = "0.1.1"
authors = ["Eduardo Pagotto"]
description = "Just a template to understand workspaces in rust"
documentation = "https://github.com/EduardoPagotto/template-rust"

#[workspace.dependencies]
```

5. Add file gitignore:[.gitignore](.gitignore)
```
target/
debug/
Cargo.lock
**/*.rs.bk
*.pdb
```

6. Structure of workspace empty:
```bash
user@thinkp:hands-on$ tree -a
.
├── .cargo
│   └── config.toml            Target, parameters of compiling and linker
├── Cargo.toml                 Workspace cargo setup
├── .gitignore
├── README.md
└── workspace.code-workspace   Just vscode setup
```

## Create Packages in Workspace
1. Create package exec01 and package library lib01
```bash
# create package executable exec01 in workspace
cargo new --bin exec01

# create package library lib01 in workspace
create new --lib lib01
```

2. Add ref of package lib01 in package exec01:[./exec01/Cargo.toml](./exec01/Cargo.toml)
```toml
[dependencies]
# add line bellow
lib01 = { path = "../lib01" }
```

3. Add ref of lib01 in  main.rs of  exec01: [./exec01/src/main.rs](./exec01/src/main.rs)
```rs
// Add other Package in same worksapce
use lib01::add;

// reference to lib inside same crate of main but other directory
// need 'mod' and 'use' to get module, no matter the order
use opp::zero::{testa_nao_zero, testa_zero};
mod opp;

// Reference module in same directori of main.rs
mod aux;
```

## Organization files
```bash
├── Cargo.toml                   Cargo of workspace
├── exec01                       Package exec01
│   ├── Cargo.toml               Cargo of Package exec01
│   └── src
│       ├── main.rs              Crate exec01, entry point
│       ├── aux.rs               Module aux belongs crate main
│       └── opp                  Crate opp
│           ├── mod.rs           Root module opp expose components
│           └── zero.rs          module opp::zero whit methods
│  
├── lib01                        Package exec01
│   ├── Cargo.toml               Package lib01
│   └── src
│       └── lib.rs               crate lib01
├── LICENSE
├── README.md
└── workspace.code-workspace
```

### to test
```bash
cargo check --workspace
cargo build
```

## VSCode
Extentions used in vscode with rust (rustc 1.88.0 (6b00bc388 2025-06-23))
```bash
# List
#code --list-extensions --profile Rust_X86

# Install
#code --profile "Your Profile Name" --install-extension "extension.id"

code --profile "Rust_X86" --install-extension "daylerees.rainglow"
code --profile "Rust_X86" --install-extension "dustypomerleau.rust-syntax"
code --profile "Rust_X86" --install-extension "fill-labs.dependi"
code --profile "Rust_X86" --install-extension "gruntfuggly.todo-tree"
code --profile "Rust_X86" --install-extension "llvm-vs-code-extensions.lldb-dap"
code --profile "Rust_X86" --install-extension "pkief.material-icon-theme"
code --profile "Rust_X86" --install-extension "rust-lang.rust-analyzer"
code --profile "Rust_X86" --install-extension "tamasfe.even-better-toml"
code --profile "Rust_X86" --install-extension "usernamehw.errorlens"
code --profile "Rust_X86" --install-extension "vadimcn.vscode-lldb"
```
