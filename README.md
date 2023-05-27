# layout-derive

A procedural-macro for deriving the [layout-trait](https://github.com/perlindgren/layout-trait). The purpose is to provide tight yet safe memory layout information for applications written in the Rust [RTIC](https://rtic.rs/) framework and provides an outset for unprecedented task isolation, and a solid foundation for reasoning on error containment and propagation.

---

## A small example

```rust
#[derive(Layout)]
struct Simple {
    data: u32,
    data2: u64,
}
```

Expands to:

```rust
struct Simple {
    data: u32,
    data2: u64,
}

impl GetLayout for Simple {
    fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
        self.data.get_layout(layout);
        self.data2.get_layout(layout);
    }
}
```

The derive also works for nested structs, tuples and enums (unions are not yet implemented but follow the same pattern as enums.)

The derive macro also supports generic type parameters, however this feature is not yet fully tested.

---

## Development

To see the expansion of an example:

```shell
cd derive_examples
cargo expand --example simple > simple_expanded.rs
```

---

## derive_examples

The `derive_examples` workspace member provides representative use case examples and serves as a testing playground. Later a proper test architecture will be provided, with fail tests etc.

Some of the examples are replicated in the `layout-trait` repository for comparison against expected expansions.

---
## Panics

Notice, we assume the `layout` (`heapless::Vec`), to be of sufficient size to accommodate the result (will panic if insufficient).

---

## License

For now copyright Per Lindgren, but the intention once stake holder interests have been confirmed, is to release the trait and derive macros under a permissive license.
