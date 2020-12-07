const_type! {
	/// [`MessageBox`](crate::HWND::MessageBox) `uType`.
	MB, u32,

	ABORTRETRYIGNORE, 0x00000002
	CANCELTRYCONTINUE, 0x00000006
	HELP, 0x00004000
	OK, 0x00000000
	OKCANCEL, 0x00000001
	RETRYCANCEL, 0x00000005
	YESNO, 0x00000004
	YESNOCANCEL, 0x00000003

	ICONEXCLAMATION, 0x00000030
	ICONWARNING, Self::ICONEXCLAMATION.0
	ICONINFORMATION, 0x00000040
	ICONASTERISK, Self::ICONINFORMATION.0
	ICONQUESTION, 0x00000020
	ICONSTOP, Self::ICONERROR.0
	ICONERROR, 0x00000010
	ICONHAND, Self::ICONERROR.0

	DEFBUTTON1, 0x00000000
	DEFBUTTON2, 0x00000100
	DEFBUTTON3, 0x00000200
	DEFBUTTON4, 0x00000300

	APPLMODAL, 0x00000000
	SYSTEMMODAL, 0x00001000
	TASKMODAL, 0x00002000

	DEFAULT_DESKTOP_ONLY, 0x00020000
	RIGHT, 0x00080000
	RTLREADING, 0x00100000
	SETFOREGROUND, 0x00010000
	TOPMOST, 0x00040000
	SERVICE_NOTIFICATION, 0x00200000
}

const_type! {
	/// Common control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/common-control-reference#notifications).
	NM, i32,

	OUTOFMEMORY, Self::FIRST.0 - 1
	CLICK, Self::FIRST.0 - 2
	DBLCLK, Self::FIRST.0 - 3
	RETURN, Self::FIRST.0 - 4
	RCLICK, Self::FIRST.0 - 5
	RDBLCLK, Self::FIRST.0 - 6
	SETFOCUS, Self::FIRST.0 - 7
	KILLFOCUS, Self::FIRST.0 - 8
	CUSTOMDRAW, Self::FIRST.0 - 12
	HOVER, Self::FIRST.0 - 13
	NCHITTEST, Self::FIRST.0 - 14
	KEYDOWN, Self::FIRST.0 - 15
	RELEASEDCAPTURE, Self::FIRST.0 - 16
	SETCURSOR, Self::FIRST.0 - 17
	CHAR, Self::FIRST.0 - 18
	TOOLTIPSCREATED, Self::FIRST.0 - 19
	LDOWN, Self::FIRST.0 - 20
	RDOWN, Self::FIRST.0 - 21
	THEMECHANGED, Self::FIRST.0 - 22
}
impl NM {
	const FIRST: Self = Self(0);
}