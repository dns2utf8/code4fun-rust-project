#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;

use diesel::prelude::*;

use dotenv::dotenv;

use actix_web::{App, HttpRequest, web::Json, Result, http::Method, web::resource, web, HttpServer, middleware};

use code4fun::models::*;

fn read_file() -> std::io::Result<()> {
    let f = File::open("foo.txt")?;
    let f = BufReader::new(f);

    let mut i = 0;
    let re = Regex::new(r"Benutzer").expect("Malformed regular expression");

    for line in f.lines() {
        let line = line?;

        if re.is_match(&line) {
            i += 1;
        }
    }

    println!("Matches: {} lines", i);

    Ok(())
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

fn complex(_req: HttpRequest) -> Result<Json<MyObj>> {
    println!("Handling request");
    Ok(Json(MyObj{name: "Hello world".to_string()})) //req.match_info().query("name")?}))
}


fn divide(x: f32, y: f32) -> Result<f32, String> {      // Return type with "->" syntax
    if y == 0. {
        Err("Don't feel like dividing by 0".to_string())
    } else {
        Ok(x/y)
    }
}

fn print_sth(x: &String) {
    println!("{}", x);
}

fn get_employees(_req: HttpRequest) -> Result<Json<Vec<Employees>>> {
    use code4fun::schema::employees::dsl::*;
    let conn = code4fun::get_employees();

    let results = employees.limit(5)
        .load::<Employees>(&conn)
        .expect("Could not load employees");

    for r in &results {
        println!("{} {}", r.first_name, r.last_name);
        println!("{:?}", r);
    }

//    Ok(Json(results));
//    format!("Hello world");
    Ok(Json(results))
//    Ok(Json(vec![MyObj{name: "Hello world".to_string()}])) //req.match_info().query("name")?}))


}

fn index() -> String {
    format!("Hello world")
}

fn main() -> std::io::Result<()> {
    dotenv().expect("unable to parse .env");

    // Probably we can enable this via env variables
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(resource("/").route(web::get().to(get_employees)))
    })
        .bind("[::]:8080")?
        .run()
}
