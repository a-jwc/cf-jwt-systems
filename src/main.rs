#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::collections::HashMap;
use std::fs::{self, File};

use chrono;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::Request;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("could not find '{}'", req.uri())
}

static PUB_PEM: &[u8] = include_bytes!("../assets/public.pem");
static PRIV_PEM: &[u8] = include_bytes!("../assets/private.pem");

// lazy_static! {
//   static ref USER_STATS_MAP: HashMap<String, [i16; 2]> = HashMap::new();
// }

#[get("/auth/<username>")]
fn create_jwt<'a>(username: String, jar: &'a CookieJar<'_>) -> Result<String, String> {
    let rsa_priv = EncodingKey::from_rsa_pem(PRIV_PEM).expect("Not a valid RSA key");
    let pub_pem = std::str::from_utf8(&PUB_PEM)
        .expect("Could not convert public RSA key bytes to string")
        .to_owned();
    let exp = (chrono::offset::Utc::now().timestamp() + (60 * 60 * 24)) as usize;
    let my_claims = Claims {
        sub: username.clone(),
        exp,
    };
    let token =
        encode(&Header::new(Algorithm::RS256), &my_claims, &rsa_priv).expect("Failed at encode");
    jar.add(
        Cookie::build("token", token.clone())
            .same_site(SameSite::None)
            .secure(true)
            .http_only(true)
            .finish(),
    );
    // let mut user_stats_map_copy = USER_STATS_MAP.clone();
    // if USER_STATS_MAP.contains_key(&username) {
    //     if let Some(user_stats) = USER_STATS_MAP.get(&username) {
    //         let mut user_stats_clone = user_stats.clone();
    //         let auth_stats = user_stats_clone.get_mut(0).unwrap();
    //         *auth_stats += 1;
    //         user_stats_map_copy.insert(username, [*auth_stats, *user_stats_clone.get(1).unwrap()]);
    //     } else {
    //         return Err("Could not find username in stats map".to_string());
    //     }
    // } else {
    //     user_stats_map_copy.insert(username, [1, 0]);
    // }
    println!("{}", token);
    Ok(pub_pem)
}

#[get("/verify")]
fn verify(jar: &CookieJar<'_>) -> Result<String, String> {
    let token = jar.get("token").unwrap().value().to_string();
    println!("{}", token);
    let rsa_pub = DecodingKey::from_rsa_pem(PUB_PEM).expect("Not a valid RSA key");
    let validation = Validation {
        leeway: 60,
        algorithms: vec![Algorithm::RS256],
        ..Default::default()
    };
    // println!("{:#?}", state.decoding_key);
    let token = decode::<Claims>(&token, &rsa_pub, &validation).unwrap_or(TokenData {
        header: Header::new(Algorithm::RS256),
        claims: Claims {
            sub: "Signature Invalid".to_string(),
            exp: 1000000000,
        },
    });
    if token.claims.sub == "Signature Invalid" {
        return Err("Signature Invalid".to_string());
    }
    println!("{}", token.claims.sub);
    Ok(token.claims.sub)
}

#[get("/README.txt")]
fn get_readme() -> Result<String, String> {
    let content = fs::read_to_string("README.txt").expect("could not open README.txt");
    Ok(content)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![verify, create_jwt, get_readme])
}
