extern crate tujiaoapi;
#[macro_use] extern crate nickel;
extern crate rustc_serialize as serialize;
extern crate jsonway;
extern crate mysql;
extern crate nickel_mysql;

use std::path::Path;
use nickel::{Nickel, HttpRouter, MediaType};
use nickel_mysql::{MysqlMiddleware, MysqlRequestExtensions};

use tujiaoapi::service::user_service::UserService;
use tujiaoapi::database::get_db_pool;
use tujiaoapi::model::user::*;
use serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

fn main() {
    let mut app = Nickel::new();
    app.utilize(MysqlMiddleware::new("tujiao", "root", "123456"));

    app.get("/user/:userid", middleware! { |request|
        format!("This is user: {:?}", request.param("userid"))
    });

    app.get("/v1/user/list", middleware! { |request, mut response|
        //response.set(MediaType::Json);

        let connection = request.db_connection();
        let mut user_service = UserService::new(connection);
        let users: Vec<User> = user_service.get_users();

        let json = jsonway::array(|json| {
            json.objects(users.iter(), |user, json| {
                json.set("id".to_string(), user.id.to_string());
                json.set("email".to_string(), user.email.as_ref().unwrap().to_string());
            })
        }).unwrap();
        let mut json_extras = BTreeMap::new();
        json_extras.insert("totalCount".to_string(), "100".to_json());
        json_extras.insert("users".to_string(), json);
        Json::Object(json_extras)

    });

    //app.listen("0.0.0.0:6767");

    let cert_path = Path::new("/Users/arden/data/repository/trunk/tj/server/tujiao-api/ssl/certificate.pem");
    let key_path = Path::new("/Users/arden/data/repository/trunk/tj/server/tujiao-api/ssl/privatekey.pem");
    app.listen_https("0.0.0.0:6767", cert_path, key_path);
}

