/* ================================================================================ */
/* ================                      FICR                      ================ */
/* ================================================================================ */


/**
  * @brief Factory Information Configuration. (FICR)
  */

// typedef struct {                                    /*!< FICR Structure                                                        */
//   __I  uint32_t  RESERVED0[4];
//   __I  uint32_t  CODEPAGESIZE;                      /*!< Code memory page size in bytes.                                       */
//   __I  uint32_t  CODESIZE;                          /*!< Code memory size in pages.                                            */
//   __I  uint32_t  RESERVED1[4];
//   __I  uint32_t  CLENR0;  dep                          /*!< Length of code region 0 in bytes.                                     */
//   __I  uint32_t  PPFC;    dep                          /*!< Pre-programmed factory code present.                                  */
//   __I  uint32_t  RESERVED2;
//   __I  uint32_t  NUMRAMBLOCK;                       /*!< Number of individualy controllable RAM blocks.                        */
  
//   union {
//     __I  uint32_t  SIZERAMBLOCK[4];                 /*!< Deprecated array of size of RAM block in bytes. This name is
//                                                          kept for backward compatinility purposes. Use SIZERAMBLOCKS
//                                                           instead.                                                             */
//     __I  uint32_t  SIZERAMBLOCKS;                   /*!< Size of RAM blocks in bytes.                                          */
//   };
//   __I  uint32_t  RESERVED3[5];
//   __I  uint32_t  CONFIGID;                          /*!< Configuration identifier.                                             */
//   __I  uint32_t  DEVICEID[2];                       /*!< Device identifier.                                                    */
//   __I  uint32_t  RESERVED4[6];
//   __I  uint32_t  ER[4];                             /*!< Encryption root.                                                      */
//   __I  uint32_t  IR[4];                             /*!< Identity root.                                                        */
//   __I  uint32_t  DEVICEADDRTYPE;                    /*!< Device address type.                                                  */
//   __I  uint32_t  DEVICEADDR[2];                     /*!< Device address.                                                       */
//   __I  uint32_t  OVERRIDEEN;                        /*!< Radio calibration override enable.                                    */
//   __I  uint32_t  NRF_1MBIT[5];                      /*!< Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit
//                                                          mode.                                                                 */
//   __I  uint32_t  RESERVED5[10];
//   __I  uint32_t  BLE_1MBIT[5];                      /*!< Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit
//                                                          mode.                                                                 */
// } NRF_FICR_Type;
use volatile::RO;

#[repr(C)]
pub struct Ficr {
	_0: [u32; 4],
	pub code_page_size: RO<u32>,
	pub code_size: RO<u32>,
	_1: [u32; 4],
	clenr0: RO<u32>,
	ppfc: RO<u32>,
	_2: u32,
	pub num_ram_block: RO<u32>,
	pub size_ram_blocks: RO<u32>,
	_3: [u32; 5],
	pub config_id: RO<u32>,
	pub device_id: [RO<u32>; 2],
	_4: [u32; 6],
	pub er: [RO<u32>; 4],
	pub ir: [RO<u32>; 4],
	pub device_addr_type: RO<device_addr_type::Register>,
	pub device_addr: [RO<u32>; 2],
	pub override_en: RO<override_en::Register>,
	pub nrf_1mbit: [RO<u32>; 5],
	_5: [u32; 10],
	pub ble_1mbit: [RO<u32>; 5],
}

reg!(device_addr_type: u32 {
	DEVICEADDRTYPE: 0,
});

reg!(override_en: u32 {
	NRF1MBIT: 0,
	BLE1MBIT: 3,
});