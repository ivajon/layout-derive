#![feature(min_specialization)]
use derive_examples::Proxy;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

#[derive(Layout)]
struct Simple {
    a: Proxy,
    b: u64,
}

#[derive(Layout)]
struct Nested {
    simple: Simple,
    b: u64,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Nested {
        simple: Simple { a: Proxy {}, b: 0 },
        b: 0,
    };
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    
}
