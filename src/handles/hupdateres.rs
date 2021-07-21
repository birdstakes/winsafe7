#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::enums::{IdStr, RtStr};
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;
use crate::structs::LANGID;
use crate::various::WString;

pub_struct_handle! {
	/// Handle to an
	/// [updateable resource](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew).
	/// Originally just a `HANDLE`.
	HUPDATERES
}

impl HUPDATERES {
	/// [`BeginUpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HUPDATERES::EndUpdateResource`](crate::HUPDATERES::EndUpdateResource)
	/// call.
	pub fn BeginUpdateResource(
		pFileName: &str, bDeleteExistingResources: bool) -> WinResult<HUPDATERES>
	{
		unsafe {
			kernel32::BeginUpdateResourceW(
				WString::from_str(pFileName).as_ptr(),
				bDeleteExistingResources as _,
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`EndUpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
	/// method.
	pub fn EndUpdateResource(self, fDiscard: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { kernel32::EndUpdateResourceW(self.ptr, fDiscard as _) },
		)
	}

	/// [`UpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// method.
	pub fn UpdateResource(self,
		lpType: RtStr, lpName: IdStr,
		wLanguage: LANGID, lpData: &[u8]) -> WinResult<()>
	{
		let mut buf_lpType = WString::default();
		let mut buf_lpName = WString::default();
		bool_to_winresult(
			unsafe {
				kernel32::UpdateResourceW(
					self.ptr,
					lpType.as_ptr(&mut buf_lpType),
					lpName.as_ptr(&mut buf_lpName),
					wLanguage.0,
					lpData.as_ptr() as _,
					lpData.len() as _,
				)
			},
		)
	}
}
