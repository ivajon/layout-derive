#![feature(min_specialization)]
use core::ops::Deref;
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

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

struct Proxy2 {}

impl Deref for Proxy2 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 2 ---");
        unsafe { &*(0x2000 as *const RegisterBlock) }
    }
}

struct Proxy3 {}

impl Deref for Proxy3 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 3 ---");
        unsafe { &*(0x3000 as *const RegisterBlock) }
    }
}

struct Proxy4 {}

impl Deref for Proxy4 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 4 ---");
        unsafe { &*(0x4000 as *const RegisterBlock) }
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
    a: u32,
    en: Enum,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Resources {
        a: 0,
        en: Enum::A(Tuple(0, Proxy1 {})),
    };
    a.get_layout(&mut layout);
    println!("{:?}", layout);
}
