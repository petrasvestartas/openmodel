
# Rust



## Installation 

### Step 1 - Install Rust via Terminal
Install Rust via [RustUp](rust-lang.org/tools/install) Package Manager.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Step 2 - Verify Installation

```bash
cargo --version
```

## Commands

### Create a new project
```bash
cargo new <projectname>
```

### Run a project
```bash
cargo run
```

### Run a project without Debug Info
```bash
cargo run -q
```


### Step 3 - VSCode

Extensions: [rust analyzer](https://code.visualstudio.com/docs/languages/rust)


### Step  4 - Example

```rust
mod point;
use point::Point;

fn main() {
    let p0 = Point::new(0.0, 0.0, 0.0);
    let p1 = Point::new(1.0, 1.0, 1.0);
    println!("Distance: {}", p0.distance(&p1));
    println!("Hello, world!");
}
```

### Step 5 - Publish Create


```bash
cargo login
cargo publish
```

### Step 6 -  Formatting
```bash
rustup component add rustfmt
cargo fmt
```