# predicate-checker

Predicate-checker is a small library written Rust whose only purpose is to check that the validity of a given predicate `A` implies the validity of a predicate `B`.

For example, given the predicates `A: x > 5` and `B: x > 6`, the library can (using the `Predicate::implies()` function) verify that for every value of `x` where `A` is true, `B` is also true (in this example, it is; so the `A.implies(B)` function would return `Implication::Total`).

> **Note: this library was not made by a mathematician, nor a good programmer, nor a good gardener. The results are not 100% truthful and should not be used for anything more important than a small project. Sorry!**


## Usage

The `Predicate` enum represents a boolean expression. You can build one "manually" by assembling other `Predicate<T>` (`T` being a numerical type), or using the builtin parser:

```rust
use predicatechecker::Predicate;
let predicate = Predicate::from("(x > 2) && (y == 4) && (z < 10)").unwrap(); // returns a Predicate<f64>
```

Now, you can check if a predicate "implies" another:

```rust
use predicatechecker::Predicate;

fn main() {
    let a = Predicate::from("(x > 2) && (y == 4) && (z < 10)").unwrap();
    let b = Predicate::from("(x > 0) && (y > 2)").unwrap();

    assert_eq!(a.implies(&b), Implication::Total);
}
```

The `implies` function can return 3 different values: `Implication::Total`, `Implication::Partial` or `Implication::Inexistant`:
- A `Total` implication means that any value that verifies A will verify B.
- A `Partial` implication means that only a subset of the values that verify A will verify B. This can happen with `Or` predicates, for which only one of the two predicates implies B.
- An `Inexistant` implication means that the verification of A by a value x is not enough to know that x also verifies B.


## Installation

The easiest way to add it to your projects is by using cargo. Add it to your `Cargo.toml` file:
```
[dependencies]
predicatechecker = "0.5.1"
```

You can also clone this repository and use it in your own projects (see [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) from The Cargo Book).

## License

This project is licensed under **Mozilla Public License 2.0**. See `LICENSE.txt`.