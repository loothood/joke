use database::{ models::{ user::User, adjective::Adjective }
              , dbimpl };
use database::connection::{init_pool, DbConn};
use middleware::randomize::RandomData;
use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};

#[derive(Serialize)]
pub struct UsAdjCount {
    pub user : User,
    pub adjective : Adjective,
    pub count : i32,
}

#[derive(Serialize)]
pub struct IndexData {
    random_user_name : String,
    random_adjective_value : String,
    usadjcounts : Vec<UsAdjCount>,
}

impl IndexData {
    pub fn new(connection : &DbConn, allDbData : &AllDbData) -> IndexData {
        let most_frequent = dbimpl::get_top10_all(&connection)
            .unwrap_or(Vec::new());
        let mut usadjcounts = Vec::new();
        for most in most_frequent.iter() {
            let user = dbimpl::get_user_by_id(&connection, &most.user_id)
                .expect(&format!("User not found in the database. user_id : {}", most.user_id));
            let adjective = dbimpl::get_adjective_by_id(&connection, &most.adjective_id)
                .expect(&format!("Adjective not found in the database. adjective_id : {}", most.adjective_id));
            let count = most.count
                .expect(&format!("Cannot get count from database for user_id : {}", most.user_id));
            let usadjcount = UsAdjCount { user, adjective, count };
            usadjcounts.push(usadjcount);
        }
        let random_adjective = RandomData::new(&allDbData.adjectives);
        let random_user = RandomData::new(&allDbData.users);
        create_or_update_us_adj(&connection, &random_user.value, &random_adjective.value);
        IndexData {
            random_user_name : random_user.value.user_name,
            random_adjective_value : random_adjective.value.adjective_value,
            usadjcounts,
        }
    }
}


fn create_or_update_us_adj(conn : &DbConn, user : &User, adjective : &Adjective) {
    let count_query = dbimpl::get_count_of_repeats(conn, &user.id, &adjective.id);
    match count_query {
        Ok(mut us_adj) =>
            match us_adj.pop() {
                Some(data) => {
                    let count_of_repeats: i32 = data.count.expect("Cannot get count of repeats!");
                    dbimpl::update_us_adj_record(conn, &user.id, &adjective.id, &count_of_repeats)
                },
                None => dbimpl::create_us_adj_record(conn, &user.id, &adjective.id),
            },
        Err(_) => panic!("Cannot get count of queries for user_id: {} and adjective_id: {}", user.id, adjective.id),
    };
}


#[derive(Serialize)]
pub struct AdjCount {
    pub adjective : Adjective,
    pub count : i32,
}

#[derive(Serialize)]
pub struct DisplayUser {
    pub user : User,
    pub adj_count : Vec<AdjCount>,
}

impl DisplayUser {
    pub fn by_id(connection : &DbConn, id : &i32) -> DisplayUser {
        let user = dbimpl::get_user_by_id(&connection, &id)
            .expect(&format!("User with id: {} no found!", id));
        DisplayUser::by_user(connection, user)
    }

    pub fn by_name(connection : &DbConn, name : &String) -> DisplayUser {
        let user = dbimpl::get_user_by_name(&connection, &name)
            .expect(&format!("User with name: {} no found!", name));
        DisplayUser::by_user(connection, user)
    }

    fn by_user(connection : &DbConn, user : User) -> DisplayUser {
        let top10 = dbimpl::get_top10_us_adj_for_user(connection, &user).
            expect(&format!("User not found in the database. User name is : {}", user.user_name));
        let mut adj_counts : Vec<AdjCount> = Vec::new();
        for top in top10 {
            let adjective = dbimpl::get_adjective_by_id(&connection, &top.adjective_id)
                .expect(&format!("Adjective not found in the database. adjective_id : {}", top.adjective_id));
            let count = dbimpl::get_count(&connection, &user, &adjective)
                .expect(&format!("Count not found for user_id: {}  and adjective_id: {}", user.id, adjective.id));
            let adj_count = AdjCount { adjective, count : count.first().unwrap().count.unwrap() };
            adj_counts.push(adj_count)
        }
        DisplayUser { user, adj_count : adj_counts }
    }

}


#[derive(Serialize)]
pub struct UrsCount {
    pub user : User,
    pub count : i32,
}

#[derive(Serialize)]
pub struct DisplayAdjective {
    pub adjective : Adjective,
    pub usr_count : Vec<UrsCount>,
}

impl DisplayAdjective {
    pub fn by_id(connection : &DbConn, id : &i32) -> DisplayAdjective {
        let adjective = dbimpl::get_adjective_by_id(&connection, &id)
            .expect(&format!("User with id: {} no found!", id));
        DisplayAdjective::by_adjective(connection, adjective)
    }

    pub fn by_adjective_value(connection : &DbConn, value : &String) -> DisplayAdjective {
        let adjective = dbimpl::get_adjective_by_name(&connection, &value)
            .expect(&format!("User with name: {} no found!", value));
        DisplayAdjective::by_adjective(connection, adjective)
    }

    fn by_adjective(connection : &DbConn, adjective : Adjective) -> DisplayAdjective {
        let top10 = dbimpl::get_top10_us_adj_for_adjective(connection, &adjective).
            expect(&format!("Adjective not found in the database. adjective name is : {}", adjective.adjective_value));
        let mut usr_counts : Vec<UrsCount> = Vec::new();
        for top in top10 {
            let user = dbimpl::get_user_by_id(&connection, &top.user_id)
                .expect(&format!("User not found in the database. user_id : {}", top.user_id));
            let count = dbimpl::get_count(&connection, &user, &adjective)
                .expect(&format!("Count not found for user_id: {} and adjective_id: {}", adjective.id, adjective.id));
            let usr_count = UrsCount { user, count : count.first().unwrap().count.unwrap() };
            usr_counts.push(usr_count)
        }
        DisplayAdjective { adjective, usr_count : usr_counts }
    }
}


#[derive(Serialize)]
pub struct GenerateData {
    random_user_name : String,
    random_adjective_value : String,
}


impl GenerateData {
    pub fn new(connection : &DbConn, allDbData : &AllDbData) -> GenerateData {
        let random_adjective = RandomData::new(&allDbData.adjectives);
        let random_user = RandomData::new(&allDbData.users);
        create_or_update_us_adj(&connection, &random_user.value, &random_adjective.value);
        GenerateData {
            random_user_name : random_user.value.user_name,
            random_adjective_value : random_adjective.value.adjective_value,
        }
    }
}

impl<'r> Responder<'r> for GenerateData {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(
                Cursor::new(
                    format!("<a href=\"/user/{user}\">{user}</a> cегодня <a href=\"/adjective/{adjective}\">{adjective}</a>",
                            user = self.random_user_name, adjective = self.random_adjective_value)))
            .ok()
    }
}


#[derive(Clone)]
pub struct AllDbData {
    pub adjectives : Vec<Adjective>,
    pub users : Vec<User>,
}

impl AllDbData {
    pub fn new() -> AllDbData {
        let connection = init_pool().get().expect("Cannot connect to DB");
        let adjectives = dbimpl::all_adjectives(&connection).expect("Cannot get adjectives for database!");
        let users = dbimpl::all_users(&connection).expect("Cannot get users from database!");
        AllDbData { adjectives, users }
    }
}