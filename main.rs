#![no_std]
#![no_main]
#![feature(macro_rules)]
#![feature(globs)]
#![feature(lang_items)]

#[repr(C, packed)]
struct GpioPort {
    gper : i32,
    gpers : i32,
    gperc : i32,
    gpert : i32,
    pmr0 : i32,
    pmr0s : i32,
    pmr0c : i32,
    pmr0t : i32,
    //0x20
    pmr1 : i32,
    pmr1s : i32,
    pmr1c : i32,
    pmr1t : i32,
    pmr2 : i32,
    pmr2s : i32,
    pmr2c : i32,
    pmr2t : i32,
    //0x40
    oder : i32,
    oders : i32,
    oderc : i32,
    odert : i32,
    ovr : i32,
    ovrs : i32,
    ovrc : i32,
    ovrt : i32,
    //0x60
    pvr : i32,
    reserved0 : [i32, ..3],
    puer : i32,
    puers : i32,
    puerc : i32,
    puert : i32,
    //0x80
    pder : i32,
    pders : i32,
    pderc : i32,
    pdert : i32,
    ier : i32,
    iers : i32,
    ierc : i32,
    iert : i32,
    //0xA0
    imr0 : i32,
    imr0s : i32,
    imr0c : i32,
    imr0t : i32,
    imr1 : i32,
    imr1s : i32,
    imr1c : i32,
    imr1t : i32,
    //0xC0
    gfer : i32,
    gfers : i32,
    gferc : i32,
    gfert : i32,
    ifr : i32,
    reserved1 : i32,
    ifrc : i32,
    reserved2 : i32,
    //0xE0
    reserved3 : [i32, ..8],
    //0x100
    odcr0 : i32,
    odcr0s : i32,
    odcr0c : i32,
    odcr0t : i32,
    odcr1 : i32,
    odcr1s : i32,
    odcr1c : i32,
    odcr1t : i32,
    //0x120
    reserved4 : [i32, ..4],
    osrr0 : i32,
    osrr0s : i32,
    osrr0c : i32,
    osrr0t : i32,
    //0x140
    reserved5: [i32, ..8],
    //0x160
    ster : i32,
    sters : i32,
    sterc : i32,
    stert : i32,
    reserved6 : [i32, ..4],
    //0x180
    ever : i32,
    evers : i32,
    everc : i32,
    evert : i32,
    reserved7: [i32, ..112]
    //0x200 end
}

enum int {
    GPIO_PORT0_ADDRESS = 0x400E1000,
    GPIO_PORT1_ADDRESS = 0x400E1200,
    GPIO_PORT2_ADDRESS = 0x400E1400
}

#[lang="sized"]
pub trait Sized {}

#[no_mangle]
pub extern fn main() -> int {
  let gpio_c : &mut GpioPort = unsafe {
    &mut *(GPIO_PORT0_ADDRESS as int as *mut GpioPort)
  };

  gpio_c.gpers = 1<<10;
  gpio_c.oders = 1<<10;
  gpio_c.sterc = 1<<10;
  
  loop {
    gpio_c.ovrt = 1<<10;
    let mut i = 0i;
    loop {
      i = i + 1;
      if i > 5000000 {
        break;  
      }
    }
  }
}
