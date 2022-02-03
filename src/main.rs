#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::fs;

use chrono;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("could not find '{}'", req.uri())
}

#[catch(401)]
fn unauthorized<'a>(_req: &'a Request) -> &'a str {
    "Invalid token"
}

static PUB_PEM: &[u8] = include_bytes!("../assets/public.pem");
static PRIV_PEM: &[u8] = include_bytes!("../assets/private.pem");

#[get("/auth/<username>")]
fn create_jwt<'a>(
    username: String,
    jar: &'a CookieJar<'_>,
) -> Result<(Status, String), (Status, String)> {
    let rsa_priv = EncodingKey::from_rsa_pem(PRIV_PEM).expect("Not a valid RSA key");
    let pub_pem = std::str::from_utf8(&PUB_PEM)
        .expect("Could not convert public RSA key bytes to string")
        .to_owned();
    let exp = (chrono::offset::Utc::now().timestamp() + (60 * 60 * 24)) as usize;
    let my_claims = Claims {
        sub: username.clone(),
        exp,
    };
    let token = match encode(&Header::new(Algorithm::RS256), &my_claims, &rsa_priv) {
        Ok(t) => t,
        Err(e) => {
            return Err((
                Status::FailedDependency,
                format!("Unable to encode. Error: {}", e),
            ))
        }
    };
    jar.add(
        Cookie::build("token", token.clone())
            .same_site(SameSite::None)
            .secure(true)
            .http_only(true)
            .finish(),
    );
    Ok((Status::Ok, pub_pem))
}

#[get("/verify")]
fn verify(jar: &CookieJar<'_>) -> Result<(Status, String), (Status, String)> {
    let token = match jar.get("token") {
        Some(c) => c.value().to_string(),
        None => return Err((Status::BadRequest, "Could not get token cookie".to_string())),
    };
    println!("{}", token);
    let rsa_pub = DecodingKey::from_rsa_pem(PUB_PEM).expect("Not a valid RSA key");
    let validation = Validation {
        leeway: 60,
        algorithms: vec![Algorithm::RS256],
        ..Validation::default()
    };
    if let Ok(token) = decode::<Claims>(&token, &rsa_pub, &validation) {
        println!("{}", token.claims.sub);
        Ok((Status::Ok, token.claims.sub))
    } else {
        Err((Status::Unauthorized, "Could not verify user".to_string()))
    }
}

#[get("/README.txt")]
fn get_readme() -> Result<String, String> {
    let content = fs::read_to_string("README.txt").expect("could not open README.txt");
    Ok(content)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, unauthorized])
        .mount("/", routes![verify, create_jwt, get_readme])
}
