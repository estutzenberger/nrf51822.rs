/* ================================================================================ */
/* ================                      UICR                      ================ */
/* ================================================================================ */


/**
  * @brief User Information Configuration. (UICR)
  */

// typedef struct {                                    /*!< UICR Structure                                                        */
//   __IO uint32_t  CLENR0;                            /*!< Length of code region 0.                                              */
//   __IO uint32_t  RBPCONF;                           /*!< Readback protection configuration.                                    */
//   __IO uint32_t  XTALFREQ;                          /*!< Reset value for CLOCK XTALFREQ register.                              */
//   __I  uint32_t  RESERVED0;
//   __I  uint32_t  FWID;                              !< Firmware ID.                                                          
  
//   union {
//     __IO uint32_t  NRFFW[15];                       /*!< Reserved for Nordic firmware design.                                  */
//     __IO uint32_t  BOOTLOADERADDR;                  /*!< Bootloader start address.                                             */
//   };
//   __IO uint32_t  NRFHW[12];                         /*!< Reserved for Nordic hardware design.                                  */
//   __IO uint32_t  CUSTOMER[32];                      /*!< Reserved for customer.                                                */
// } NRF_UICR_Type;

use volatile::{RW};

#[repr(C)]

pub struct Uicr {
	pub clenr0: RW<u32>,
	pub rbpconf: RW<rbpconf::Register>,
	pub xtalfreq: RW<xtalfreq::Register>,
	_0: u32,
	pub fwid: RW<u32>,
	pub bootloader_addr: RW<u32>,
	nrffw: [RW<u32>; 12],
	customer: [RW<u32>; 32],
}

reg!(rbpconf: u32 {
	bitfields {
		PR0: 0 {
			DISABLED: 0xFF,
			ENABLED: 0x00,
		},
		PALL: 8 {
			DISABLED: 0xFF,
			ENABLED: 0x00,
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