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

The derive also works for nested structs and tuples, while enums are yet not implemented. Notice, primitive enums should work since a default implementation of `GetLayout` is provided for all `T`.

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
