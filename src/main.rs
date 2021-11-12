#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::Arc;
use std::sync::Mutex;
use rocket::State;

struct ControllerData {
    temperature: i32
}

type ControllerDataState = Arc<Mutex<ControllerData>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world from the bread box controller"
}

#[get("/temperature")]
fn count(temperature_state: State<ControllerDataState>) -> String {
  let state = temperature_state.as_ref().lock().expect("Error");
  format!("The current temperature is {}", state.temperature)
}

fn main() {
  let controller_data = ControllerData{temperature: 0};
  let controller_data_state = Arc::new(Mutex::new(controller_data));

  rocket::ignite()
    .manage(Arc::clone(&controller_data_state))
    .mount("/", routes![index, count])
    .launch();
}
