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
struct Tuple(u32, Proxy);

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Tuple(1, Proxy {});
    a.get_layout(&mut layout);
    println!("{:?}", layout);
}
