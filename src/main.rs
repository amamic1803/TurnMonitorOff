#![allow(non_snake_case)] // disables non_snake_case warning
#![windows_subsystem = "windows"]  // hides terminal

use windows::Win32::Foundation::{
    LPARAM,
    WPARAM,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SendMessageA,
    HWND_BROADCAST,
    SC_MONITORPOWER,
    WM_SYSCOMMAND,
};

fn main() {
    unsafe {
        // windows API call to put the monitor to sleep
        SendMessageA(HWND_BROADCAST, WM_SYSCOMMAND, WPARAM(SC_MONITORPOWER as usize), LPARAM(2))
    };
}
