use database::schema::user_adjective;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserAdjective {
    pub id : i32,
    pub user_id : i32,
    pub adjective_id : i32,
    pub count : Option<i32>
}

#[derive(Insertable)]
#[table_name="user_adjective"]
pub struct NewUsAdRecord<'a> {
    pub user_id : &'a i32,
    pub adjective_id : &'a i32,
    pub count : &'a i32,
}
