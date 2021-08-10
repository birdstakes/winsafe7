#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::structs::PROCESSENTRY32;

pub_struct_handle_closeable! {
	/// Handle to a process list
	/// [snapshot](https://docs.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes).
	/// Originally just a `HANDLE`.
	///
	/// # Examples
	///
	/// Listing the names of all processes currently running, along with their
	/// process ID and number of execution threads:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HPROCESSLIST, PROCESSENTRY32};
	///
	/// let mut pe = PROCESSENTRY32::default();
	/// let hpl = HPROCESSLIST::CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)
	///     .unwrap();
	///
	/// if hpl.Process32First(&mut pe).unwrap() {
	///     loop {
	///         println!("{} {} {}", pe.szExeFile(), pe.th32ProcessID, pe.cntThreads);
	///         if !hpl.Process32Next(&mut pe).unwrap() {
	///             break;
	///         }
	///     }
	/// }
	///
	/// hpl.CloseHandle().unwrap();
	/// ```
	HPROCESSLIST
}

impl HPROCESSLIST {
	/// [`CreateToolhelp32Snapshot`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HPROCESSLIST::CloseHandle`](crate::HPROCESSLIST::CloseHandle) call.
	pub fn CreateToolhelp32Snapshot(
		dwFlags: co::TH32CS,
		th32ProcessID: Option<u32>) -> WinResult<HPROCESSLIST>
	{
		unsafe {
			kernel32::CreateToolhelp32Snapshot(
				dwFlags.0,
				th32ProcessID.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`Process32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// method.
	pub fn Process32First(self, lppe: &mut PROCESSENTRY32) -> WinResult<bool> {
		match unsafe {
			kernel32::Process32FirstW(self.ptr, lppe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32Next`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw)
	/// method.
	pub fn Process32Next(self, lppe: &mut PROCESSENTRY32) -> WinResult<bool> {
		match unsafe {
			kernel32::Process32NextW(self.ptr, lppe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}