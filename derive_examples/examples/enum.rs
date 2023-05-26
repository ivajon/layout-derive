#![feature(min_specialization)]
use derive_examples::Proxy;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

#[derive(Layout)]
enum Enum {
    A(Proxy),
    #[allow(unused)]
    B(u32, u32),
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let d = Enum::A(Proxy {});
    d.get_layout(&mut layout);
    println!("{:?}", layout);

    // Enum
    assert_eq!(layout[0].size, 12); // tag (u32) + 2 * u32 in tuple

    // Proxy
    assert_eq!(
        layout[1],
        Layout {
            address: 0x1000,
            size: 8
        }
    );
}
