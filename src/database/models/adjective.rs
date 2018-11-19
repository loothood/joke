#[derive(Clone, Queryable, Serialize, Deserialize)]
pub struct Adjective {
    pub id : i32,
    pub adjective_value : String,
}