#![feature(min_specialization)]
use derive_examples::{Proxy1, Proxy2, Proxy3, Proxy4};
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

#[derive(Layout)]
struct Tuple(u32, Proxy1);

#[derive(Layout)]
#[allow(unused)]
enum Enum {
    A(Tuple),
    B(Proxy2),
    C(Proxy3, Proxy4),
    D([u32; 16]),
}

#[derive(Layout)]
struct Resources {
    a: u32,
    en: Enum,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Resources {
        a: 0,
        en: Enum::A(Tuple(0, Proxy1 {})),
    };
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    // u32
    assert_eq!(layout[0].size, 0x4);

    // Enum
    assert_eq!(layout[1].size, 0x44);

    // Proxy 1
    assert_eq!(
        layout[2],
        Layout {
            address: 0x1000,
            size: 0x8,
        }
    );

    // Proxy 2
    assert_eq!(
        layout[3],
        Layout {
            address: 0x2000,
            size: 0x8,
        }
    );

    // Proxy 3
    assert_eq!(
        layout[4],
        Layout {
            address: 0x3000,
            size: 0x8,
        }
    );

    // Proxy 4
    assert_eq!(
        layout[5],
        Layout {
            address: 0x4000,
            size: 0x8,
        }
    );
}
