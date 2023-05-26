# layout-derive

A proc-macro for deriving the `layout-trait`.

---

## Examples

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

The derive macro does not currently support generic parameters.

---

## Development

To see the expansion of an example:

```shell
cd derive_examples
cargo expand --example simple > simple_expanded.rs
```

---

## Panics

Notice, we assume the `layout` (`heapless::Vec`), to be of sufficient size to accommodate the result (will panic if insufficient).

---

## License

For now copyright Per Lindgren, but the intention once stake holder interests have been confirmed, is to release the trait and derive macros under a permissive license.
