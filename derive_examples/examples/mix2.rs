#![feature(min_specialization)]
use core::ops::Deref;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, GetLayoutType};

struct Proxy1 {}

#[derive(Debug)]
struct RegisterBlock {
    reg1: u32,
    reg2: u32,
}

impl Deref for Proxy1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 1 ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

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
    println!("{:?}", layout);
}
