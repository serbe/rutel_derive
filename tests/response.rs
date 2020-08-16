#![allow(dead_code)]
// #[rutel_derive::response(result = "Vec<i64>")]
// pub struct GetUpdates {
//     pub offset: Option<i64>,
//     pub limit: Option<i64>,
//     pub timeout: Option<i64>,
//     pub allowed_updates: Option<Vec<String>>,
// }
use rutel_derive::Response;
use serde_json::{to_string, from_value, Value};
use serde::Serialize;
// use rutel_derive::generate;

#[derive(Debug)]
pub struct Bot {
    token: String,
    proxy: String,
}

impl Bot {
    pub fn create_request(
        &mut self,
        _method: &'static str,
        _values: String,
    ) -> Result<Value, String> {
        Err(String::from("test error"))
    }
}

#[derive(Response, Serialize)]
#[response = "Vec<i64>"]
pub struct EStruct {
    a_usize: usize,
    pub b_opt_i64: Option<i64>,
    pub c_opt_string: Option<String>,
    pub d_opt_vec_string: Option<Vec<String>>,
}

// #[test]
// fn works() {

// }
