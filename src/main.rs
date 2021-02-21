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

    // Enable PIOA
    let pmc = p.PMC;
    pmc.pmc_pcer0.write_with_zero(|w| w.pid8().set_bit());

    // Configure RTT resolution to approx. 1ms
    let rtt = p.RTT;
    rtt.mr.write_with_zero(|w| unsafe { w.rtpres().bits(0x20) });

    let pioa = p.PIOA;
    let uart = p.UART;

    pioa.pdr.write_with_zero(|w| w.p8().set_bit());
    pioa.pdr.write_with_zero(|w| w.p9().set_bit());
    pioa.absr.write_with_zero(|w| w.p8().set_bit());
    pioa.absr.write_with_zero(|w| w.p9().set_bit());
    pioa.puer.write_with_zero(|w| w.p8().set_bit());
    pioa.puer.write_with_zero(|w| w.p9().set_bit());
    uart.cr.write_with_zero(|w| w.rstrx().set_bit().rsttx().set_bit());
    uart.brgr.write_with_zero(|w| unsafe { w.cd().bits(546) });
    uart.mr.write_with_zero(|w| w.par().no());
    uart.cr.write_with_zero(|w| w.rxen().set_bit().txen().set_bit());

    loop {
        let a = "a".as_bytes()[0];
        loop {
            if uart.sr.read().rxrdy().bit() { break }
        }
        uart.thr.write_with_zero(|w| unsafe { w.txchr().bits(a) });
        delay_ms(&rtt, 1000);
    }
}
