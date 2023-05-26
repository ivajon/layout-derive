#![feature(min_specialization)]
use esp32c3;
use esp32c3::{AES, DMA, GPIO, I2C0};
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

#[derive(Layout)]
struct MyI2C {
    gpio: GPIO,
    i2c0: I2C0,
}

#[derive(Layout)]
struct Resources {
    aes: AES,
    dma: DMA,
    i2c: MyI2C,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let p = unsafe { esp32c3::Peripherals::steal() };
    let r = Resources {
        aes: p.AES,
        dma: p.DMA,
        i2c: MyI2C {
            gpio: p.GPIO,
            i2c0: p.I2C0,
        },
    };
    r.get_layout(&mut layout);
    println!("{:x?}", layout);
}
