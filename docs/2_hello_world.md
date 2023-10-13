---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
marp: true
---

# **Hello World**

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

```bash
# helloworld.rs
fn main() {
    println!("Hello, world!");
}
```

```bash
rustc helloworld.rs
./helloworld
```

---

## rustfmt

- **rustfmt** tool reformats your code according to the community code style.

### install rustfmt

```bash
rustup component add rustfmt
```

- rustfmt and cargo-fmt are similiar with rustc and cargo

---

# **Hello Cargo**

## creating a project with cargo

```bash
cargo new hello_cargo
cd hello_cargo
```

### override git

```bash
cargo new --vcs=git

# or run cargo new command within an existing Git repo
```

### build project

```bash
cargo build
```

---

### run project

```bash
cargo run
```

### check code to make sure it compiles (no run)

```bash
cargo check
```

### release build

```bash
cargo build --release
```

### cargo docs

- https://doc.rust-lang.org/cargo/

---
