//! Copy & paste from https://github.com/MabezDev/xtensa-rust-quickstart
//! The watchdog timers need to be disabled to prevent the board from resetting all the time

use esp32_hal::target;
use esp32_hal::target::Peripherals;

const WDT_WKEY_VALUE: u32 = 0x50D83AA1;

pub fn disable_watchdog_timers(peripherals: &mut Peripherals) {
    disable_timg_wdts(&mut peripherals.TIMG0, &mut peripherals.TIMG1);
    disable_rtc_wdt(&mut peripherals.RTCCNTL);
}

fn disable_rtc_wdt(rtccntl: &mut target::RTCCNTL) {
    /* Disables the RTCWDT */
    rtccntl
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w.wdt_stg0()
            .bits(0x0)
            .wdt_stg1()
            .bits(0x0)
            .wdt_stg2()
            .bits(0x0)
            .wdt_stg3()
            .bits(0x0)
            .wdt_flashboot_mod_en()
            .clear_bit()
            .wdt_en()
            .clear_bit()
    });
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(0x0) });
}

fn disable_timg_wdts(timg0: &mut target::TIMG0, timg1: &mut target::TIMG1) {
    timg0
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    timg1
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });

    timg0.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
    timg1.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
}
