use database::connection::DbConn;
use diesel::result::Error;
use database::{ dbimpl
                , models::{ user::User
                          , adjective::Adjective }
};
use rocket::http::Status;
use rocket::{ response::{ Failure
                          , NamedFile
                          , Redirect }
              , Request
              , State
};
use rocket_contrib::{ Json
                      , Template };
use middleware::display_data::*;

use std::{ collections::HashMap
           , path::{Path, PathBuf}
           , io::Error as StdIoError
};

#[get("/")]
fn index(connection : DbConn, allDbData : State<AllDbData>) -> Template {
    let context = IndexData::new(&connection, &allDbData);
    Template::render("index", &context)
}


#[get("/user/<id>")]
fn get_user_by_id(id: i32, connection: DbConn) -> Template {
    let context = DisplayUser::by_id(&connection, &id);
    Template::render("user", &context)
}


#[get("/user/<name>", rank = 2)]
fn get_user_by_name(name: String, connection: DbConn) -> Template {
    let context = DisplayUser::by_name(&connection, &name);
    Template::render("user", &context)
}

#[get("/adjective/<id>")]
fn get_adjective_by_id(id: i32, connection: DbConn) -> Template {
    let context = DisplayAdjective::by_id(&connection, &id);
    Template::render("adjective", &context)
}


#[get("/adjective/<name>", rank = 2)]
fn get_adjective_by_name(name: String, connection: DbConn) -> Template {
    let context = DisplayAdjective::by_adjective_value(&connection, &name);
    Template::render("adjective", &context)
}

#[post("/generate_new_one")]
fn generate_new_one(connection : DbConn, allDbData : State<AllDbData>) -> Option<GenerateData> {
    Some(GenerateData::new(&connection, &allDbData))
}

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> Result<NamedFile, StdIoError> {
     NamedFile::open(Path::new("static/").join(file))
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::MethodNotAllowed,
        _ => Status::InternalServerError
    })
}

#[get("/redirect")]
fn redirect() -> Redirect {
    Redirect::to("/")
}


#[get("/all_users")]
fn all_users(connection: DbConn) -> Result<Json<Vec<User>>, Failure> {
    dbimpl::all_users(&connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}


#[get("/all_adjectives")]
fn all_adjectives(connection: DbConn) -> Result<Json<Vec<Adjective>>, Failure> {
    dbimpl::all_adjectives(&connection)
        .map(|adjective| Json(adjective))
        .map_err(|error| error_status(error))
}