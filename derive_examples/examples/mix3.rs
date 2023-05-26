#![feature(min_specialization)]
use derive_examples::Proxy1;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

#[derive(Layout)]
struct Struct {
    a: u32,
    b: Proxy1,
}

#[derive(Layout)]
enum Enum {
    A(Struct),
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Enum::A(Struct { a: 0, b: Proxy1 {} });

    a.get_layout(&mut layout);
    println!("{:#x?}", layout);
}
