#![feature(min_specialization)]
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{self, GetLayout};

struct Custom {}
impl layout_trait::GetLayout for Custom {
    fn get_layout<const N: usize>(
        &self,
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        println!("-- custom --");
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
    B(u32),
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Enum::A;
    a.get_layout(&mut layout);
    println!("{:?}", layout);
}
