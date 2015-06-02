use volatile::{WO};

#[repr(C)]
pub struct Wdt
{
	pub tasks_start: WO<u32>,
}