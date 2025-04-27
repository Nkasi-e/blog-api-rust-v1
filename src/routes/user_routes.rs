use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::auth_middleware};



pub fn config(config: &mut web::ServiceConfig) {
   config
   .service(web::scope("/users")
   .wrap(from_fn(auth_middleware::check_auth_middleware))
    .service(handlers::user_handler::user)
   ); // passing the prefix of the home route
} 