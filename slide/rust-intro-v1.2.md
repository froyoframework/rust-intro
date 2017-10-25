# Introduction to Rust

## Adityo Pratomo [@kotakmakan](http://www.twitter.com/kotakmakan)

---

# This Talk is Also Available at

[github.com/froyoframework/rust-intro/slide](http://www.github.com/froyoframework/rust-intro)

---

# Background

- Chief Academic Officer at Framework
- Chief Technology Officer at Labtek Indie
- Certified Unity Developer

---

# Framework

- Providing software development course, training and workshop
- For Company and general public
- Based in Bintaro

![](https://scontent.cdninstagram.com/t51.2885-19/s150x150/12501964_776556112474284_1583292585_a.jpg)

---

# Labtek Indie

- Rapid Prototyping As A Service
- Based in Bandung

![](https://d1qb2nb5cznatu.cloudfront.net/startups/i/321497-a83fd6324ed6c76e05dc137012e59bef-medium_jpg.jpg?buster=1389436901)

---

# Codewise, I'm

- Generalist, but mostly interactive graphic and multimedia
- Creative Coder
- C/C++, JS, C#, Java

---

# When I Meet Rust

- Fast and safe system programming language
- Like C++, without segfaults (yum!)
- Better handling of reference and pointers
- Mixture of imperative and functional paradigm

---

# Rust Trifecta

![](img/rusttrifecta.jpg)

---

# What to Do with Rust?

- System programming
    - Apps that require precise memory management
        - Web Browser (Mozilla Firefox)
        - Distributed storage system (Dropbox)
        - 3D Games
        - Device driver
        - Operating System
- General tool

---

# Rust's Killer Features

- Concept of "Borrowing and Ownership"
    - Type safety
    - Memory safety
- Zero-cost abstractions
    - Pattern matching
- Easy cross compilation    

---

#  How Rust is Fast?

- Access to lower resource with thin layer
- Fast: No garbage collection, Rust automatically detect when to free memory
    + Lifetime
    + Ownership of data

---

# How Rust is Secure?

- Extensive compiler checking
- Secure: No data race, guaranteed data lifetime, no dangling-pointer
    + Ownership and Borrowing only allows one mutable reference (write access)

---

# Rust Type System

- Borrows from the functional programming -> structured types and traits 
- The type system is very strong 
- Furthermore, the type system is static
    - Must know the types of all variables at compile time 
    - The compiler can usually infer the type based on value and usage

---

# Rust Type System Goal

- Safety: By checking a program’s types, the Rust compiler rules out whole classes of common mistakes -> null pointer and unchecked unions
- Efficiency: Programmers have fine-grained control over how Rust programs represent values in memory
- Concision: Rust manages the above without requiring too much guidance from the programmer

---

# Also in Rust

- Built-in unit testing
- Cargo: Rust's built-in package and build manager
- rustup: Toolchain manager
- Helpful error messages in compiler

---

# Rust and other languages

- Thanks to LLVM backend, it can talk to other languages that use the same backend
- Rust has built-in FFI (Foreign Function Interface)
- For example, C, C++, Ruby, python, NodeJS, and other
- This might actually boost your program performance
    + My python code runs ~12x faster
    + My node code runs ~8x faster

---

# Cross-compiling Rust

- Cross compilation is a big thing in Rust
- With the toolchain available in Rustup, it's easy to setup a cross-compilation environment
- Ex: from Linux, you can also build for Windows, or from Mac you can build for Linux

---

# Installing Rust

- Use rustup
- [http://www.rustup.rs](http://www.rustup.rs)
- rustup will install:
    - toolchain for compilation including Cargo
    - additional target for cross-compilation
    - various rust version (`stable`, `nightly`, `beta`)

---

# Creating New Rust Project

```
cargo new myproject --bin
```

---

# Compiling Rust Project

```
cargo build myproject
cargo run myproject
```

---

# Cargo and Crate

- Cargo manages the project
- Cargo also handles the external dependencies
- Dependencies are called crate
- Crates are available at [crates.io](http://www.crates.io)

---

# Cargo and Crate

```
[package]
name = "omar"
version = "0.1.0"
authors = ["Adityo Pratomo <pratomo.adityo@gmail.com>"]

[dependencies]
iron = "*"
mime = "*"
router = "*"
mysql = "*"
rustc-serialize = "*"
```

---

# Writing Rust

- Various support for IDE: Visual Studio Code, IntelliJ, Atom, vi, emacs
- I choose Visual Studio Code because of its features (RLS)

---

# A Tour of Rust's Syntax

[https://github.com/froyoframework/rust-intro/tree/master/basic-rust-sample](https://github.com/froyoframework/rust-intro/tree/master/basic-rust-sample)

---

# Variable

- Variable by default is immutable
- A binding to value exists
- Rust can automatically infer a variable type

```
let angka = 9;
let salam = "Selamat datang, Android no ";
let halo = format!("{} {}", salam, angka);
println!("{:?}", halo);
```

---

# `String` and `str`

- Rust keeps 2 format of string, `String` and `str`
- `String` is a high level format of string and not zero cost in memory
    - Content can be changed, mutable and is guaranteed Unicode
- `str` is a string slice that allows no allocation costs and zero copying of memory when passing it around
    - fixed size and content cannot be changed

---

# `String` and `str`

```
fn say_hello(to_whom: &str) {
    println!("Hey {}!", to_whom)
}

fn main() {
    let string_slice: &'static str = "you";
    let string: String = string_slice.into();
    say_hello(string_slice);
    say_hello(&string);
}
```

---

# Function

- Return value in function is explicitly denoted using arrow
- The returned value is the last variable stated without semicolon

```
let angka_saya = calc(angka);

fn calc(x: i32) -> i32 {
    let y;
    match x {
        1...40 => y = 34,
        _ => y = 2,
    }
    y
}

```

---

# Struct

- A simple compount data structure that contains key-value entities
- Each key-value can use different data Type
- other compound data type includes `enum` and `tuple`

```
struct Pemain {
    nama: String,
    umur: i32,
    gol: i32,
}

let buffon = Pemain {nama: "Buffon".to_string(), umur: 39, gol: 0 };

```

---

# Struct Method

- A struct can have its own method that operate on a specific struct or return values of a specific struct
- To accomplish this, we use implementation blocks with `impl` keyword

```
impl Pemain {
    fn new(nama: &str) -> Pemain {
        Pemain {
            nama: nama.to_string(),
            umur: 27,
            gol: 100
        }
    }

    fn get_gol(&self) -> i32 {
        self.gol
    }
}
```

---

# Struct Method

- And the previous struct method can be called as follow:

```
let pemain_milan = Pemain::new("Andre Silva");
let jumlah_gol = pemain_milan.get_gol();
println!("Jumlah gol Andre Silva {}", jumlah_gol);
```

---


# Make Struct with Function

```

fn tambah_p(nama_: &str, umur_: i32, gol_: i32) -> Pemain {
  let pemain_saya = Pemain { 
    nama: nama_.to_string(),
    umur: umur_,
    gol: gol_,
  };
  pemain_saya
}

let ronaldo = tambah_p("Ronaldo", 31, 510);

```

---



# Vector

- Array-like structure that can be dynamically manipulated during runtime
- Can contain anything, from integers, floats, Strings, to Structs

```
let deret = vec![1, 2, 3];
let mut himpunan = Vec::new();
himpunan.push(5);
himpunan.push(6)
```

---

# Vector of Structs

```
fn tambah_para_pemain() -> Vec<Pemain> {
    let ronaldo = tambah_pemain("Ronaldo", 31, 510);
    let bacca = tambah_pemain("Bacca", 31, 235);
    let payet = tambah_pemain("Payet", 28, 150);

    let mut pemain_favorit = Vec::new();
    pemain_favorit.push(ronaldo);
    pemain_favorit.push(bacca);
    pemain_favorit.push(payet);

    pemain_favorit
}

let pemain_keren = tambah_para_pemain();
```

---

# Ownership and Borrowing

- A key concept that ensures safety and concurrency in Rust
- Basically everytime a variable is used, its ownership is transferred to the one uses/calls it
- When an ownership is transferred, the old owner can't use the entity anymore
- Checked at compile time

---

# Ownership and Borrowing

```
let pemain_bola = pemain_keren;
println!("pemain pertama adalah: {}", pemain_keren[0].nama);
```

---

# Ownership and Borrowing

- To solve the previous problem, Rust introduces Borrowing
- This means that a variable can be borrowed, thus, it's still valid for being used elsewhere
- This is accomplished by simple referencing that intended variable, thus the term "reference"

```
let pemain_bola = &pemain_keren;
println!("pemain pertama adalah: {}", pemain_keren[0].nama);
```

---

# Ownership and Borrowing

- Another thing that's correlated with referencing, is dereferencing
- This means, accessing the value of a referenced variable
- By default, a reference is immutable
- Change the value of a referenced variable by using mutable reference

---

# Ownership and Borrowing

```
let mut a = 90;
let b = &mut a;
*b += 1;
println!("{}", b); //prints 91
```

---

# Ownership and Borrowing

- A consequence of borrowing, is the concept of a borrow lifetime
- This is denoted by a curly brace

```
let mut a = 90;
{
    let b = &mut a;     // a dipinjam di sini
    *b += 9;            // isi a diakses di sini
}                       // peminjaman a berakhir di sini
println!("{}", a);
```

---

# Ownership and Borrowing

- To ensure safety, the main rule in borrowing is:
    + one or more references (&T) to a resource,
    + exactly one mutable reference (&mut T).

---

# Concurrency

- Concurrency enables us to do things in parallel
- Rust inherent safety prevents events such as race condition, data race and deadlock
- We can easily spawn new thread to do concurrency

---

# Concurrency

```
use std::thread;
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    tx.send(0);
    thread::spawn(move || {
        tx.send(1)
    });
    thread::spawn(move || {
        tx_clone.send(2)
    });
    println!("Received {} via the channel", rx.recv().unwrap());
    println!("Received {} via the channel", rx.recv().unwrap());
}
```

---

# A Simple Web Service

[https://github.com/lunchboxav/rust-basic-tcp-server](https://github.com/lunchboxav/rust-basic-tcp-server)

---

# Why Learn New Language?

- Gains new perspective on how things are done
- Gains new understanding on programming itself
- Make old and new things in a different way

---

# Tips for Learning Rust

- Katas: learn by making familiar things
- Try make small tool to replace your existing tool
- Consult the documentation
- Ask people on SO/Twitter
- Organize a community

---

# Learning Resources

- The Rust Book (https://doc.rust-lang.org/book/)
- Rust 101 (https://www.ralfj.de/projects/rust-101/main.html)
- Rust Tutorial (http://aml3.github.io/RustTutorial/html/toc.html)
- Rust Syntax (https://gist.github.com/brson/9dec4195a88066fa42e6)

---

# Learning Resources

- Rust By Example (http://rustbyexample.com/expression.html)
- Rustlings, small Rust Exercises (https://github.com/carols10cents/rustlings)
- 24 Days of Rust (http://zsiciarz.github.io/24daysofrust/)
- Rust FFI Omnibus (http://jakegoulding.com/rust-ffi-omnibus/)
- New Rustacean (http://www.newrustacean.com)

---

# More Info

Interested in learning more about Rust?
[didit@froyo.co.id](didit@froyo.co.id)

Interested in making prototype for your idea?
[didit@labtekindie.com](didit@labtekindie.com)

---

# Thank You


