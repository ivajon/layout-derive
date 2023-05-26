#![feature(min_specialization)]
use esp32c3;
use esp32c3::{AES, DMA, GPIO, I2C0};
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::{GetLayout, Layout};

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
    println!("{:#x?}", layout);

    // AES
    assert_eq!(
        layout[0],
        Layout {
            address: 0x6003a000,
            size: 0xbc
        }
    );

    // DMA
    assert_eq!(
        layout[1],
        Layout {
            address: 0x6003f000,
            size: 0x284
        }
    );

    // GPIO
    assert_eq!(
        layout[2],
        Layout {
            address: 0x60004000,
            size: 0x700
        }
    );

    // I2C0
    assert_eq!(
        layout[3],
        Layout {
            address: 0x60013000,
            size: 0x184
        }
    );
}
