use core::intrinsics;

#[repr(C, packed)]
struct Nvic {
    iser: [u32; 28]
}

#[repr(C)]
#[derive(Copy)]
pub enum NvicIdx {
    HFLASHC,
    PDCA0,
    PDCA1,
    PDCA2,
    PDCA3,
    PDCA4,
    PDCA5,
    PDCA6,
    PDCA7,
    PDCA8,
    PDCA9,
    PDCA10,
    PDCA11,
    PDCA12,
    PDCA13,
    PDCA14,
    PDCA15,
    CRCCU,
    USBC,
    PEVCTR,
    PEVCOV,
    AESA,
    PM,
    SCIF,
    FREQM,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
    GPIO10,
    GPIO11,
    BPM,
    BSCIF,
    ASTALARM,
    ASTPER,
    ASTOVF,
    ASTREADY,
    ASTCLKREADY,
    WDT,
    EIC1,
    EIC2,
    EIC3,
    EIC4,
    EIC5,
    EIC6,
    EIC7,
    EIC8,
    IISC,
    SPI,
    TC00,
    TC01,
    TC02,
    TC10,
    TC11,
    TC12,
    TWIM0,
    TWIS0,
    TWIM1,
    TWIS1,
    USART0,
    USART1,
    USART2,
    USART3,
    ADCIFE,
    DACC,
    ACIFC,
    TRNG,
    PARC,
    CATB,
    TWIM2,
    TWIM3,
    LCDCA
}

pub const BASE_ADDRESS : usize = 0xe000e100;

pub fn enable(signal: NvicIdx) {
    let nvic : &mut Nvic = unsafe { intrinsics::transmute(BASE_ADDRESS) };
    let interrupt = signal as usize;

    volatile!(nvic.iser[interrupt / 32] = 1 << (interrupt & 31));
}

