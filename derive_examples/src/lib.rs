use core::ops::Deref;

// emulate peripheral access proxies
pub struct Proxy {}

#[derive(Debug)]
pub struct RegisterBlock {
    _reg1: u32,
    _reg2: u32,
}

impl Deref for Proxy {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

pub struct Proxy1 {}

impl Deref for Proxy1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 1 ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

pub struct Proxy2 {}

impl Deref for Proxy2 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 2 ---");
        unsafe { &*(0x2000 as *const RegisterBlock) }
    }
}

pub struct Proxy3 {}

impl Deref for Proxy3 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 3 ---");
        unsafe { &*(0x3000 as *const RegisterBlock) }
    }
}

pub struct Proxy4 {}

impl Deref for Proxy4 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 4 ---");
        unsafe { &*(0x4000 as *const RegisterBlock) }
    }
}
