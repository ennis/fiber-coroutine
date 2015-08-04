
extern crate winapi;
use self::winapi::{BOOL, SIZE_T};
pub use self::winapi::{LPVOID};

#[link(name = "kernel32")]
extern "stdcall" {
	pub fn ConvertFiberToThread() -> BOOL;
	pub fn ConvertThreadToFiber(lpParameter: LPVOID) -> LPVOID;
	pub fn CreateFiber(dwStackSize: SIZE_T, lpStartAddress: LPVOID, lpParameter: LPVOID) -> LPVOID;
	pub fn IsThreadAFiber() -> BOOL;
	pub fn SwitchToFiber(lpFiber: LPVOID);
}

extern {
	pub fn GetFiberData() -> LPVOID;
	pub fn GetCurrentFiber() -> LPVOID;
}
