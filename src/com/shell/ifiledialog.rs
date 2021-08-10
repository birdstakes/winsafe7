#![allow(non_snake_case)]

use crate::com::shell::vt::IModalWindowVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCSTR, PCVOID, PSTR, PVOID};
use crate::structs::IID;

/// [`IFileDialog`](crate::shell::IFileDialog) virtual table.
pub struct IFileDialogVT {
	pub IModalWindowVT: IModalWindowVT,
	pub SetFileTypes: fn(PPVT, u32, PCVOID) -> HRESULT,
	pub SetFileTypeIndex: fn(PPVT, u32) -> HRESULT,
	pub GetFileTypeIndex: fn(PPVT, *mut u32) -> HRESULT,
	pub Advise: fn(PPVT, PVOID, *mut u32) -> HRESULT,
	pub Unadvise: fn(PPVT, u32) -> HRESULT,
	pub SetOptions: fn(PPVT, u32) -> HRESULT,
	pub GetOptions: fn(PPVT, *mut u32) -> HRESULT,
	pub SetDefaultFolder: fn(PPVT, PPVT) -> HRESULT,
	pub SetFolder: fn(PPVT, PPVT) -> HRESULT,
	pub GetFolder: fn(PPVT, *mut PVOID) -> HRESULT,
	pub GetCurrentSelection: fn(PPVT, *mut PPVT) -> HRESULT,
	pub SetFileName: fn(PPVT, PCSTR) -> HRESULT,
	pub GetFileName: fn(PPVT, *mut PSTR) -> HRESULT,
	pub SetTitle: fn(PPVT, PCSTR) -> HRESULT,
	pub SetOkButtonLabel: fn(PPVT, PCSTR) -> HRESULT,
	pub SetFileNameLabel: fn(PPVT, PCSTR) -> HRESULT,
	pub GetResult: fn(PPVT, *mut PPVT) -> HRESULT,
	pub AddPlace: fn(PPVT, PPVT, u32) -> HRESULT,
	pub SetDefaultExtension: fn(PPVT, PCSTR) -> HRESULT,
	pub Close: fn(PPVT, HRESULT) -> HRESULT,
	pub SetClientGuid: fn(PPVT, PCVOID) -> HRESULT,
	pub ClearClientData: fn(PPVT) -> HRESULT,
	pub SetFilter: fn(PPVT, PVOID) -> HRESULT,
}

/// [`IFileDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
/// COM interface over [`IFileDialogVT`](crate::shell::vt::IFileDialogVT).
/// Inherits from [`IModalWindow`](crate::shell::IModalWindow),
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IFileDialog {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IFileDialog {
	const IID: IID = IID::new(0x42f85136, 0xdb7e, 0x439c, 0x85f1, 0xe4075d135fc8);
}

macro_rules! impl_IFileDialog {
	($name:ty, $vt:ty) => {
		use crate::com::funcs::CoTaskMemFree;
		use crate::com::shell::{COMDLG_FILTERSPEC, IShellItem};
		use crate::com::shell::co as shellco;
		use crate::structs::GUID;
		use crate::various::WString;

		impl $name {
			fn ifiledialog_vt(&self) -> &IFileDialogVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IFileDialog::AddPlace`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-addplace)
			/// method.
			pub fn AddPlace(&self,
				si: &IShellItem, fdap: shellco::FDAP) -> WinResult<()>
			{
				hr_to_winresult(
					(self.ifiledialog_vt().AddPlace)(self.ppvt, si.ppvt, fdap.0)
				)
			}

			/// [`IFileDialog::ClearClientData`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
			/// method.
			pub fn ClearClientData(&self) -> WinResult<()> {
				hr_to_winresult((self.ifiledialog_vt().ClearClientData)(self.ppvt))
			}

			/// [`IFileDialog::Close`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
			/// method.
			pub fn Close(&self, hr: co::ERROR) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().Close)(self.ppvt, hr.0 as _),
				)
			}

			/// [`IFileDialog::GetCurrentSelection`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
			/// method.
			pub fn GetCurrentSelection(&self) -> WinResult<IShellItem> {
				let mut ppvQueried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifiledialog_vt().GetCurrentSelection)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IFileDialog::GetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
			/// method.
			pub fn GetFileName(&self) -> WinResult<String> {
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifiledialog_vt().GetFileName)(self.ppvt, &mut pstr),
				).map(|_| {
					let name = WString::from_wchars_nullt(pstr);
					CoTaskMemFree(pstr);
					name.to_string()
				})
			}

			/// [`IFileDialog::GetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
			/// method.
			pub fn GetFileTypeIndex(&self) -> WinResult<u32> {
				let mut index: u32 = 0;
				hr_to_winresult(
					(self.ifiledialog_vt().GetFileTypeIndex)(self.ppvt, &mut index),
				).map(|_| index)
			}

			/// [`IFileDialog::GetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
			/// method.
			pub fn GetFolder(&self) -> WinResult<IShellItem> {
				let mut ppvQueried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifiledialog_vt().GetFolder)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IFileDialog::GetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
			/// method.
			pub fn GetOptions(&self) -> WinResult<shellco::FOS> {
				let mut opts: u32 = 0;
				hr_to_winresult(
					(self.ifiledialog_vt().GetOptions)(self.ppvt, &mut opts),
				).map(|_| shellco::FOS(opts))
			}

			/// [`IFileDialog::GetResult`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
			/// method.
			pub fn GetResult(&self) -> WinResult<IShellItem> {
				let mut ppvQueried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifiledialog_vt().GetResult)(
						self.ppvt,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IFileDialog::SetClientGuid`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
			/// method.
			pub fn SetClientGuid(&self, guid: &GUID) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetClientGuid)(
						self.ppvt,
						guid as *const _ as _,
					),
				)
			}

			/// [`IFileDialog::SetDefaultExtension`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
			/// method.
			pub fn SetDefaultExtension(&self,
				defaultExtension: &str) -> WinResult<()>
			{
				hr_to_winresult(
					(self.ifiledialog_vt().SetDefaultExtension)(
						self.ppvt,
						unsafe { WString::from_str(defaultExtension).as_ptr() },
					),
				)
			}

			/// [`IFileDialog::SetDefaultFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
			/// method.
			pub fn SetDefaultFolder(&self, si: &IShellItem) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetDefaultFolder)(self.ppvt, si.ppvt),
				)
			}

			/// [`IFileDialog::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
			/// method.
			pub fn SetFileName(&self, name: &str) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetFileName)(
						self.ppvt,
						unsafe { WString::from_str(name).as_ptr() },
					),
				)
			}

			/// [`IFileDialog::SetFileNameLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
			/// method.
			pub fn SetFileNameLabel(&self, label: &str) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetFileNameLabel)(
						self.ppvt,
						unsafe { WString::from_str(label).as_ptr() },
					),
				)
			}

			/// [`IFileDialog::SetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
			/// method.
			///
			/// **Note:** The index is one-based.
			pub fn SetFileTypeIndex(&self, iFileType: u32) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetFileTypeIndex)(self.ppvt, iFileType),
				)
			}

			/// [`IFileDialog::SetFileTypes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypes)
			/// method.
			///
			/// # Examples
			///
			/// ```rust,ignore
			/// use winsafe::shell::IFileDialog;
			///
			/// let file_dlg: IFileDialog; // initialized somewhere
			///
			/// file_dlg.SetFileTypes(&[
			///     ("Documents", "*.docx;*.txt"),
			///     ("Images", "*.jpg;*.png;*.bmp"),
			///     ("All files", "*.*"),
			/// ]).unwrap();
			/// ```
			pub fn SetFileTypes<S: AsRef<str>>(&self,
				filterSpec: &[(S, S)]) -> WinResult<()>
			{
				let mut namesBuf = Vec::with_capacity(filterSpec.len());
				let mut specsBuf = Vec::with_capacity(filterSpec.len());
				let mut comDlgs = Vec::with_capacity(filterSpec.len());

				for (name, spec) in filterSpec.iter() {
					namesBuf.push(WString::from_str(name.as_ref()));
					specsBuf.push(WString::from_str(spec.as_ref()));
					comDlgs.push(COMDLG_FILTERSPEC::default());
				}

				namesBuf.iter_mut().enumerate()
					.for_each(|(i, el)| comDlgs[i].set_pszName(Some(el)));

				specsBuf.iter_mut().enumerate()
					.for_each(|(i, el)| comDlgs[i].set_pszSpec(Some(el)));

				hr_to_winresult(
					(self.ifiledialog_vt().SetFileTypes)(
						self.ppvt,
						filterSpec.len() as _,
						comDlgs.as_ptr() as _,
					),
				)
			}

			/// [`IFileDialog::SetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfolder)
			/// method.
			pub fn SetFolder(&self, si: &IShellItem) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetFolder)(self.ppvt, si.ppvt),
				)
			}

			/// [`IFileDialog::SetOkButtonLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
			/// method.
			pub fn SetOkButtonLabel(&self, text: &str) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetOkButtonLabel)(
						self.ppvt,
						unsafe { WString::from_str(text).as_ptr() },
					),
				)
			}

			/// [`IFileDialog::SetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
			/// method.
			pub fn SetOptions(&self, opts: shellco::FOS) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetOptions)(self.ppvt, opts.0),
				)
			}

			/// [`IFileDialog::SetTitle`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
			/// method.
			pub fn SetTitle(&self, text: &str) -> WinResult<()> {
				hr_to_winresult(
					(self.ifiledialog_vt().SetTitle)(
						self.ppvt,
						unsafe { WString::from_str(text).as_ptr() },
					),
				)
			}
		}
	};
}

impl_IUnknown!(IFileDialog, IFileDialogVT);
impl_IModalWindow!(IFileDialog, IFileDialogVT);
impl_IFileDialog!(IFileDialog, IFileDialogVT);