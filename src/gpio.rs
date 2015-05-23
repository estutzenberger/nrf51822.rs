/* ================================================================================ */
/* ================                      GPIO                      ================ */
/* ================================================================================ */


/**
  * @brief General purpose input and output. (GPIO)
  */

// typedef struct {                                    /*!< GPIO Structure                                                        */
//   __I  uint32_t  RESERVED0[321];
//   __IO uint32_t  OUT;                               /*!< Write GPIO port.                                                      */
//   __IO uint32_t  OUTSET;                            /*!< Set individual bits in GPIO port.                                     */
//   __IO uint32_t  OUTCLR;                            /*!< Clear individual bits in GPIO port.                                   */
//   __I  uint32_t  IN;                                !< Read GPIO port.                                                       
//   __IO uint32_t  DIR;                               /*!< Direction of GPIO pins.                                               */
//   __IO uint32_t  DIRSET;                            /*!< DIR set register.                                                     */
//   __IO uint32_t  DIRCLR;                            /*!< DIR clear register.                                                   */
//   __I  uint32_t  RESERVED1[120];
//   __IO uint32_t  PIN_CNF[32];                       /*!< Configuration of GPIO pins.                                           */
// } NRF_GPIO_Type;
use volatile::{RW, RO};

#[repr(C)]
pub struct Gpio {
	_0: [u32; 321],
	pub out: RW<out::Register>,
	pub out_set: RW<out_set::Register>,
	pub out_clr: RW<out_clr::Register>,
	pub input: RO<input::Register>,
	pub dir: RW<dir::Register>,
	pub dir_set: RW<dir_set::Register>,
	pub dir_clr: RW<dir_clr::Register>,
	_1: [u32; 120],
	pub pin_cnf: [RW<pin_cnf::Register>; 32],
}

reg!(out: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(out_set: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(out_clr: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(input: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(dir: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(dir_set: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(dir_clr: u32 {
	PIN0: 0,
	PIN1: 1,
	PIN2: 2,
	PIN3: 3,
	PIN4: 0,
	PIN5: 1,
	PIN6: 2,
	PIN7: 3,
	
	PIN8: 8,
	PIN9: 9,
	PIN10: 10,
	PIN11: 11,
	PIN12: 12,
	PIN13: 13,
	PIN14: 14,
	PIN15: 15,
	
	PIN116: 16,
	PIN17: 17,
	PIN18: 18,
	PIN19: 19,
	PIN20: 20,
	PIN21: 21,
	PIN22: 22,
	PIN23: 23,
	
	PIN24: 24,
	PIN25: 25,
	PIN26: 26,
	PIN27: 27,
	PIN28: 28,
	PIN29: 29,
	PIN30: 30,
	PIN31: 31,
});

reg!(pin_cnf: u32 {
	bits {
		DIR: 0,
		INPUT: 1,
	}
	bitfields {
		PULL: 2 {
			DISABLED: 0b00,
			PULLDOWN: 0b01,
			PULLUP: 0b11,
		},
		DRIVE: 8 {
			S0S1: 0b000,
			H0S1: 0b001,
			S0H1: 0b010,
			H0H1: 0b011,
			D0S1: 0b100,
			D0H1: 0b101,
			S0D1: 0b110,
			H0D1: 0b111,
		},
		SENSE: 16 {
			DISABLED: 0b00,
			HIGH: 0b10,
			LOW: 0b11,
		}
	}
});