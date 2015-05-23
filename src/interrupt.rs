use core::option::Option::{None, Some, self};

extern crate cortex;

extern {
    /// Power clock interrupt
    pub fn power_clock();
    /// Radio interrupt
    pub fn radio();
    /// UART 0 interrupt
    pub fn uart0();
    /// SPI TWI 0 interrupt
    pub fn spi_twi_0();
    /// SPI TWI 1 interrput
    pub fn spi_twi_1();
    /// GPIOTE interrupt
    pub fn gpiote();
    /// ADC interrupt
    pub fn adc();
    /// Timer0 interrupt
    pub fn timer0();
    /// Timer1 interrupt
    pub fn timer1();
    /// Timer2 interrupt
    pub fn timer2();
    /// RTC0 interrupt
    pub fn rtc0();
    /// Temperature interrupt
    pub fn temp();
    /// Random number generator interrupt
    pub fn rng();
    /// ECB interrupt
    pub fn ecb();
    /// CCM AAR interrupt
    pub fn ccm_aar();
    /// Watchdog interrupt
    pub fn wdt();
    /// RTC 1 interrupt
    pub fn rtc1();
    /// QDEC interrupt
    pub fn qdec();
    /// LP Comparator interrupt
    pub fn lpcomp();
    /// SWI0 interrupt
    pub fn swi0();
    /// SWI1 interrupt
    pub fn swi1();
    /// SWI2 interrupt
    pub fn swi2();
    /// SWI3 interrupt
    pub fn swi3();
    /// SWI4 interrupt
    pub fn swi4();
    /// SWI5 interrupt
    pub fn swi5();
}

/// Interrupt "vector"
#[link_section=".interrupt_vector"]
pub static VECTOR: [Option<unsafe extern fn()>; 32] = [
    // OFFSET    HANDLER
    /* 0x0040 */ Some(power_clock),
    /* 0x0044 */ Some(radio),
    /* 0x0048 */ Some(uart0),
    /* 0x004C */ Some(spi_twi_0),
    /* 0x0050 */ Some(spi_twi_1),
    /* 0x0054 */ None,
    /* 0x0058 */ Some(gpiote),
    /* 0x005C */ Some(adc),
    /* 0x0060 */ Some(timer0),
    /* 0x0064 */ Some(timer1),
    /* 0x0068 */ Some(timer2),
    /* 0x006C */ Some(rtc0),
    /* 0x0070 */ Some(temp),
    /* 0x0074 */ Some(rng),
    /* 0x0078 */ Some(ecb),
    /* 0x007C */ Some(ccm_aar),
    /* 0x0080 */ Some(wdt),
    /* 0x0084 */ Some(rtc1),
    /* 0x0088 */ Some(qdec),
    /* 0x008C */ Some(lpcomp),
    /* 0x0090 */ Some(swi0),
    /* 0x0094 */ Some(swi1),
    /* 0x0098 */ Some(swi2),
    /* 0x009C */ Some(swi3),
    /* 0x00A0 */ Some(swi4),
    /* 0x00A4 */ Some(swi5),
    /* 0x00A8 */ None,
    /* 0x00AC */ None,
    /* 0x00B0 */ None,
    /* 0x00B4 */ None,
    /* 0x00B8 */ None,
    /* 0x00BC */ None,
];

pub enum IntVector {
    RTC0Irqn = 11,
}
