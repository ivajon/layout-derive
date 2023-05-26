#![feature(min_specialization)]
use core::ops::Deref;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, GetLayoutType, Layout};
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
enum Enum {
    A(Proxy),
    B(u32, u32),
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let d = Enum::A(Proxy {});
    d.get_layout(&mut layout);
    println!("{:?}", layout);
}
