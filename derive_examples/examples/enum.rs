#![feature(min_specialization)]
use heapless::Vec;

use layout_derive::Layout;
use layout_trait::{GetLayout, GetLayoutType, Layout};

struct Proxy {}
impl GetLayoutType for Proxy {
    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<Layout, N>) {
        println!("-- Proxy --");
        layout
            .push(layout_trait::Layout {
                address: 1024,
                size: 4,
            })
            .unwrap()
    }
}

#[derive(Layout)]
enum Enum {
    A,
    B(Proxy),
    C([u32; 100]),
    D,
    E,
}

#[derive(Layout)]

struct Resource {
    en: Enum,
    data: u32,
    data2: u64,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let r = Resource {
        en: Enum::A,
        data: 0,
        data2: 0,
    };
    r.get_layout(&mut layout);
    println!("{:?}", layout);
}
