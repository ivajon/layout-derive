#![feature(min_specialization)]
use core::ops::Deref;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

struct Proxy {}

#[derive(Debug)]
struct RegisterBlock {
    reg1: u32,
    reg2: u32,
}

impl Deref for Proxy {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

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
    println!("{:?}", layout);
}
