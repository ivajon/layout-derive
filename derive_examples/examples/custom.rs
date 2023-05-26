#![feature(min_specialization)]
use derive_examples::Proxy;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

#[derive(Layout)]
struct Custom {
    proxy: Proxy,
    b: u32,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Custom {
        proxy: Proxy {},
        b: 0,
    };
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    // Proxy
    assert_eq!(
        layout[0],
        Layout {
            address: 0x1000,
            size: 8
        }
    );

    // u32
    assert_eq!(layout[1].size, 4);
}
