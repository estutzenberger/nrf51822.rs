use volatile::{RW, RO, WO};

/* ================================================================================ */
/* ================                       RTC                      ================ */
/* ================================================================================ */


/**
  * @brief Real time counter 0. (RTC)
  */

// typedef struct {                                    /*!< RTC Structure                                                         */
//   __O  uint32_t  TASKS_START;                       /*!< Start RTC Counter.                                                    */
//   __O  uint32_t  TASKS_STOP;                        /*!< Stop RTC Counter.                                                     */
//   __O  uint32_t  TASKS_CLEAR;                       /*!< Clear RTC Counter.                                                    */
//   __O  uint32_t  TASKS_TRIGOVRFLW;                  /*!< Set COUNTER to 0xFFFFFFF0.                                            */
//   __I  uint32_t  RESERVED0[60];
//   __IO uint32_t  EVENTS_TICK;                       /*!< Event on COUNTER increment.                                           */
//   __IO uint32_t  EVENTS_OVRFLW;                     /*!< Event on COUNTER overflow.                                            */
//   __I  uint32_t  RESERVED1[14];
//   __IO uint32_t  EVENTS_COMPARE[4];                 /*!< Compare event on CC[n] match.                                         */
//   __I  uint32_t  RESERVED2[109];
//   __IO uint32_t  INTENSET;                          /*!< Interrupt enable set register.                                        */
//   __IO uint32_t  INTENCLR;                          /*!< Interrupt enable clear register.                                      */
//   __I  uint32_t  RESERVED3[13];
//   __IO uint32_t  EVTEN;                             /*!< Configures event enable routing to PPI for each RTC event.            */
//   __IO uint32_t  EVTENSET;                          /*!< Enable events routing to PPI. The reading of this register gives
//                                                          the value of EVTEN.                                                   */
//   __IO uint32_t  EVTENCLR;                          /*!< Disable events routing to PPI. The reading of this register
//                                                          gives the value of EVTEN.                                             */
//   __I  uint32_t  RESERVED4[110];
//   __I  uint32_t  COUNTER;                           /*!< Current COUNTER value.                                                */
//   __IO uint32_t  PRESCALER;                         /*!< 12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).
//                                                          Must be written when RTC is STOPed.                                   */
//   __I  uint32_t  RESERVED5[13];
//   __IO uint32_t  CC[4];                             /*!< Capture/compare registers.                                            */
//   __I  uint32_t  RESERVED6[683];
//   __IO uint32_t  POWER;                             /*!< Peripheral power control.                                             */
// } NRF_RTC_Type;

#[repr(C)]
pub struct Rtc {
    pub tasks_start: WO<u32>,
    pub tasks_stop: WO<u32>,
    pub tasks_clear: WO<u32>,
    pub tasks_trigovrflw: WO<u32>,
    _0: [u32; 60],
    pub events_tick: RW<u32>,
    pub events_ovrflw: RW<u32>,
    _1: [u32; 14],
    pub events_compare0: RW<u32>,
    pub events_compare1: RW<u32>,
    pub events_compare2: RW<u32>,
    pub events_compare3: RW<u32>,
    _2: [u32; 108],
    pub inten: RW<inten::Register>,
    pub intenset: RW<intenset::Register>,
    pub intenclr: RW<intenclr::Register>,
    _3: [u32; 13],
    pub evten: RW<evten::Register>,
    pub evtenset: RW<evtenset::Register>,
    pub evtenclr: RW<evtenclr::Register>,
    _4: [u32; 110],
    pub counter: RO<u32>,
    pub prescaler: RW<u32>,
    _5: [u32; 13],
    pub cc0: RW<u32>,
    pub cc1: RW<u32>,
    pub cc2: RW<u32>,
    pub cc3: RW<u32>,
    _6: [u32; 683],
    pub power: RW<u32>,
}

reg!(inten: u32 {
    TICK: 0,
    OVRFLW: 1,
    COMPARE0: 16,
    COMPARE1: 17,
    COMPARE2: 18,
    COMPARE3: 19,
});

reg!(intenset: u32 {
    TICK: 0,
    OVRFLW: 1,
    COMPARE0: 16,
    COMPARE1: 17,
    COMPARE2: 18,
    COMPARE3: 19,
});

reg!(intenclr: u32 {
    TICK: 0,
    OVRFLW: 1,
    COMPARE0: 16,
    COMPARE1: 17,
    COMPARE2: 18,
    COMPARE3: 19,
});

reg!(evten: u32 {
    TICK: 0,
    OVRFLW: 1,
    COMPARE0: 16,
    COMPARE1: 17,
    COMPARE2: 18,
    COMPARE3: 19,
});

reg!(evtenset: u32 {
    TICK: 0,
    OVRFLW: 1,
    COMPARE0: 16,
    COMPARE1: 17,
    COMPARE2: 18,
    COMPARE3: 19,
});

reg!(evtenclr: u32 {
    TICK: 0,
    OVRFLW: 1,
    COMPARE0: 16,
    COMPARE1: 17,
    COMPARE2: 18,
    COMPARE3: 19,
});