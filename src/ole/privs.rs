#![allow(dead_code)]

use crate::co;
use crate::ffi_types::HRES;
use crate::ole::decl::HrResult;

/// If value is `S_OK` yields `Ok()`, othersize `Err(hresult)`.
pub(crate) fn ok_to_hrresult(hr: HRES) -> HrResult<()> {
	match co::HRESULT(hr) {
		co::HRESULT::S_OK => Ok(()),
		hr => Err(hr),
	}
}

/// If value is `S_OK` yields `Ok(true)`, if `S_FALSE` yields `Ok(false)`
/// othersize `Err(hresult)`.
pub(crate) fn okfalse_to_hrresult(hr: HRES) -> HrResult<bool> {
	match co::HRESULT(hr) {
		co::HRESULT::S_OK => Ok(true),
		co::HRESULT::S_FALSE => Ok(false),
		hr => Err(hr),
	}
}
