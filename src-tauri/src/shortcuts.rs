use tauri::{Manager,AppHandle, GlobalShortcutManager};
extern crate winapi;
use winapi::um::winuser::{
    keybd_event, KEYEVENTF_KEYUP,VK_MENU,VK_CONTROL
};
use std::thread;
use std::time::Duration;
// 定义虚拟键码
const VK_C: u8 = 0x43;

#[tauri::command]
pub fn update_shortcut(shortcut: String, handle: AppHandle) {
    handle
        .global_shortcut_manager()
        .unregister_all()
        .unwrap();

    let window = handle.get_window("main").unwrap();
    match handle
        .global_shortcut_manager()
        .register(&shortcut, move || {
            println!("Shortcut triggered successfully");
            unsafe {
                // 取消按下的快捷键
                keybd_event(VK_MENU.try_into().unwrap(), 0, KEYEVENTF_KEYUP, 0);
                // 模拟 Ctrl+C 键组合
                keybd_event(VK_CONTROL.try_into().unwrap(), 0, 0, 0);
                keybd_event(VK_C, 0, 0, 0);
                thread::sleep(Duration::from_millis(50)); // 等待一小会儿确保按键被处理
                keybd_event(VK_C, 0, KEYEVENTF_KEYUP, 0);
                keybd_event(VK_CONTROL.try_into().unwrap(), 0, KEYEVENTF_KEYUP, 0);
              }
            window.unminimize().unwrap();
            window.set_focus().unwrap();
            window.emit("activate_input_field", {}).unwrap();
        }) {
        Ok(_) => println!("Shortcut registered successfully"),
        Err(err) => eprintln!("Failed to register shortcut: {}", err),
    }
}