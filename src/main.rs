#![allow(non_snake_case)] // disables non_snake_case warning
#![windows_subsystem = "windows"]  // hides terminal

use winapi::um::winuser::SendMessageA;
use winapi::shared::windef::HWND__;

fn main() {
    unsafe {
        SendMessageA(0xFFFF as *mut HWND__, 0x0112, 0xF170, 2)  // windows API call to stop sending video to monitor
    };
}
