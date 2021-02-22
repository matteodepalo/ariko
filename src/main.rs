#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate sam3x8e;

use cortex_m_rt::entry;
use sam3x8e::RTT;

fn delay_ms(rtt: &RTT, ms: u32) {
    // We're not considering overflow here, 32 bits can keep about 49 days in ms
    let now = rtt.vr.read().bits();
    let until = now + ms;

    while rtt.vr.read().bits() < until {}
}

#[entry]
fn main() -> ! {
    let p = sam3x8e::Peripherals::take().unwrap();

    // Configure RTT resolution to approx. 1ms
    let rtt = p.RTT;
    rtt.mr.write_with_zero(|w| unsafe { w.rtpres().bits(0x20) });

    let pioa = p.PIOA;
    let uart = p.UART;

    pioa.pdr.write_with_zero(|w| w.p8().set_bit());
    pioa.pdr.write_with_zero(|w| w.p9().set_bit());
    pioa.absr.write_with_zero(|w| w.p8().clear_bit());
    pioa.absr.write_with_zero(|w| w.p9().clear_bit());
    uart.cr
        .write_with_zero(|w| w.rstrx().set_bit().rsttx().set_bit());
    uart.brgr.write_with_zero(|w| unsafe { w.cd().bits(25) });
    uart.mr.write_with_zero(|w| w.par().no());
    uart.cr
        .write_with_zero(|w| w.rxen().set_bit().txen().set_bit());

    loop {
        let string = "ciao".as_bytes();
        for char in string {
            uart.thr
                .write_with_zero(|w| unsafe { w.txchr().bits(*char) });
            delay_ms(&rtt, 1)
        }
        delay_ms(&rtt, 1000)
    }
}
