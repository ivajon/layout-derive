#![feature(min_specialization)]
use derive_examples::Proxy1;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

#[derive(Layout)]
struct Tuple(u32, Proxy1);

#[derive(Layout)]
enum Enum {
    A(Tuple),
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Enum::A(Tuple(0, Proxy1 {}));

    a.get_layout(&mut layout);
    println!("{:#x?}", layout);
}
