use core::prelude::*;
use core::intrinsics;
use hil;

#[repr(C, packed)]
struct Register {
    val: u32,
    set: u32,
    clear: u32,
    toggle: u32
}

#[repr(C, packed)]
struct RegisterRO {
    val: u32,
    reserved: [u32; 3]
}

#[repr(C, packed)]
struct RegisterRC {
    val: u32,
    reserved0: u32,
    clear: u32,
    reserved1: u32
}

#[repr(C, packed)]
struct GPIOPortRegisters {
    gper: Register,
    pmr0: Register,
    pmr1: Register,
    pmr2: Register,
    oder: Register,
    ovr: Register,
    pvr: RegisterRO,
    puer: Register,
    pder: Register,
    ier: Register,
    imr0: Register,
    imr1: Register,
    gfer: Register,
    ifr: RegisterRC,
    reserved0: [u32; 8],
    ocdr0: Register,
    ocdr1: Register,
    reserved1: [u32; 4],
    osrr0: Register,
    reserved2: [u32; 8],
    ster: Register,
    reserved3: [u32; 4],
    ever: Register,
    reserved4: [u32; 26],
    parameter: u32,
    version: u32,
}

#[derive(Copy,Clone)]
pub enum PeripheralFunction {
    A, B, C, D, E, F, G, H
}


const BASE_ADDRESS: usize = 0x400E1000;
const SIZE: usize = 0x200;

#[derive(Copy,Clone)]
pub enum GPIOPort {
    GPIO0,
    GPIO1,
    GPIO2
}

// TODO: Should probably rename to GPIOPinLocation or something.
#[derive(Copy,Clone)]
pub enum Location {
    GPIOPin0, GPIOPin1, GPIOPin2, GPIOPin3, GPIOPin4, GPIOPin5, GPIOPin6,
    GPIOPin7, GPIOPin8, GPIOPin9, GPIOPin10, GPIOPin11, GPIOPin12, GPIOPin13,
    GPIOPin14, GPIOPin15, GPIOPin16, GPIOPin17, GPIOPin18, GPIOPin19, GPIOPin20,
    GPIOPin21, GPIOPin22, GPIOPin23, GPIOPin24, GPIOPin25, GPIOPin26, GPIOPin27,
    GPIOPin28, GPIOPin29, GPIOPin30, GPIOPin31, GPIOPin32, GPIOPin33, GPIOPin34,
    GPIOPin35, GPIOPin36, GPIOPin37, GPIOPin38, GPIOPin39, GPIOPin40, GPIOPin41,
    GPIOPin42, GPIOPin43, GPIOPin44, GPIOPin45, GPIOPin46, GPIOPin47, GPIOPin48,
    GPIOPin49, GPIOPin50, GPIOPin51, GPIOPin52, GPIOPin53, GPIOPin54, GPIOPin55,
    GPIOPin56, GPIOPin57, GPIOPin58, GPIOPin59, GPIOPin60, GPIOPin61, GPIOPin62,
    GPIOPin63, GPIOPin64, GPIOPin65, GPIOPin66, GPIOPin67, GPIOPin68, GPIOPin69,
    GPIOPin70, GPIOPin71, GPIOPin72, GPIOPin73, GPIOPin74, GPIOPin75, GPIOPin76,
    GPIOPin77, GPIOPin78, GPIOPin79, GPIOPin80, GPIOPin81, GPIOPin82, GPIOPin83,
    GPIOPin84, GPIOPin85, GPIOPin86, GPIOPin87, GPIOPin88, GPIOPin89, GPIOPin90,
    GPIOPin91, GPIOPin92, GPIOPin93, GPIOPin94, GPIOPin95
}

#[derive(Copy,Clone)]
pub struct GPIOPinParams {
    pub location: Location,
    pub port: GPIOPort,
    pub function: Option<PeripheralFunction>
}

pub struct GPIOPin {
    port: &'static mut GPIOPortRegisters,
    number: u8,
    pin_mask: u32
}

macro_rules! port_register_fn {
    ($name:ident, $reg:ident, $option:ident) => (
        fn $name(&mut self) {
            volatile!(self.port.$reg.$option = self.pin_mask);
        }
    );
}

// Note: Perhaps the 'new' function should return Result<T> to do simple init
// checks as soon as possible. Here, for example, we chould check that 'pin' is
// valid and panic before continuing to boot.
impl GPIOPin {
    pub fn new(params: GPIOPinParams) -> GPIOPin {
        let address = BASE_ADDRESS + (params.port as usize) * SIZE;
        let pin_number = ((params.location as usize) % 32) as u8;

        let mut pin = GPIOPin {
            port: unsafe { intrinsics::transmute(address) },
            number: pin_number,
            pin_mask: 1 << (pin_number as u32)
        };

        if params.function.is_some() {
            pin.select_peripheral(params.function.unwrap());
        }

        pin
    }

    pub fn select_peripheral(&mut self, function: PeripheralFunction) {
        let (f, n) = (function as u32, self.number as u32);
        let (bit0, bit1, bit2) = (f & 0b1, (f & 0b10) >> 1, (f & 0b100) >> 2);

        // clear GPIO enable for pin
        volatile!(self.port.gper.clear = self.pin_mask);

        // Set PMR0-2 according to passed in peripheral

        // bradjc: This code doesn't look great, but actually works.
        if bit0 == 0 {
            volatile!(self.port.pmr0.clear = 1 << n);
        } else {
            volatile!(self.port.pmr0.set = 1 << n);
        }
        if bit1 == 0 {
            volatile!(self.port.pmr1.clear = 1 << n);
        } else {
            volatile!(self.port.pmr1.set = 1 << n);
        }
        if bit2 == 0 {
            volatile!(self.port.pmr2.clear = 1 << n);
        } else {
            volatile!(self.port.pmr2.set = 1 << n);
        }
        // bradjc: These register assigns erase previous settings and don't
        //         work.
        // volatile!(self.port.pmr0.val = bit0 << n);
        // volatile!(self.port.pmr1.val = bit1 << n);
        // volatile!(self.port.pmr2.val = bit2 << n);
    }

    pub fn set_ster(&mut self) {
        volatile!(self.port.ster.set = 1 << self.number);
    }
}

impl hil::GPIOPin for GPIOPin {
    fn enable_output(&mut self) {
        volatile!(self.port.gper.set = self.pin_mask);
        volatile!(self.port.oder.set = self.pin_mask);
        volatile!(self.port.ster.clear = self.pin_mask);
    }

    fn read(&self) -> bool {
        (volatile!(self.port.pvr.val) & self.pin_mask) > 0
    }

    port_register_fn!(toggle, ovr, toggle);
    port_register_fn!(set, ovr, set);
    port_register_fn!(clear, ovr, clear);
}
