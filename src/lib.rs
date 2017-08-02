#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!("./bindings.rs"));

extern crate winapi;
extern crate user32;
extern crate kernel32;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ffi::OsStr;
    use std::ffi::CString;
    use std::io::Error;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;

    use std::io::Write;
    use winapi::HWND;
    use winapi::UINT;
    use winapi::LPCWSTR;
    use std::os::raw::c_int;

    macro_rules! eprintln(
        ($($arg:tt)*) => { {
            let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
            r.expect("failed printing to stderr");
        } }
    );

    fn utf16(input: &str) -> Vec<u16> {
        OsStr::new(input).encode_wide().chain(once(0)).collect()
    }

    fn ascii(input: &str) -> CString {
        CString::new(input).unwrap()
    }

    static mut MessageBoxW_real: LPVOID = 0 as LPVOID;

    unsafe extern "C" fn MessageBoxW_hook(
        hWnd: HWND,
        lpText: LPCWSTR,
        _: LPCWSTR, // lpCaption
        uType: UINT,
    ) -> c_int {
        let MessageBoxW: unsafe extern "C" fn(
            hWnd: HWND,
            lpText: LPCWSTR,
            lpCaption: LPCWSTR,
            uType: UINT,
        ) -> c_int = mem::transmute(MessageBoxW_real);

        MessageBoxW(
            hWnd,
            utf16("Haha I hooked your caption!").as_ptr(),
            lpText,
            uType,
        )
    }

    #[test]
    fn inject_simple_program() {
        let mut hooker = unsafe { CNktHookLib::new() };

        let lib = unsafe { kernel32::LoadLibraryW(utf16("user32.dll").as_ptr()) };
        if lib.is_null() {
            panic!("Could not open kernel32.dll");
        }

        let MessageBoxW_addr =
            unsafe { kernel32::GetProcAddress(lib, ascii("MessageBoxW").as_ptr()) };
        if MessageBoxW_addr.is_null() {
            panic!("Could not find MessageBoxW");
        }

        let mut hookId: SIZE_T = 0;
        {
            let result = unsafe {
                hooker.Hook(
                    &mut hookId,
                    &mut MessageBoxW_real as *mut LPVOID,
                    MessageBoxW_addr as LPVOID,
                    MessageBoxW_hook as LPVOID,
                    0,
                )
            };
            if result != 0 {
                panic!("Hooking failed with code {}", result);
            }
            eprintln!("Hooking successful");
        }

        fn showBox() {
            let msg = "Hello, world!";
            let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
            let ret = unsafe {
                user32::MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), winapi::MB_OK)
            };
            if ret == 0 {
                println!("Failed: {:?}", Error::last_os_error());
            }
        }

        showBox();

        {
            let result = unsafe { hooker.Unhook(hookId) };
            if result != 0 {
                panic!("Unhooking failed with code {}", result);
            }
            eprintln!("Unhooking successful");
        }

        showBox();
    }
}
