# Template-rust workspace

## Iniciando um projeto com estrutura completa

1. Criar workspace chamado hands-on
    ```bash
    mkdir hands-on
    cd hands-on
    ```

2. Informação sobre o target no item <b>host<b>
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

3. Criar arquivo do controle de build .cargo/config.toml
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

4. Criar arquivo base do worksapce (./Cargo.toml)
    ```toml
    [workspace]
    members = []
    resolver = "1"
    ```

5. Criar arquivo ./.gitignore
   ```
    target/
    debug/
    Cargo.lock
    **/*.rs.bk
    *.pdb
   ```

6. Final estrutura basica:
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
