# QuestPackageManager-Rust

QPM but rusty

# Building the program

First, make sure you have [Installed Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Open a command line / Powershell window

clone the repo

```
git clone https://github.com/RedBrumbler/QuestPackageManager-Rust.git
```

go into the folder

```
cd QuestPackageManager-Rust
```

run the build command

```
cargo build --release
```

the executable should now be found in ./target/release/qpm-rust

if you want to use it like this, add it to path

# Downloading the program

Download qpm-rust from the [latest github actions build](https://github.com/RedBrumbler/QuestPackageManager-Rust/actions/workflows/cargo-build.yml).
if nothing shows up, make sure you're logged in, if nothing still shows up we might have to manually make it generate a new version
Make sure you select the appropriate platform for your OS!

Now that you have this downloaded, you can unzip it and store it where you want it. I keep my qpm-rust executable in `S:/QPM-RUST` (irrelevant but just an example)

now you want to add the program to path so that you can run it from anywhere, 