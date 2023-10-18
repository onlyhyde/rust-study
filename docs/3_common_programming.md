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

```rust
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
- constant is always immutable
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

# Data Types

- Rust의 모든 값은 특정 데이터 유형을 갖는다.
- Scalary Types
- Compound Types

---

## Scalar Types

- 단일 값을 나타냄
- Int, Float, Bool, Char

---

### Integer Type

- 부호 있음
- i8, i16, i32, i64, i128, isize

- 부호 없음
- u8, u16, u32, u64, u128, usize

---

### Floating-Point Type

- f32, f64
- default : f64

### Numeric Operations

- 덧셈, 뺄셈, 곱셈, 나눗셈, 나머지 연산 지원

### Boolean Type

- size : 1byte
- true or false

### Character type

- size : 4byte
- unicode scalar 값을 나타낸다
- unicode scalar 값의 범위 : U+0000 ~ U+D7FF, U+E000 ~ U+10FFFF

## Compound Types

### Tuple Type

- 다양한 유형의 여러 값을 하나의 복합 유형으로 그룹화하는 방법.
- 길이가 고정되어있고, 일단 선언되면 크기가 늘어나거나 줄어들 수 없다.

### Array Type

- 동일한 유형
- 고정된 길이

---

# Functions

## Parameters

## Statements and Expressions

## Functions with Return Values

---

# Comments

---

# Control Flow

## if Expressions

## Repetition with Loops

### Repeating Code with loop

### Returning Values from loop

### Loop Labels to Disambiguate Between Multiple Loops

### while

### for

---
