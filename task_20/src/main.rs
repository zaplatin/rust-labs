// TODO: Удалите эту строчку, когда все будет готово.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // См. неопределенные типы (opaque) https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Раскладка согласно ман странице Linux для функции readdir(3), где ino_t и
    // off_t соответствуют определениям в
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Раскладка в соответствии в ман страницей macOS для dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    unsafe extern "C" {
        pub unsafe fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        // См. https://github.com/rust-lang/libc/issues/414 и раздел
        // _DARWIN_FEATURE_64_BIT_INODE в ман страницах macOS для stat(2).
        //
        // "Platforms that existed before these updates were available" это ссылка на
        // macOS (в противоположность iOS / wearOS / и пр.) на Intel и PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        pub unsafe fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Вызовите opendir и верните значение Ok если она сработала,
        // иначе Err с сообщением.
        
        // Преобразуем строковый путь в CString
        let c_path = CString::new(path).map_err(|e| format!("Invalid path: {}", e))?;
        
        // Вызываем opendir (небезопасный код)
        let dir_ptr = unsafe { ffi::opendir(c_path.as_ptr()) };
        
        // Проверяем, успешно ли открылась директория
        if dir_ptr.is_null() {
            Err(format!("Failed to open directory: {}", path))
        } else {
            Ok(DirectoryIterator {
                path: c_path,
                dir: dir_ptr,
            })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Продолжайте вызывать readdir пока не получите нулевой указатель.
        
        // Вызываем readdir (небезопасный код)
        let entry_ptr = unsafe { ffi::readdir(self.dir) };
        
        // Проверяем, не достигли ли конца директории
        if entry_ptr.is_null() {
            return None;
        }
        
        // Безопасно разыменовываем указатель (we know it's valid here)
        let entry = unsafe { &*entry_ptr };
        
        // Преобразуем имя файла из C строки в OsString
        // d_name - это массив c_char, который представляет нуль-терминированную строку
        let name_bytes = unsafe {
            // Находим длину строки
            let mut len = 0;
            while len < entry.d_name.len() && entry.d_name[len] != 0 {
                len += 1;
            }
            // Создаем слайс байтов
            std::slice::from_raw_parts(entry.d_name.as_ptr() as *const u8, len)
        };
        
        // Преобразуем байты в OsStr, а затем в OsString
        Some(OsStr::from_bytes(name_bytes).to_os_string())
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Вызовите как надо closedir.
        if !self.dir.is_null() {
            unsafe {
                ffi::closedir(self.dir);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("файлы: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}