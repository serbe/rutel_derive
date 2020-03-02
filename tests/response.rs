// #[rutel_derive::response(result = "Vec<i64>")]
// pub struct GetUpdates {
//     pub offset: Option<i64>,
//     pub limit: Option<i64>,
//     pub timeout: Option<i64>,
//     pub allowed_updates: Option<Vec<String>>,
// }

#[test]
fn works() {
    #[rutel_derive::response(result, "Vec<i64>")]
    pub struct GetUpdates {
        pub offset: Option<i64>,
        pub limit: Option<i64>,
        pub timeout: Option<i64>,
        pub allowed_updates: Option<Vec<String>>,
    }
}