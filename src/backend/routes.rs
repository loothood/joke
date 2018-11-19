use rocket;
use database::connection;
use backend::handler;
use rocket_contrib::Template;
use middleware::display_data::AllDbData;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .manage(AllDbData::new())
        .mount("/",
               routes![ handler::index
                             , handler::all_users
                             , handler::all_adjectives
                             , handler::get_user_by_id
                             , handler::get_user_by_name
                             , handler::get_adjective_by_name
                             , handler::get_adjective_by_id
                             , handler::redirect
                             , handler::static_content
                             , handler::generate_new_one
                             ],
        )
        .attach(Template::fairing())
        .catch(catchers![handler::not_found])
        .launch();
}