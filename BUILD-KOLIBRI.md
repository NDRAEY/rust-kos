# Building Rust compiler to build for KolibriOS

0. Ensure you have Rust host compiler installed!

If not, install it:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. Clone the repo:
```
git clone https://github.com/NDRAEY/rust-kos.git
cd rust-kos
```

2. Checkout the `kolibri` branch:
```
git checkout kolibri
```

3. Copy the config:
```
cp bootstrap.kolibri.toml bootstrap.toml
```

4. Build:
```
./x build
```

5. When build is successful, link toolchain:
```
rustup toolchain link kolibri-stage1 build/x86_64-unknown-linux-gnu/stage1/
```

6. Now you're all set! Use new toolchain and don't forget to specify target to compile crates for KolibriOS:
```
cargo +kolibri-stage1 build --target i586-unknown-kolibri
```

*You can find built `.kex` executables at `target/i586-unknown-kolibri/{debug,release}/` directory.*
