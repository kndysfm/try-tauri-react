#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate serde;
extern crate hidapi;
use std::ffi::{CStr, CString};
use hidapi::{HidApi, DeviceInfo};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyOption {
    id: String,
    label: String,
}

fn info_to_option(info: &&DeviceInfo) -> MyOption {
    let text = format!("PID:{:04X}_VID:{:04X}&UP:{:04X}_U:{:04X}", 
        info.vendor_id(), info.product_id(),
        info.usage_page(), info.usage()); 
    let path = info.path().to_str().unwrap().to_string();
    MyOption {label: text, id: path}
}

fn new_hidapi() -> HidApi { HidApi::new().expect("Failed to create `HidApi`") }

#[tauri::command]
fn enum_hid() -> Vec<MyOption> {
    println!("enum_hid()");
    let hidapi = new_hidapi();
    let mut devs: Vec<_> = hidapi.device_list().collect();
    devs.sort_by_key(|d| d.product_id());
    devs.sort_by_key(|d| d.vendor_id());
    devs.iter().map(info_to_option).collect()
}

#[tauri::command]
fn sel_hid(path: &str) -> String {
    println!("sel_hid(\"{}\")", path);
    let c_string = CString::new(path).unwrap();
    let cpath = c_string.as_c_str();
    let hidapi = new_hidapi();
    if let Ok(dev) = hidapi.open_path(cpath) {
        if let Ok(Some(prod_str)) = dev.get_product_string() {
            println!("Succeeded to oepn HID {}", &prod_str);
            prod_str
        } else {
            "".to_string()
        }
    } else {
        println!("Failed to open HID : {}", path);
        "Error".to_string()
    }
}

fn main() {
    println!("main()");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![enum_hid,sel_hid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
