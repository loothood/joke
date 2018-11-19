use diesel;
use diesel::{ pg::PgConnection
              , RunQueryDsl
              , prelude::*
};
use database::schema::*;
use database::models::{ user::User
                        , adjective::Adjective
                        , user_adjective::{ UserAdjective
                                            , NewUsAdRecord }
};
use failure::Error;

pub fn all_users(connection: &PgConnection) -> QueryResult<Vec<User>> {
    active_user::table.load::<User>(&*connection)
}

pub fn all_adjectives(connection: &PgConnection) -> QueryResult<Vec<Adjective>> {
    adjective::table.load::<Adjective>(&*connection)
}

pub fn get_user_by_id(connection : &PgConnection, id : &i32) -> QueryResult<User> {
    active_user::table
        .find(id)
        .get_result::<User>(connection)
}

pub fn get_user_by_name(connection : &PgConnection, name : &str) -> QueryResult<User> {
    active_user::table
        .filter(active_user::user_name.eq(name))
        .get_result(connection)
}

pub fn get_adjective_by_id(connection : &PgConnection, id : &i32) -> QueryResult<Adjective> {
    adjective::table
        .find(id)
        .get_result::<Adjective>(connection)
}

pub fn get_adjective_by_name(connection : &PgConnection, name : &str) -> QueryResult<Adjective> {
    adjective::table
        .filter(adjective::adjective_value.eq(name))
        .get_result(connection)
}

pub fn get_top10_all(conn : &PgConnection) -> Result<Vec<UserAdjective>, Error> {
    let query = user_adjective::table
        .order(user_adjective::user_id)
        .order(user_adjective::count.desc())
        .limit(10);

    let us_adj = query.load::<UserAdjective>(conn)?;

    Ok(us_adj)
}

pub fn get_count_of_repeats(conn : &PgConnection, user_id : &i32, adjective_id : &i32) -> Result<Vec<UserAdjective>, Error> {
    let query = user_adjective::table
        .filter(user_adjective::user_id.eq(user_id))
        .filter(user_adjective::adjective_id.eq(adjective_id));

    let us_adj = query.load::<UserAdjective>(conn)?;

    Ok(us_adj)
}


pub fn get_count(conn : &PgConnection, user : &User, adjective : &Adjective) -> Result<Vec<UserAdjective>, Error> {
    let query = user_adjective::table
        .filter(user_adjective::user_id.eq(user.id))
        .filter(user_adjective::adjective_id.eq(adjective.id));

    let us_adj = query.load::<UserAdjective>(conn)?;

    Ok(us_adj)
}


pub fn update_us_adj_record<'a> (conn : &PgConnection, user_id : &i32, adjective_id : &i32, count_of_repeats : &i32) -> UserAdjective {
    let count = count_of_repeats + 1;
    diesel::update(user_adjective::table
        .filter(user_adjective::user_id.eq(user_id))
        .filter(user_adjective::adjective_id.eq(adjective_id)))
        .set(user_adjective::count.eq(count))
        .get_result(conn)
        .expect("Error updating data in the users_adjectives table!")
}


pub fn create_us_adj_record<'a>(conn : &PgConnection, user_id : &'a i32, adjective_id : &'a i32) -> UserAdjective {
    let new_usad_record = NewUsAdRecord {
        user_id,
        adjective_id,
        count : &1i32
    };

    diesel::insert_into(user_adjective::table)
        .values(&new_usad_record)
        .get_result(conn)
        .expect("Error saving new record in the users_adjectives table!")
}


pub fn get_top10_us_adj_for_user(connection : &PgConnection, user : &User) -> Result<Vec<UserAdjective>, Error> {
    let query = user_adjective::table
        .filter(user_adjective::user_id.eq(user.id))
        .order(user_adjective::adjective_id)
        .order(user_adjective::count.desc())
        .limit(10);

    let us_adj : Vec<UserAdjective> = query.load::<UserAdjective>(connection)?;

    Ok(us_adj)
}

pub fn get_top10_us_adj_for_adjective(connection : &PgConnection, adjective : &Adjective) -> Result<Vec<UserAdjective>, Error> {
    let query = user_adjective::table
        .filter(user_adjective::adjective_id.eq(adjective.id))
        .order(user_adjective::user_id)
        .order(user_adjective::count.desc())
        .limit(10);

    let us_adj : Vec<UserAdjective> = query.load::<UserAdjective>(connection)?;

    Ok(us_adj)
}


