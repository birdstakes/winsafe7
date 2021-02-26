use crate::aliases::WinResult;
use crate::co;
use crate::msg::{Message, Wm};
use crate::privs::{CB_ERR, CB_ERRSPACE};
use crate::WString;

/// [`CB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-addstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbAddString<'a> {
	pub text: &'a str,
}

impl<'a> Message for CbAddString<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR | CB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-deletestring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbDeleteString {
	pub index: u32,
}

impl Message for CbDeleteString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::DELETESTRING.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_DIR`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-dir)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbDir<'a> {
	pub attributes: co::DDL,
	pub path: &'a str,
}

impl<'a> Message for CbDir<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::DELETESTRING.into(),
			wparam: self.attributes.0 as usize,
			lparam: unsafe { WString::from_str(self.path).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_FINDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct CbFindString<'a> {
	pub preceding_index: Option<u32>,
	pub text: &'a str,
}

impl<'a> Message for CbFindString<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::FINDSTRING.into(),
			wparam: match self.preceding_index {
				None => -1,
				Some(idx) => idx as i32,
			} as usize,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbGetCount {}

impl Message for CbGetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct CbGetCurSel {}

impl Message for CbGetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETLBTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtext)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbGetLbText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> Message for CbGetLbText<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::GETLBTEXT.into(),
			wparam: self.index as usize,
			lparam: unsafe { self.text.as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETLBTEXTLEN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtextlen)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbGetLbTextLen {
	pub index: u32,
}

impl Message for CbGetLbTextLen {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::GETLBTEXTLEN.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETMINVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getminvisible)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct CbGetMinVisible {}

impl Message for CbGetMinVisible {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::GETMINVISIBLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-gettopindex)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbGetTopIndex {}

impl Message for CbGetTopIndex {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_INITSTORAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-initstorage)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct CbInitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl Message for CbInitStorage {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			n_items => Ok(n_items as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::INITSTORAGE.into(),
			wparam: self.num_items as usize,
			lparam: self.memory_bytes as isize,
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { CbResetContent, co::CB::RESETCONTENT.into(),
	/// [`CB_RESETCONTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-resetcontent)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`CB_SETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setcursel)
/// message parameters.
///
/// Return type: `()`.
pub struct CbSetCurSel {
	pub index: Option<u32>,
}

impl Message for CbSetCurSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::SETCURSEL.into(),
			wparam: match self.index {
				Some(index) => index as i32,
				None => -1,
			} as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_SETMINVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setminvisible)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct CbSetMinVisible {
	pub num_items: u32,
}

impl Message for CbSetMinVisible {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::SETMINVISIBLE.into(),
			wparam: self.num_items as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_SETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-settopindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct CbSetTopIndex {
	pub index: u32,
}

impl Message for CbSetTopIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::SETTOPINDEX.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_SHOWDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-showdropdown)
/// message parameters.
///
/// Return type: `()`.
pub struct CbShowDropDown {
	pub show: bool,
}

impl Message for CbShowDropDown {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::CB::SHOWDROPDOWN.into(),
			wparam: self.show as usize,
			lparam: 0,
		}
	}
}