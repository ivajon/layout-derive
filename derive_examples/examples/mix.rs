#![feature(min_specialization)]
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, GetLayoutType, Layout};

struct Proxy1 {}
impl GetLayoutType for Proxy1 {
    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<Layout, N>) {
        println!("-- Proxy1 --");
        layout
            .push(layout_trait::Layout {
                address: 1024,
                size: 4,
            })
            .unwrap()
    }
}

struct Proxy2 {}
impl GetLayoutType for Proxy2 {
    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<Layout, N>) {
        println!("-- Proxy2 --");
        layout
            .push(layout_trait::Layout {
                address: 2048,
                size: 8,
            })
            .unwrap()
    }
}

struct Proxy3 {}
impl GetLayoutType for Proxy3 {
    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<Layout, N>) {
        println!("-- Proxy3 --");
        layout
            .push(layout_trait::Layout {
                address: 3333,
                size: 8,
            })
            .unwrap()
    }
}

struct Proxy4 {}
impl GetLayoutType for Proxy4 {
    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<Layout, N>) {
        println!("-- Proxy4 --");
        layout
            .push(layout_trait::Layout {
                address: 3333,
                size: 8,
            })
            .unwrap()
    }
}

#[derive(Layout)]
struct Tuple(u32, Proxy1);

#[derive(Layout)]
enum Enum {
    A(Tuple),
    B(Proxy2),
    C(Proxy3, Proxy4),
    D([u32; 16]),
}

#[derive(Layout)]
struct Resources {
    tuple: Tuple,
    en: Enum,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Resources {
        tuple: Tuple(0, Proxy1 {}),
        en: Enum::A(Tuple(0, Proxy1 {})),
    };
    a.get_layout(&mut layout);
    println!("{:?}", layout);
}
