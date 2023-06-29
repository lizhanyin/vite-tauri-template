#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{thread, time};
use tauri::window::Window;
use serde_json::{Value};

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

struct State{
  listening: bool
}

#[tauri::command]
fn event(window: Window, data: String) -> Value{
  let in_json_data: Value = serde_json::from_str(&data).unwrap();
  let cmd: &Value = in_json_data.get("command").unwrap();
  let val: &Value = in_json_data.get("value").unwrap();

  let mut rst: Value = Value::Null;

  if let Value::String(cmd_str) = cmd {
    match &cmd_str as &str {
      "set_window_top" => rst = set_window_top(window, val.clone()),
      "init_process" => rst = init_process(window, val.clone()),
      _ => (),
    }
  }

  rst
}

fn set_window_top(window: Window, data: Value) -> Value{
  let _ = window.set_always_on_top(data == "yes");
  Value::Bool(true)
}

static mut LISTENING_STATE: State = State{listening: false};
fn init_process(window: Window, _data: Value) -> Value {
  if !unsafe { LISTENING_STATE.listening } {
    unsafe { LISTENING_STATE.listening = true };

    std::thread::spawn(move || {
      loop { // 循环触发
        window.emit("my-event", Payload { message: "Tauri is awesome!".into() }).unwrap(); // 前端自动解析为对象
        thread::sleep(time::Duration::from_millis(1000)); // 每隔一秒执行
      }
    });
    return Value::Bool(false);
  } 
  return Value::Bool(true);
}

fn main() {
  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      event
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");


}

