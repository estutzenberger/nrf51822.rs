/**
  * @brief Clock control. (CLOCK)
  */

// typedef struct {                                    /*!< CLOCK Structure                                                       */
//   __O  uint32_t  TASKS_HFCLKSTART;                  /*!< Start HFCLK clock source.                                             */
//   __O  uint32_t  TASKS_HFCLKSTOP;                   /*!< Stop HFCLK clock source.                                              */
//   __O  uint32_t  TASKS_LFCLKSTART;                  /*!< Start LFCLK clock source.                                             */
//   __O  uint32_t  TASKS_LFCLKSTOP;                   /*!< Stop LFCLK clock source.                                              */
//   __O  uint32_t  TASKS_CAL;                         /*!< Start calibration of LFCLK RC oscillator.                             */
//   __O  uint32_t  TASKS_CTSTART;                     /*!< Start calibration timer.                                              */
//   __O  uint32_t  TASKS_CTSTOP;                      /*!< Stop calibration timer.                                               */
//   __I  uint32_t  RESERVED0[57];
//   __IO uint32_t  EVENTS_HFCLKSTARTED;               /*!< HFCLK oscillator started.                                             */
//   __IO uint32_t  EVENTS_LFCLKSTARTED;               /*!< LFCLK oscillator started.                                             */
//   __I  uint32_t  RESERVED1;
//   __IO uint32_t  EVENTS_DONE;                       /*!< Calibration of LFCLK RC oscillator completed.                         */
//   __IO uint32_t  EVENTS_CTTO;                       /*!< Calibration timer timeout.                                            */
//   __I  uint32_t  RESERVED2[124];
//   __IO uint32_t  INTENSET;                          /*!< Interrupt enable set register.                                        */
//   __IO uint32_t  INTENCLR;                          /*!< Interrupt enable clear register.                                      */
//   __I  uint32_t  RESERVED3[63];
//   __I  uint32_t  HFCLKRUN;                          /*!< Task HFCLKSTART trigger status.                                       */
//   __I  uint32_t  HFCLKSTAT;                         /*!< High frequency clock status.                                          */
//   __I  uint32_t  RESERVED4;
//   __I  uint32_t  LFCLKRUN;                          /*!< Task LFCLKSTART triggered status.                                     */
//   __I  uint32_t  LFCLKSTAT;                         /*!< Low frequency clock status.                                           */
//   __I  uint32_t  LFCLKSRCCOPY;                      /*!< Clock source for the LFCLK clock, set when task LKCLKSTART is
//                                                          triggered.                                                            */
//   __I  uint32_t  RESERVED5[62];
//   __IO uint32_t  LFCLKSRC;                          /*!< Clock source for the LFCLK clock.                                     */
//   __I  uint32_t  RESERVED6[7];
//   __IO uint32_t  CTIV;                              /*!< Calibration timer interval.                                           */
//   __I  uint32_t  RESERVED7[5];
//   __IO uint32_t  XTALFREQ;                          /*!< Crystal frequency.                                                    */
// } NRF_CLOCK_Type;

use volatile::{RW, RO, WO};

#[repr(C)]
pub struct Clock {
    pub tasks_hfclkstart: WO<u32>,
    pub tasks_hfclkstop: WO<u32>,
    pub tasks_lfclkstart: WO<u32>,
    pub tasks_lfclkstop: WO<u32>,
    pub tasks_cal: WO<u32>,
    pub tasks_ctstart: WO<u32>,
    pub tasks_ctstop: WO<u32>,
    _0: [u32; 57],
    pub events_hfclkstarted: RW<u32>,
    pub events_lfclkstarted: RW<u32>,
    _1: u32,
    pub events_done: RW<u32>,
    pub events_ctto: RW<u32>,
    _2: [u32; 124],
    pub intenset: RW<intenset::Register>,
    pub intenclr: RW<intenclr::Register>,
    _3: [u32; 63],
    pub hfclkrun: RO<hfclkrun::Register>,
    pub hfclkstat: RO<hfclkstat::Register>,
    _4: u32,
    pub lfclkrun: RO<lfclkrun::Register>,
    pub lfclkstat: RO<lfclkstat::Register>,
    pub lfclksrccopy: RO<lfclksrccopy::Register>,
    _5: [u32; 62],
    pub lfclksrc: RW<lfclksrc::Register>,
    _6: [u32; 7],
    pub ctiv: RW<u8>,
    _ctiv: [u8; 3],
    _7: [u32; 5],
    pub xtalfreq: RW<xtalfreq::Register>,
}

reg!(intenset: u32 {
    HFCLKSTARTED: 0,
    LFCLKSTARTED: 1,
    DONE: 3,
    CTTO: 4,
});

reg!(intenclr: u32 {
    HFCLKSTARTED: 0,
    LFCLKSTARTED: 1,
    DONE: 3,
    CTTO: 4,
});

reg!(hfclkrun: u32 {
    STATUS: 0,
});

reg!(hfclkstat: u32 {
    SRC: 0,
    STATE: 16,
});

reg!(lfclkrun: u32 {
    STATUS: 0,
});

reg!(lfclkstat: u32 {
    bits {
        STATE: 16,
    }
    bitfields {
        SRC: 0 {
            RC: 0b00,
            XTAL: 0b01,
            SYNTH: 0b10,
        }
    }
});

reg!(lfclksrccopy: u32 {
    // bits {

    // }
    bitfields {
        SRC: 0 {
            RC: 0b00,
            XTAL: 0b01,
            SYNTH: 0b10,
        },    
    }
});

reg!(lfclksrc: u32 {
    bitfields {
        SRC: 0 {
            RC: 0b00,
            XTAL: 0b01,
            SYNTH: 0b10,
        },
    }
});

reg!(xtalfreq: u32 {
    bitfields {
        XTALFREQ: 0 {
            MHZ_16: 0xFF,
            MHZ_32: 0x00,
        },
    }
});
