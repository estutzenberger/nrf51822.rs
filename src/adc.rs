/* ================================================================================ */
/* ================                       ADC                      ================ */
/* ================================================================================ */


/**
  * @brief Analog to digital converter. (ADC)
  */

// typedef struct {                                    /*!< ADC Structure                                                         */
//   __O  uint32_t  TASKS_START;                       /*!< Start an ADC conversion.                                              */
//   __O  uint32_t  TASKS_STOP;                        /*!< Stop ADC.                                                             */
//   __I  uint32_t  RESERVED0[62];
//   __IO uint32_t  EVENTS_END;                        /*!< ADC conversion complete.                                              */
//   __I  uint32_t  RESERVED1[128];
//   __IO uint32_t  INTENSET;                          /*!< Interrupt enable set register.                                        */
//   __IO uint32_t  INTENCLR;                          !< Interrupt enable clear register.                                      
//   __I  uint32_t  RESERVED2[61];
//   __I  uint32_t  BUSY;                              /*!< ADC busy register.                                                    */
//   __I  uint32_t  RESERVED3[63];
//   __IO uint32_t  ENABLE;                            /*!< ADC enable.                                                           */
//   __IO uint32_t  CONFIG;                            /*!< ADC configuration register.                                           */
//   __I  uint32_t  RESULT;                            /*!< Result of ADC conversion.                                             */
//   __I  uint32_t  RESERVED4[700];
//   __IO uint32_t  POWER;                             /*!< Peripheral power control.                                             */
// } NRF_ADC_Type;


use volatile::{RW, RO, WO};

#[repr(C)]
pub struct Adc {
	pub tasks_start: WO<u32>,
	pub tasks_stop: WO<u32>,
	_0: [u32; 62],
	pub events_end: RW<u32>,
	_1: [u32; 128],
	pub intenset: RW<intenset::Register>,
	pub intenclr: RW<intenclr::Register>,
	_2: [u32; 61],
	pub busy: RO<busy::Register>,
	_3: [u32; 63],
	pub enable: RW<enable::Register>,
	pub config: RW<config::Register>,
	pub result: RO<u32>,
	_4: [u32; 400],
	pub power: RW<u32>,
}

reg!(intenset: u32 {
	END: 0,
});

reg!(intenclr: u32 {
	END: 0,
});

reg!(busy: u32 {
	BUSY: 0,
});

reg!(enable: u32 {
	ENABLE: 0,
});

reg!(config: u32 {
	bitfields {
		RES: 0 {
			BITS8: 0b00,
			BITS9: 0b01,
			BITS10: 0b10,
		},
		INPSEL: 2 {
			AINOPRE: 0b000,
			AITTPRE: 0b001,
			AIOTPRE: 0b010,
			STTPRE: 0b101,
			SOTPRE: 0b110,
		},
		REFSEL: 5 {
			VBG: 0b00,
			EXT: 0b01,
			SOHPRE: 0b10,
			SOTPRE: 0b11,
		},
		PSEL: 8 {
			DISABLED: 0b00000000,
			AINPUT0: 0b00000001,
			AINPUT1: 0b00000010,
			AINPUT2: 0b00000100,
			AINPUT3: 0b00001000,
			AINPUT4: 0b00010000,
			AINPUT5: 0b00100000,
			AINPUT6: 0b01000000,
			AINPUT7: 0b10000000,
		},
		EXTREFSEL: 16 {
			NONE: 0b00,
			AREF0: 0b01,
			AREF1: 0b10,
		},
	}
});