//! Power Control. (POWER)

// typedef struct {                                    /*!< POWER Structure                                                       */
//   __I  uint32_t  RESERVED0[30];
//   __O  uint32_t  TASKS_CONSTLAT;                    /*!< Enable constant latency mode.                                         */
//   __O  uint32_t  TASKS_LOWPWR;                      /*!< Enable low power mode (variable latency).                             */
//   __I  uint32_t  RESERVED1[34];
//   __IO uint32_t  EVENTS_POFWARN;                    /*!< Power failure warning.                                                */
//   __I  uint32_t  RESERVED2[126];
//   __IO uint32_t  INTENSET;                          /*!< Interrupt enable set register.                                        */
//   __IO uint32_t  INTENCLR;                          /*!< Interrupt enable clear register.                                      */
//   __I  uint32_t  RESERVED3[61];
//   __IO uint32_t  RESETREAS;                         /*!< Reset reason.                                                         */
//   __I  uint32_t  RESERVED4[9];
//   __I  uint32_t  RAMSTATUS;                         /*!< Ram status register.                                                  */
//   __I  uint32_t  RESERVED5[53];
//   __O  uint32_t  SYSTEMOFF;                         /*!< System off register.                                                  */
//   __I  uint32_t  RESERVED6[3];
//   __IO uint32_t  POFCON;                            /*!< Power failure configuration.                                          */
//   __I  uint32_t  RESERVED7[2];
//   __IO uint32_t  GPREGRET;                          /*!< General purpose retention register. This register is a retained
//                                                          register.                                                             */
//   __I  uint32_t  RESERVED8;
//   __IO uint32_t  RAMON;                             /*!< Ram on/off.                                                           */
//   __I  uint32_t  RESERVED9[7];
//   __IO uint32_t  RESET;                             /*!< Pin reset functionality configuration register. This register
//                                                          is a retained register.                                               */
//   __I  uint32_t  RESERVED10[3];
//   __IO uint32_t  RAMONB;                            /*!< Ram on/off.                                                           */
//   __I  uint32_t  RESERVED11[8];
//   __IO uint32_t  DCDCEN;                            /*!< DCDC converter enable configuration register.                         */
//   __I  uint32_t  RESERVED12[291];
//   __IO uint32_t  DCDCFORCE;                         /*!< DCDC power-up force register.                                         */
// } NRF_POWER_Type;

use volatile::{RW, WO};

#[repr(C)]
pub struct Power {
    _0: [u32; 30],
    pub tasks_constlat: WO<u32>,
    pub tasks_lowpwr: WO<u32>,
    _1: [u32; 34],
    pub events_pofwarn: RW<u32>,
    _2: [u32; 126],
    pub intenset: RW<intenset::Register>,
    pub intenclr: RW<intenclr::Register>,
    _3: [u32; 61],
    pub resetreas: RW<resetreas::Register>,
    _4: [u32; 9],
    pub ramstatus: RW<ramstatus::Register>,
    _5: [u32; 53],
    pub systemoff: RW<systemoff::Register>,
    _6: [u32; 3],
    pub pofcon: RW<pofcon::Register>,
    _7: [u32; 2],
    pub gpregret: RW<u8>,
    _gp: [u8; 3],
    _8: u32,
    pub ramon: RW<ramon::Register>,
    _9: [u32; 7],
    pub reset: RW<reset::Register>,
    _10: [u32; 3],
    pub ramonb: RW<ramonb::Register>,
    _11: [u32; 8],
    pub dcdcen: RW<dcdcen::Register>,
    _12: [u32; 291],
    pub dcdcforce: RW<u32>,
}

reg!(intenset: u32 {
    POFWARN: 2,
});

reg!(intenclr: u32{
    POFWARN: 2,
});

reg!(resetreas: u32 {
    RESETPIN: 0,
    DOG: 1,
    SREQ: 2,
    LOCKUP: 3,
    OFF: 16,
    LPCOMP: 17,
    DIF: 18,
});

reg!(ramstatus: u32 {
    RAMBLOCK0: 0,
    RAMBLOCK1: 1,
    RAMBLOCK2: 2,
    RAMBLOCK3: 3,
});

reg!(systemoff: u32 {
    SYSTEMOFF: 0,
});

reg!(pofcon: u32 {
    bits {
        POF: 0,
    }
    bitfields {
        THRESHOLD: 1 {
            V21: 0b00,
            V23: 0b01,
            V25: 0b10,
            V27: 0b11,
        }
    }
});

reg!(ramon: u32 {
    ONRAM0: 0,
    ONRAM1: 1,
    OFFRAM1: 16,
    OFFRAM0: 17,
});

reg!(reset: u32 {
    RESET: 0,
});

reg!(ramonb: u32 {
    ONRAM2: 0,
    ONRAM3: 1,
    OFFRAM2: 16,
    OFFRAM3: 17,
});

reg!(dcdcen: u32 {
    DCDCEN: 1,
});
