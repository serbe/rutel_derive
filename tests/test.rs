#[macro_use]
extern crate rutel_derive;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde_json::to_string;

#[derive(Serialize, GetSet)]
struct SomeStruct {
    one: i32,
    two: Option<i32>,
}

#[test]
fn test_new_struct() {
    let mut x = SomeStruct::new(1);
    assert_eq!(&1, x.get_one());
    assert!(x.get_two().is_none());
    x.two(Some(1));
    assert_eq!(&Some(1), x.get_two());
}
