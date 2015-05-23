extern {
	static __POWER__: ::power::Power;
	static __CLOCK__: ::clock::Clock;
	// static __MPU__: ::mpu::Mpu;
	// static __PU__: ::pu::Pu;
	// static __AMLI__: ::amli::Amli;
	// static __RADIO__: ::radio::Radio;
	// static __UART0__: ::uart::Uart;
	// static __SPI0__: ::spi::Spi;
	// static __TWI0__: ::twi::Twi;
	// static __SPI1__: ::spi::Spi;
	// static __TWI1__: ::twi::Twi;
	// static __SPI1S__: ::spis::Spis;
	// static __SPI1M__: ::spi::Spi;
	// static __GPIOTE__: ::gpiote::Gpiote;
	static __ADC__: ::adc::Adc;
	// static __TIMER0__: ::timer::Timer;
	// static __TIMER1__: ::timer::Timer;
	// static __TIMER2__: ::timer::Timer;
	static __RTC0__: ::rtc::Rtc;
	// static __TEMP__: ::temp::Temp;
	// static __RNG__: ::rng::Rng;
	// static __ECB__: ::ecb::Ecb;
	// static __AAR__: ::aar::Aar;
	// static __CCM__: ::ccm::Ccm;
	// static __WDT__: ::wdt::Wdt;
	// static __RTC1__: ::rtc::Rtc;
	// static __QDEC__: ::qdec::Qdec;
	// static __LPCOMP__: ::lpcomp::Lpcomp;
	// static __SWI__: ::swi::Swi;
	// static __NVMC__: ::nvmc::Nvmc;
	// static __PPI__: ::ppi::Ppi;
	static __FICR__: ::ficr::Ficr;
	static __UICR__: ::uicr::Uicr;
	static __GPIO__: ::gpio::Gpio;
}

// #define NRF_POWER_BASE                  0x40000000UL
pub fn power() -> &'static ::power::Power {
	&__POWER__
}
// #define NRF_CLOCK_BASE                  0x40000000UL
pub fn clock() -> &'static ::clock::Clock {
	&__CLOCK__
}
// #define NRF_MPU_BASE                    0x40000000UL
// pub fn mpu() -> &'static ::mpu::Mpu {
// 	&__MPU__
// }
// #define NRF_PU_BASE                     0x40000000UL
// pub fn pu() -> &'static ::pu::Pu {
// 	&__PU__
// }
// // #define NRF_AMLI_BASE                   0x40000000UL
// pub fn amli() -> &'static ::amli::Amli {
// 	&__AMLI__
// }
// // #define NRF_RADIO_BASE                  0x40001000UL
// pub fn radio() -> &'static ::radio::Radio {
// 	&__RADIO__
// }
// // #define NRF_UART0_BASE                  0x40002000UL
// pub fn uart0() -> &'static ::uart::Uart {
// 	&__UART0__
// }
// // #define NRF_SPI0_BASE                   0x40003000UL
// pub fn spi0() -> &'static ::spi::Spi {
// 	&__SPI0__
// }
// // #define NRF_TWI0_BASE                   0x40003000UL
// pub fn twi0() -> &'static ::twi::Twi {
// 	&__TWI0__
// }
// // #define NRF_SPI1_BASE                   0x40004000UL
// pub fn spi1() -> &'static ::spi::Spi {
// 	&__SPI1__
// }
// // #define NRF_TWI1_BASE                   0x40004000UL
// pub fn twi1() -> &'static ::twi::Twi {
// 	&__TWI1__
// }
// // #define NRF_SPIS1_BASE                  0x40004000UL
// pub fn spis1() -> &'static ::spis::Spis {
// 	&__SPIS1__
// }
// // #define NRF_SPIM1_BASE                  0x40004000UL
// pub fn spim1() -> &'static ::spi::Spi {
// 	&__SPIM1__
// }
// // #define NRF_GPIOTE_BASE                 0x40006000UL
// pub fn gpiote() -> &'static ::gpiote::Gpiote {
// 	&__GPIOTE__
// }
// #define NRF_ADC_BASE                    0x40007000UL
pub fn adc() -> &'static ::adc::Adc {
	&__ADC__
}
// // #define NRF_TIMER0_BASE                 0x40008000UL
// pub fn timer0() -> &'static ::timer::Timer {
// 	&__TIMER0__
// }
// // #define NRF_TIMER1_BASE                 0x40009000UL
// pub fn timer1() -> &'static ::timer::Timer {
// 	&__TIMER1__
// }
// // #define NRF_TIMER2_BASE                 0x4000A000UL
// pub fn timer2() -> &'static ::timer::Timer {
// 	&__TIMER2__
// }
// #define NRF_RTC0_BASE                   0x4000B000UL
pub fn rtc0() -> &'static ::rtc::Rtc {
	&__RTC0__
}
// // #define NRF_TEMP_BASE                   0x4000C000UL
// pub fn temp() -> &'static ::temp::Temp {
// 	&__TEMP__
// }
// // #define NRF_RNG_BASE                    0x4000D000UL
// pub fn rng() -> &'static ::rng::Rng {
// 	&__RNG__
// }
// // #define NRF_ECB_BASE                    0x4000E000UL
// pub fn ecb() -> &'static ::ecb::Ecb {
// 	&__ECB__
// }
// // #define NRF_AAR_BASE                    0x4000F000UL
// pub fn aar() -> &'static ::aar::Aar {
// 	&__AAR__
// }
// // #define NRF_CCM_BASE                    0x4000F000UL
// pub fn ccm() -> &'static ::ccm::Ccm {
// 	&__CCM__
// }
// // #define NRF_WDT_BASE                    0x40010000UL
// pub fn wdt() -> &'static ::wdt::Wdt {
// 	&__WDT__
// }
// // #define NRF_RTC1_BASE                   0x40011000UL
// pub fn rtc1() -> &'static ::rtc::Rtc {
// 	&__RTC__
// }
// // #define NRF_QDEC_BASE                   0x40012000UL
// pub fn qdec() -> &'static ::qdec::Qdec {
// 	&__QDEC__
// }
// // #define NRF_LPCOMP_BASE                 0x40013000UL
// pub fn lpcomp() -> &'static ::lpcomp::Lpcomp {
// 	&__LPCOMP__
// }
// // #define NRF_SWI_BASE                    0x40014000UL
// pub fn swi() -> &'static ::swi::Swi {
// 	&__SWI__
// }
// // #define NRF_NVMC_BASE                   0x4001E000UL
// pub fn nvmc() -> &'static ::nvmc::Nvmc {
// 	&__NVMC__
// }
// // #define NRF_PPI_BASE                    0x4001F000UL
// pub fn ppi() -> &'static ::ppi::Ppi {
// 	&__PPI__
// }
// #define NRF_FICR_BASE                   0x10000000UL
pub fn ficr() -> &'static ::ficr::Ficr {
	&__FICR__
}
// #define NRF_UICR_BASE                   0x10001000UL
pub fn uicr() -> &'static ::uicr::Uicr {
	&__UICR__
}
// #define NRF_GPIO_BASE                   0x50000000UL
pub fn gpio() -> &'static ::gpio::Gpio {
	&__GPIO__
}