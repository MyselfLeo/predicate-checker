# predicate-checker

Predicate-checker is a small library written Rust whose only purpose is to check that the validity of a given predicate `A` implies the validity of a predicate `B`.

For example, given the predicates `A: x > 5` and `B: x > 6`, the library can (using the `Predicate::implies()` function) verifies that for every values of `x` where `A` is true, `B` is also true (in this example, it is; so the `A.implies(B)` function would return `true`)

> **Note: this library was not made by a mathematician, nor a good programmer, nor a good gardener. The results are not 100% truthful and should not be used for anything more important than a small project. Sorry!**


## Usage

The `Predicate` enum represents a boolean expression. You can build one "manually" by assembling other `Predicate`, or using the builtin parser:

```rust
use predicatechecker::Predicate;
let predicate = Predicate::from("(x > 2) && (y == 4) && (z < 10)").unwrap();
```

Now, you can check if a predicate "implies" into another:

```rust
use predicatechecker::Predicate;

fn main() {
    let a = Predicate::from("(x > 2) && (y == 4) && (z < 10)").unwrap();
    let b = Predicate::from("(x > 0) && (y > 2)").unwrap();

    println!("{}", a.implies(&b)); // returns true
}
```


## Installation
You can clone this repository and use it in your own projects (see [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) from The Cargo Book). I do not plan to make it available on `crates.io` for now.


## License

This project is licensed under **Mozilla Public License 2.0**. See `LICENSE.txt`.