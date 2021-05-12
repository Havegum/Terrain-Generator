use web_sys::console;
use std::collections::HashMap;

use super::board::{Board, Action};
use super::civ::Civilization;


#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
      console::log_1(
        &JsValue::from_str(
          format!( $( $t )* ).as_str()
        )
      );
    }
}

