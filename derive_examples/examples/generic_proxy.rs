#![feature(min_specialization)]
use derive_examples::Proxy;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

#[derive(Layout)]
struct Generic<T> {
    generic: T,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Generic { generic: Proxy {} };
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    // Proxy
    assert_eq!(
        layout[0],
        Layout {
            address: 0x1000,
            size: 0x8
        }
    );
}
