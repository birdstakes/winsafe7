#![allow(non_upper_case_globals)]

const_type! { VER_COND, u8,
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `Condition` (`u8`).

	EQUAL, 1
	GREATER, 2
	GREATER_EQUAL, 3
	LESS, 4
	LESS_EQUAL, 5
	AND, 6
	OR, 7
	CONDITION_MASK, 7
}

const_type! { VER_MASK, u32,
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `TypeMask` (`u32`).

	MINORVERSION, 0x0000001
	MAJORVERSION, 0x0000002
	BUILDNUMBER, 0x0000004
	PLATFORMID, 0x0000008
	SERVICEPACKMINOR, 0x0000010
	SERVICEPACKMAJOR, 0x0000020
	SUITENAME, 0x0000040
	PRODUCT_TYPE, 0x0000080
}

const_type! { VER_NT, u8,
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wProductType` (`u8`).

	WORKSTATION, 0x0000001
	DOMAIN_CONTROLLER, 0x0000002
	SERVER, 0x0000003
}

const_type! { VER_PLATFORM, u32,
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `dwPlatformId` (`u32`).

	WIN32s, 0
	WIN32_WINDOWS, 1
	WIN32_NT, 2
}

const_type! { VER_SUITE, u16,
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wSuiteMask` (`u16`).

	SMALLBUSINESS, 0x0001
	ENTERPRISE, 0x0002
	BACKOFFICE, 0x0004
	COMMUNICATIONS, 0x0008
	TERMINAL, 0x0010
	SMALLBUSINESS_RESTRICTED, 0x0020
	EMBEDDEDNT, 0x0040
	DATACENTER, 0x0080
	SINGLEUSERTS, 0x0100
	PERSONAL, 0x0200
	BLADE, 0x0400
	EMBEDDED_RESTRICTED, 0x0800
	SECURITY_APPLIANCE, 0x1000
	STORAGE_SERVER, 0x2000
	COMPUTE_SERVER, 0x4000
	WH_SERVER, 0x8000
	//MULTIUSERTS, 0x00020000 // Win32 bug, truncated to zero as u16
}

const_type! { VK, u16,
	/// [Virtual key codes](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
	/// (`u16`).

	LBUTTON, 0x01
	RBUTTON, 0x02
	CANCEL, 0x03
	MBUTTON, 0x04
	XBUTTON1, 0x05
	XBUTTON2, 0x06
	BACK, 0x08
	TAB, 0x09
	CLEAR, 0x0c
	RETURN, 0x0d
	SHIFT, 0x10
	CONTROL, 0x11
	MENU, 0x12
	PAUSE, 0x13
	CAPITAL, 0x14
	KANA, 0x15
	HANGEUL, 0x15
	HANGUL, 0x15
	JUNJA, 0x17
	FINAL, 0x18
	HANJA, 0x19
	KANJI, 0x19
	ESCAPE, 0x1b
	CONVERT, 0x1c
	NONCONVERT, 0x1d
	ACCEPT, 0x1e
	MODECHANGE, 0x1f
	SPACE, 0x20
	PRIOR, 0x21
	NEXT, 0x22
	END, 0x23
	HOME, 0x24
	LEFT, 0x25
	UP, 0x26
	RIGHT, 0x27
	DOWN, 0x28
	SELECT, 0x29
	PRINT, 0x2a
	EXECUTE, 0x2b
	SNAPSHOT, 0x2c
	INSERT, 0x2d
	DELETE, 0x2e
	HELP, 0x2f
	LWIN, 0x5b
	RWIN, 0x5c
	APPS, 0x5d
	SLEEP, 0x5f
	NUMPAD0, 0x60
	NUMPAD1, 0x61
	NUMPAD2, 0x62
	NUMPAD3, 0x63
	NUMPAD4, 0x64
	NUMPAD5, 0x65
	NUMPAD6, 0x66
	NUMPAD7, 0x67
	NUMPAD8, 0x68
	NUMPAD9, 0x69
	MULTIPLY, 0x6a
	ADD, 0x6b
	SEPARATOR, 0x6c
	SUBTRACT, 0x6d
	DECIMAL, 0x6e
	DIVIDE, 0x6f
	F1, 0x70
	F2, 0x71
	F3, 0x72
	F4, 0x73
	F5, 0x74
	F6, 0x75
	F7, 0x76
	F8, 0x77
	F9, 0x78
	F10, 0x79
	F11, 0x7a
	F12, 0x7b
	F13, 0x7c
	F14, 0x7d
	F15, 0x7e
	F16, 0x7f
	F17, 0x80
	F18, 0x81
	F19, 0x82
	F20, 0x83
	F21, 0x84
	F22, 0x85
	F23, 0x86
	F24, 0x87
	NUMLOCK, 0x90
	SCROLL, 0x91
	OEM_NEC_EQUAL, 0x92
	OEM_FJ_JISHO, 0x92
	OEM_FJ_MASSHOU, 0x93
	OEM_FJ_TOUROKU, 0x94
	OEM_FJ_LOYA, 0x95
	OEM_FJ_ROYA, 0x96
	LSHIFT, 0xa0
	RSHIFT, 0xa1
	LCONTROL, 0xa2
	RCONTROL, 0xa3
	LMENU, 0xa4
	RMENU, 0xa5
	BROWSER_BACK, 0xa6
	BROWSER_FORWARD, 0xa7
	BROWSER_REFRESH, 0xa8
	BROWSER_STOP, 0xa9
	BROWSER_SEARCH, 0xaa
	BROWSER_FAVORITES, 0xab
	BROWSER_HOME, 0xac
	VOLUME_MUTE, 0xad
	VOLUME_DOWN, 0xae
	VOLUME_UP, 0xaf
	MEDIA_NEXT_TRACK, 0xb0
	MEDIA_PREV_TRACK, 0xb1
	MEDIA_STOP, 0xb2
	MEDIA_PLAY_PAUSE, 0xb3
	LAUNCH_MAIL, 0xb4
	LAUNCH_MEDIA_SELECT, 0xb5
	LAUNCH_APP1, 0xb6
	LAUNCH_APP2, 0xb7
	OEM_1, 0xba
	OEM_PLUS, 0xbb
	OEM_COMMA, 0xbc
	OEM_MINUS, 0xbd
	OEM_PERIOD, 0xbe
	OEM_2, 0xbf
	OEM_3, 0xc0
	OEM_4, 0xdb
	OEM_5, 0xdc
	OEM_6, 0xdd
	OEM_7, 0xde
	OEM_8, 0xdf
	OEM_AX, 0xe1
	OEM_102, 0xe2
	ICO_HELP, 0xe3
	ICO_00, 0xe4
	PROCESSKEY, 0xe5
	ICO_CLEAR, 0xe6
	PACKET, 0xe7
	OEM_RESET, 0xe9
	OEM_JUMP, 0xea
	OEM_PA1, 0xeb
	OEM_PA2, 0xec
	OEM_PA3, 0xed
	OEM_WSCTRL, 0xee
	OEM_CUSEL, 0xef
	OEM_ATTN, 0xf0
	OEM_FINISH, 0xf1
	OEM_COPY, 0xf2
	OEM_AUTO, 0xf3
	OEM_ENLW, 0xf4
	OEM_BACKTAB, 0xf5
	ATTN, 0xf6
	CRSEL, 0xf7
	EXSEL, 0xf8
	EREOF, 0xf9
	PLAY, 0xfa
	ZOOM, 0xfb
	NONAME, 0xfc
	PA1, 0xfd
	OEM_CLEAR, 0xfe
}

const_type! { VS_PART, i32,
	/// System visual style
	/// [part](https://docs.microsoft.com/en-us/windows/win32/controls/parts-and-states)
	/// (`i32`).

	// button parts
	BP_PUSHBUTTON, 1
	BP_RADIOBUTTON, 2
	BP_CHECKBOX, 3
	BP_GROUPBOX, 4
	BP_USERBUTTON, 5
	BP_COMMANDLINK, 6
	BP_COMMANDLINKGLYPH, 7
	BP_RADIOBUTTON_HCDISABLED, 8
	BP_CHECKBOX_HCDISABLED, 9
	BP_GROUPBOX_HCDISABLED, 10
	BP_PUSHBUTTONDROPDOWN, 11

	// clock parts
	CLP_TIME, 1

	// combo box parts
	CP_DROPDOWNBUTTON, 1
	CP_BACKGROUND, 2
	CP_TRANSPARENTBACKGROUND, 3
	CP_BORDER, 4
	CP_READONLY, 5
	CP_DROPDOWNBUTTONRIGHT, 6
	CP_DROPDOWNBUTTONLEFT, 7
	CP_CUEBANNER, 8
	CP_DROPDOWNITEM, 9

	// communications parts
	CSST_TAB, 1

	// control panel parts
	CPANEL_NAVIGATIONPANE, 1
	CPANEL_CONTENTPANE, 2
	CPANEL_NAVIGATIONPANELABEL, 3
	CPANEL_CONTENTPANELABEL, 4
	CPANEL_TITLE, 5
	CPANEL_BODYTEXT, 6
	CPANEL_HELPLINK, 7
	CPANEL_TASKLINK, 8
	CPANEL_GROUPTEXT, 9
	CPANEL_CONTENTLINK, 10
	CPANEL_SECTIONTITLELINK, 11
	CPANEL_LARGECOMMANDAREA, 12
	CPANEL_SMALLCOMMANDAREA, 13
	CPANEL_BUTTON, 14
	CPANEL_MESSAGETEXT, 15
	CPANEL_NAVIGATIONPANELINE, 16
	CPANEL_CONTENTPANELINE, 17
	CPANEL_BANNERAREA, 18
	CPANEL_BODYTITLE, 19

	// date and time picker parts
	DP_DATETEXT, 1
	DP_DATEBORDER, 2
	DP_SHOWCALENDARBUTTONRIGHT, 3

	// drag and drop parts
	DD_COPY, 1
	DD_MOVE, 2
	DD_UPDATEMETADATA, 3
	DD_CREATELINK, 4
	DD_WARNING, 5
	DD_NONE, 6
	DD_IMAGEBG, 7
	DD_TEXTBG, 8

	// edit parts
	EP_EDITTEXT, 1
	EP_CARET, 2
	EP_BACKGROUND, 3
	EP_PASSWORD, 4
	EP_BACKGROUNDWITHBORDER, 5
	EP_EDITBORDER_NOSCROLL, 6
	EP_EDITBORDER_HSCROLL, 7
	EP_EDITBORDER_VSCROLL, 8
	EP_EDITBORDER_HVSCROLL, 9

	// explorer bar parts
	EBP_HEADERBACKGROUND, 1
	EBP_HEADERCLOSE, 2
	EBP_HEADERPIN, 3
	EBP_IEBARMENU, 4
	EBP_NORMALGROUPBACKGROUND, 5
	EBP_NORMALGROUPCOLLAPSE, 6
	EBP_NORMALGROUPEXPAND, 7
	EBP_NORMALGROUPHEAD, 8
	EBP_SPECIALGROUPBACKGROUND, 9
	EBP_SPECIALGROUPCOLLAPSE, 10
	EBP_SPECIALGROUPEXPAND, 11
	EBP_SPECIALGROUPHEAD, 12

	// flyout parts
	FLYOUT_HEADER, 1
	FLYOUT_BODY, 2
	FLYOUT_LABEL, 3
	FLYOUT_LINK, 4
	FLYOUT_DIVIDER, 5
	FLYOUT_WINDOW, 6
	FLYOUT_LINKAREA, 7
	FLYOUT_LINKHEADER, 8

	// globals parts
	GP_BORDER, 1
	GP_LINEHORZ, 2
	GP_LINEVERT, 3

	// header parts
	HP_HEADERITEM, 1
	HP_HEADERITEMLEFT, 2
	HP_HEADERITEMRIGHT, 3
	HP_HEADERSORTARROW, 4
	HP_HEADERDROPDOWN, 5
	HP_HEADERDROPDOWNFILTER, 6
	HP_HEADEROVERFLOW, 7

	// list box parts
	LBCP_BORDER_HSCROLL, 1
	LBCP_BORDER_HVSCROLL, 2
	LBCP_BORDER_NOSCROLL, 3
	LBCP_BORDER_VSCROLL, 4
	LBCP_ITEM, 5

	// list view parts
	LVP_LISTITEM, 1
	LVP_LISTGROUP, 2
	LVP_LISTDETAIL, 3
	LVP_LISTSORTEDDETAIL, 4
	LVP_EMPTYTEXT, 5
	LVP_GROUPHEADER, 6
	LVP_GROUPHEADERLINE, 7
	LVP_EXPANDBUTTON, 8
	LVP_COLLAPSEBUTTON, 9
	LVP_COLUMNDETAIL, 10
}

const_type! { VS_STATE, i32,
	/// System visual style
	/// [state](https://docs.microsoft.com/en-us/windows/win32/controls/parts-and-states)
	/// (`i32`).

	NONE, 0

	// list view states
	LVCB_NORMAL, 1
	LVCB_HOVER, 2
	LVCB_PUSHED, 3

	LVEB_NORMAL, 1
	LVEB_HOVER, 2
	LVEB_PUSHED, 3

	LVGH_OPEN, 1
	LVGH_OPENHOT, 2
	LVGH_OPENSELECTED, 3
	LVGH_OPENSELECTEDHOT, 4
	LVGH_OPENSELECTEDNOTFOCUSED, 5
	LVGH_OPENSELECTEDNOTFOCUSEDHOT, 6
	LVGH_OPENMIXEDSELECTION, 7
	LVGH_OPENMIXEDSELECTIONHOT, 8
	LVGH_CLOSE, 9
	LVGH_CLOSEHOT, 10
	LVGH_CLOSESELECTED, 11
	LVGH_CLOSESELECTEDHOT, 12
	LVGH_CLOSESELECTEDNOTFOCUSED, 13
	LVGH_CLOSESELECTEDNOTFOCUSEDHOT, 14
	LVGH_CLOSEMIXEDSELECTION, 15
	LVGH_CLOSEMIXEDSELECTIONHOT, 16

	LVGHL_OPEN, 1
	LVGHL_OPENHOT, 2
	LVGHL_OPENSELECTED, 3
	LVGHL_OPENSELECTEDHOT, 4
	LVGHL_OPENSELECTEDNOTFOCUSED, 5
	LVGHL_OPENSELECTEDNOTFOCUSEDHOT, 6
	LVGHL_OPENMIXEDSELECTION, 7
	LVGHL_OPENMIXEDSELECTIONHOT, 8
	LVGHL_CLOSE, 9
	LVGHL_CLOSEHOT, 10
	LVGHL_CLOSESELECTED, 11
	LVGHL_CLOSESELECTEDHOT, 12
	LVGHL_CLOSESELECTEDNOTFOCUSED, 13
	LVGHL_CLOSESELECTEDNOTFOCUSEDHOT, 14
	LVGHL_CLOSEMIXEDSELECTION, 15
	LVGHL_CLOSEMIXEDSELECTIONHOT, 16

	LISS_NORMAL, 1
	LISS_HOT, 2
	LISS_SELECTED, 3
	LISS_DISABLED, 4
	LISS_SELECTEDNOTFOCUS, 5
	LISS_HOTSELECTED, 6
}
