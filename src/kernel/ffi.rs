use crate::ffi_types::{BOOL, HANDLE, PCSTR, PCVOID, PFUNC, PSTR, PVOID};

extern_sys! { "kernel32";
	BeginUpdateResourceW(PCSTR, BOOL) -> HANDLE
	CloseHandle(HANDLE) -> BOOL
	CopyFileW(PCSTR, PCSTR, BOOL) -> BOOL
	CreateFileMappingW(HANDLE, PVOID, u32, u32, u32, PCSTR) -> HANDLE
	CreateFileW(PCSTR, u32, u32, PVOID, u32, u32, HANDLE) -> HANDLE
	CreatePipe(*mut HANDLE, *mut HANDLE, PVOID, u32) -> BOOL
	CreateProcessW(PCSTR, PSTR, PVOID, PVOID, BOOL, u32, PVOID, PCSTR, PVOID, PVOID) -> BOOL
	CreateThread(PVOID, u64, PVOID, PVOID, u32, *mut u32) -> HANDLE
	CreateToolhelp32Snapshot(u32, u32) -> HANDLE
	DeleteFileW(PCSTR) -> BOOL
	DuplicateToken(HANDLE, u32, *mut HANDLE) -> BOOL
	EndUpdateResourceW(HANDLE, BOOL) -> BOOL
	EnumResourceLanguagesW(HANDLE, PCSTR, PCSTR, PFUNC, isize) -> BOOL
	EnumResourceNamesW(HANDLE, PCSTR, PFUNC, isize) -> BOOL
	EnumResourceTypesW(HANDLE, PFUNC, isize) -> BOOL
	ExitProcess(u32)
	ExitThread(u32)
	ExpandEnvironmentStringsW(PCSTR, PSTR, u32) -> u32
	FileTimeToSystemTime(PCVOID, PVOID) -> BOOL
	FindClose(HANDLE) -> BOOL
	FindFirstFileW(PCSTR, PVOID) -> HANDLE
	FindNextFileW(HANDLE, PVOID) -> BOOL
	FindResourceExW(HANDLE, PCSTR, PCSTR, u16) -> HANDLE
	FindResourceW(HANDLE, PCSTR, PCSTR) -> HANDLE
	FlushInstructionCache(HANDLE, PCVOID, u64) -> BOOL
	FlushProcessWriteBuffers()
	FormatMessageW(u32, PCVOID, u32, u32, PSTR, u32, PVOID) -> u32
	FreeEnvironmentStringsW(HANDLE) -> BOOL
	FreeLibrary(HANDLE) -> BOOL
	GetBinaryTypeW(PCSTR, *mut u32) -> BOOL
	GetCommandLineW() -> PCSTR
	GetComputerNameW(PSTR, *mut u32) -> BOOL
	GetCurrentDirectoryW(u32, PSTR) -> u32
	GetCurrentProcess() -> HANDLE
	GetCurrentProcessId() -> u32
	GetCurrentProcessToken() -> HANDLE
	GetCurrentThread() -> HANDLE
	GetCurrentThreadEffectiveToken() -> HANDLE
	GetCurrentThreadId() -> u32
	GetEnvironmentStringsW() -> HANDLE
	GetExitCodeProcess(HANDLE, *mut u32) -> BOOL
	GetExitCodeThread(HANDLE, *mut u32) -> BOOL
	GetFileAttributesW(PCSTR) -> u32
	GetFileInformationByHandle(HANDLE, PVOID) -> BOOL
	GetFileSizeEx(HANDLE, *mut i64) -> BOOL
	GetFileType(HANDLE) -> u32
	GetGuiResources(HANDLE, u32) -> u32
	GetLargePageMinimum() -> u64
	GetLastError() -> u32
	GetLogicalDriveStringsW(u32, PSTR) -> u32
	GetModuleFileNameW(HANDLE, PSTR, u32) -> u32
	GetModuleHandleW(PCSTR) -> HANDLE
	GetNativeSystemInfo(PVOID)
	GetProcAddress(HANDLE, *const u8) -> PCVOID
	GetProcessId(HANDLE) -> u32
	GetProcessIdOfThread(HANDLE) -> u32
	GetProcessTimes(HANDLE, PVOID, PVOID, PVOID, PVOID) -> BOOL
	GetStartupInfoW(PVOID)
	GetSystemDirectoryW(PSTR, u32) -> u32
	GetSystemInfo(PVOID)
	GetSystemTime(PVOID)
	GetSystemTimeAsFileTime(PVOID)
	GetSystemTimes(PVOID, PVOID, PVOID) -> BOOL
	GetTempPathW(u32, PSTR) -> u32
	GetThreadId(HANDLE) -> u32
	GetThreadTimes(HANDLE, PVOID, PVOID, PVOID, PVOID) -> BOOL
	GetTickCount64() -> u64
	GetVolumeInformationW(PCSTR, PSTR, u32, *mut u32, *mut u32, *mut u32, PSTR, u32) -> BOOL
	GlobalAlloc(u32, u64) -> HANDLE
	GlobalFlags(HANDLE) -> u32
	GlobalFree(HANDLE) -> HANDLE
	GlobalLock(HANDLE) -> PVOID
	GlobalMemoryStatusEx(PVOID) -> BOOL
	GlobalReAlloc(HANDLE, u64, u32) -> HANDLE
	GlobalSize(HANDLE) -> u64
	GlobalUnlock(HANDLE) -> BOOL
	IsTokenRestricted(HANDLE) -> BOOL
	IsWow64Process(HANDLE, *mut BOOL) -> BOOL
	LoadLibraryW(PCSTR) -> HANDLE
	LoadResource(HANDLE, HANDLE) -> HANDLE
	LocalFree(HANDLE) -> HANDLE
	LocalSize(HANDLE) -> u64
	LockFile(HANDLE, u32, u32, u32, u32) -> BOOL
	LockResource(HANDLE) -> PVOID
	lstrlenW(PCSTR) -> i32
	MapViewOfFile(HANDLE, u32, u32, u32, usize) -> PVOID
	MoveFileW(PCSTR, PCSTR) -> BOOL
	MulDiv(i32, i32, i32) -> i32
	MultiByteToWideChar(u32, u32, *const u8, i32, PSTR, i32) -> i32
	OpenProcess(u32, BOOL, u32) -> HANDLE
	OpenProcessToken(HANDLE, u32, *mut HANDLE) -> BOOL
	OpenThreadToken(HANDLE, u32, BOOL, *mut HANDLE) -> BOOL
	OutputDebugStringW(PCSTR)
	Process32FirstW(HANDLE, PVOID) -> BOOL
	Process32NextW(HANDLE, PVOID) -> BOOL
	QueryFullProcessImageNameW(HANDLE, u32, PSTR, *mut u32) -> BOOL
	QueryPerformanceCounter(*mut i64) -> BOOL
	QueryPerformanceFrequency(*mut i64) -> BOOL
	ReadFile(HANDLE, PVOID, u32, *mut u32, PVOID) -> BOOL
	ReplaceFileW(PCSTR, PCSTR, PCSTR, u32, PVOID, PVOID) -> BOOL
	SetCurrentDirectoryW(PCSTR) -> BOOL
	SetEndOfFile(HANDLE) -> BOOL
	SetFilePointerEx(HANDLE, i64, *mut i64, u32) -> BOOL
	SetLastError(u32)
	SizeofResource(HANDLE, HANDLE) -> u32
	Sleep(u32)
	SystemTimeToFileTime(PCVOID, PVOID) -> BOOL
	SystemTimeToTzSpecificLocalTime(PCVOID, PCVOID, PVOID) -> BOOL
	UnlockFile(HANDLE, u32, u32, u32, u32) -> BOOL
	UnmapViewOfFile(PCVOID) -> BOOL
	UpdateResourceW(HANDLE, PCSTR, PCSTR, u16, PVOID, u32) -> BOOL
	VerifyVersionInfoW(PVOID, u32, u64) -> BOOL
	VerSetConditionMask(u64, u32, u8) -> u64
	WaitForSingleObject(HANDLE, u32) -> u32
	WideCharToMultiByte(u32, u32, PCSTR, i32, PSTR, i32, *const u8, *mut BOOL) -> i32
	WriteFile(HANDLE, PCVOID, u32, *mut u32, PVOID) -> BOOL
}
