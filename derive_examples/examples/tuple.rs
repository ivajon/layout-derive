#![feature(min_specialization)]

use derive_examples::Proxy;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

#[derive(Layout)]
struct Tuple(u32, Proxy);

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Tuple(1, Proxy {});
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    // u32
    assert_eq!(layout[0].size, 4);

    // Proxy
    assert_eq!(
        layout[1],
        Layout {
            address: 0x1000,
            size: 8
        }
    );
}
