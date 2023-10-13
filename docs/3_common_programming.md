---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
marp: true
---

# **Variables and Mutability**

- by default, variables are immutable
- have option to make variables mutable

---

## Immutable

- when variable is immutable, can't change that value.

### Error code

```bash
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // x is immutable
    println!("The value of x is: {x}");
}
```

```bash
cargo run
```

---

## Mutable

```bash
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

---

## Constants

- not allowed to use **mut**
- rust's naming convention for constants : all upercase with underscores

```bash
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

---

## Shadowing

- can declare a new variable with the same name as a previous variable
- different with **mut**
- will get a compile-time error if we try to reassign to this variable without **let**
- can change type with same name

---

```bash
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

---

- shadowing

```bash
let spaces = "   ";
let spaces = spaces.len();

```

- error

```bash
let mut spaces = "   ";
spaces = spaces.len();
```

---

## Data Types

---
