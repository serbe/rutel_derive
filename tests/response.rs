use std::result;

use rutel_derive::Response;
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error as TErr;

pub type Result<T> = result::Result<T, Error>;

#[derive(TErr, Debug)]
pub enum Error {
    #[error("json error")]
    JSON(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct Bot {}

impl Bot {
    pub async fn create_request(
        &mut self,
        _method: &'static str,
        _values: String,
    ) -> Result<Value> {
        let value = json!(null);
        Ok(value)
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

#[test]
fn testing_generate() {
    let mut est = EStruct::new(2);
    assert_eq!(est.a_usize, 2);
    assert_eq!(est.b_opt_i64, None);
    assert_eq!(est.c_opt_string, None);
    assert_eq!(est.d_opt_vec_string, None);
    assert_eq!(est.get_a_usize(), &2);
    assert_eq!(est.get_b_opt_i64(), &None);
    assert_eq!(est.get_c_opt_string(), &None);
    assert_eq!(est.get_d_opt_vec_string(), &None);
    est.a_usize(3);
    est.b_opt_i64(Some(4));
    est.c_opt_string(Some(String::from("5")));
    est.d_opt_vec_string(Some(vec![String::from("6")]));
    assert_eq!(est.get_a_usize(), &3);
    assert_eq!(est.get_b_opt_i64(), &Some(4));
    assert_eq!(est.get_c_opt_string(), &Some(String::from("5")));
    assert_eq!(est.get_d_opt_vec_string(), &Some(vec![String::from("6")]));
    est.a_usize = 4;
    est.b_opt_i64 = Some(5);
    est.c_opt_string = Some(String::from("6"));
    est.d_opt_vec_string = Some(vec![String::from("7")]);
    assert_eq!(est.get_a_usize(), &4);
    assert_eq!(est.get_b_opt_i64(), &Some(5));
    assert_eq!(est.get_c_opt_string(), &Some(String::from("6")));
    assert_eq!(est.get_d_opt_vec_string(), &Some(vec![String::from("7")]));
}
