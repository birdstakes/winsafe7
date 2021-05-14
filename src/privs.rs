//! Internal definitions used by the library.

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{BOOL, HRESULT, PCVOID, PVOID};
use crate::funcs::GetLastError;
use crate::WString;

pub(crate) const CB_ERR: i32 = -1;
pub(crate) const CB_ERRSPACE: i32 = -2;
pub(crate) const CLR_INVALID: u32 = 0xffff_ffff;
pub(crate) const FAPPCOMMAND_MASK: u16 = 0xf000;
pub(crate) const GDI_ERROR: u32 = 0xffff_ffff;
pub(crate) const GDT_ERROR: i32 = -1;
pub(crate) const GMEM_INVALID_HANDLE: u32 = 0x8000;
pub(crate) const INFINITE: u32 = 0xffff_ffff;
pub(crate) const INVALID_FILE_ATTRIBUTES: i32 = -1;
pub(crate) const INVALID_HANDLE_VALUE: isize = -1;
pub(crate) const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub(crate) const LB_ERR: i32 = -1;
pub(crate) const LB_ERRSPACE: i32 = -2;
pub(crate) const LF_FACESIZE: usize = 32;
pub(crate) const MAX_LINKID_TEXT: usize = 48;
pub(crate) const MAX_PATH: usize = 260;

/// Converts a const reference to `ffi::PCVOID`.
pub(crate) fn ref_as_pcvoid<T>(r: &T) -> PCVOID {
	r as *const _ as _
}

/// Converts a mut reference to `ffi::PVOID`.
pub(crate) fn ref_as_pvoid<T>(r: &mut T) -> PVOID {
	r as *mut _ as _
}

/// Converts a `BOOL` value to a `WinResult`. `TRUE` is `Ok(())`, `FALSE` is
/// `Err(GetLastError())`.
pub(crate) fn bool_to_winresult(expr: BOOL) -> WinResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// Converts a native `HRESULT` to `WinResult`, `S_OK` yielding `Ok` and
/// anything else yielding `Err`.
pub(crate) fn hr_to_winresult(hresult: HRESULT) -> WinResult<()> {
	match co::ERROR(hresult as _) {
		co::ERROR::S_OK => Ok(()),
		err => Err(err),
	}
}

/// Converts a native `HRESULT` to `WinResult`, `S_OK` yielding `Ok(true)`,
/// `S_FALSE` yielding `Ok(false)` and anything else yielding `Err`.
pub(crate) fn hr_to_winresult_bool(hresult: HRESULT) -> WinResult<bool> {
	match co::ERROR(hresult as _) {
		co::ERROR::S_OK => Ok(true),
		co::ERROR::S_FALSE => Ok(false),
		err => Err(err),
	}
}

/// If number is zero, yields `Err(GetLastError)`, otherwise `Ok(expr)`.
pub(crate) fn nonzero_to_winresult(expr: i32) -> WinResult<i32> {
	match expr {
		0 => Err(GetLastError()),
		val => Ok(val),
	}
}

/// Parses a null-delimited multi-string, which must terminate with two nulls.
pub(crate) fn parse_multi_z_str(src: *const u16) -> Vec<String> {
	let mut src = src;
	let mut strings = Vec::default();
	let mut i = 0;

	loop {
		if unsafe { *src.add(i) } == 0 {
			let slice = unsafe { std::slice::from_raw_parts(src, i) };
			if slice.is_empty() {
				break;
			}
			strings.push(WString::from_wchars_slice(slice).to_string());
			src = unsafe { src.add(i + 1) };
			i = 0;
		} else {
			i += 1;
		}
	}
	strings
}