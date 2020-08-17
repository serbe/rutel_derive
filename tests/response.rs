use serde::Serialize;
use serde_json::{from_value, to_string, Value, Result, json};

use rutel_derive::Response;

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
    ) -> Result<Value> {
        Ok(json!(null))
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
