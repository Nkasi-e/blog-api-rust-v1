use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::auth_middleware};



pub fn config(config: &mut web::ServiceConfig) {
   config
   .service(web::scope("secure/post")
   .wrap(from_fn(auth_middleware::check_auth_middleware))
   .service(handlers::post_handler::create_post)
   .service(handlers::post_handler::my_post)
   )
   .service(web::scope("/post")
   .service(handlers::post_handler::all_post)
   .service(handlers::post_handler::single_post)
   ); // passing the prefix of the home route
} 