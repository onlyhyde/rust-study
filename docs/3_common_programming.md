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
