#![feature(min_specialization)]
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

#[derive(Layout)]
struct Generic<T> {
    generic: T,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Generic { generic: 0u32 };
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    // u32
    assert_eq!(layout[0].size, 4);
}
