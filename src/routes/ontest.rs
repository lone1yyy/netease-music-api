use super::*;
use rand::Rng;
use regex::Regex;

pub fn ontest_config(cfg: &mut web::ServiceConfig) {
    cfg.service(do_login)
        .service(do_login_phone)
        .service(do_login_refresh)
        .service(do_login_status)
        .service(do_logout)
        .service(user_playlist)
        .service(song_url)
        .service(playlist_detail)
        .service(song_detail);
}

use actix_web::dev::Body;
use futures::stream::StreamExt;
use rustc_serialize::json::{Json, ToJson};

async fn add_cookie(resp: HttpResponse) -> Result<HttpResponse, Error> {
    if resp.status().as_u16() != 200   {
        return Ok(resp);
    }
    let mut resp = resp;

    let str_cookie = resp.headers().get("cookie").unwrap().to_str().unwrap().to_string(); 
    if !str_cookie.is_empty() {
        let mut body = resp.take_body();
        let mut body_vec = Vec::new();
        while let Some(chunk) = body.next().await {
            body_vec.extend_from_slice(&chunk?);
        }
        let mut body = Json::from_str(String::from_utf8(body_vec).unwrap().as_str()).unwrap();
        let body = body.as_object_mut().unwrap();
        body.insert("cookie".to_string(), Json::String(str_cookie));

        let body_string = body.to_json().to_string();
        Ok(resp.set_body(Body::from_slice(body_string.as_bytes())))
    }else{
        Ok(HttpResponse::ExpectationFailed().finish())
    }
}

async fn _extract_info(resp: HttpResponse) -> Result<HttpResponse, Error> {
    let mut resp = resp;
    let mut body = resp.take_body();
    let mut body_vec = Vec::new();
    while let Some(chunk) = body.next().await {
        body_vec.extend_from_slice(&chunk?);
    }
    let body = String::from_utf8(body_vec).unwrap();

    let rgx = Regex::new(r"GUser\s*=\s*([^;]+);").unwrap();
    let mut profile = Json::Null;
    if let Some(caps) = rgx.captures(&body) {
        let caps = caps.get(1).unwrap().as_str();
        profile = Json::from_str(caps).unwrap();
    }

    let rgx = Regex::new(r"GBinds\s*=\s*([^;]+);").unwrap();
    let mut bindings = Json::Null;
    if let Some(caps) = rgx.captures(&body) {
        let caps = caps.get(1).unwrap().as_str();
        bindings = Json::from_str(caps).unwrap();
    }
    
    let mut body = std::collections::BTreeMap::new();
    body.insert("profile".to_string(),profile);
    body.insert("bindings".to_string(),bindings);
    body.insert("code".to_string(),Json::U64(200));

    let body_string = body.to_json().to_string();
    Ok(resp.set_body(Body::from_slice(body_string.as_bytes())))
}

#[get("/login")]
async fn do_login(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    let mut data: Value = json!({
        "phone":query["email"],
        "password":md5::compute(&query["password"].to_string()).to_hex(),
        "rememberLogin":"true",
    });

    let mut options = json!({
        "crypto": "weapi",
        "ua":"pc",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/login".to_string();

    let resp = ForwordRequest::new("login")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await?;
    add_cookie(resp).await
}

#[get("/login/cellphone")]
async fn do_login_phone(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    if query["countrycode"].is_null() {
        query["countrycode"] = Value::String("86".to_string());
    }

    let mut data: Value = json!({
        "phone":query["phone"],
        "countrycode":query["countrycode"],
        "password":md5::compute(query["password"].as_str().unwrap()).to_hex(),
        "rememberLogin":"true",
    });

    let mut options = json!({
        "crypto": "weapi",
        "ua":"pc",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/login/cellphone".to_string();

    let resp = ForwordRequest::new("login")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await?;
    add_cookie(resp).await
}

#[get("/login/refresh")]
async fn do_login_refresh(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    let mut data: Value = Value::Null;
    let mut options = json!({
        "crypto": "weapi",
        "ua":"pc",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/login/token/refresh".to_string();

    ForwordRequest::new("do_login_refresh")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/login/status")]
async fn do_login_status(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data = json!(null);
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/w/nuser/account/get".to_string();

    ForwordRequest::new("do_login_status")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/logout")]
async fn do_logout(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    let mut data: Value = Value::Null;
    let mut options = json!({
        "crypto": "weapi",
        "ua":"pc",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/logout".to_string();

    ForwordRequest::new("do_login_status")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/user/playlist")]
async fn user_playlist(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "uid":query["uid"],
        "limit":query["limit"].as_u64().unwrap_or_else(||30),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "includeVideo":true
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/user/playlist".to_string();

    ForwordRequest::new("artist")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/song/url")]
async fn song_url(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        if !cookie.contains_key("MUSIC_U") {
            let mut rng = rand::thread_rng();
            let bytes: String = (0..16)
                .map(|_| rng.gen_range(0, 255))
                .collect::<Vec<u8>>()
                .to_hex();
            cookie.insert("_ntes_nuid".to_string(), Value::from(bytes));
        }
        cookie.insert("os".to_string(), Value::from("pc"));
    }

    let mut data: Value = json!({
        "ids":format!("[{}]",query["id"].as_str().unwrap()),
        "br":query["br"].as_u64().unwrap_or_else(||999000),
    });

    let mut options = json!({
        "crypto": "eapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
        "url":"/api/song/enhance/player/url",
    });

    let mut url = "https://interface3.music.163.com/eapi/song/enhance/player/url".to_string();

    ForwordRequest::new("song_url")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/playlist/detail")]
async fn playlist_detail(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id": query["id"],
        "n":100000,
        "s":query["s"].as_u64().unwrap_or_else(||8)
    });

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/v6/playlist/detail".to_string();

    ForwordRequest::new("playlist_detail")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/song/detail")]
async fn song_detail(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let ids1 = query["ids"].as_str().unwrap_or_else(|| "");
    let ids2 = ids1.split(',').collect::<Vec<_>>();
    let ids3 = ids2
        .iter()
        .map(|&id| format!("{{\"id\":{}}}", id))
        .collect::<Vec<_>>()
        .join(",");

    let mut c = "[".to_string();
    c.push_str(&ids3);
    c.push(']');

    let ids4 = ids2.join(",");

    let mut ids = "[".to_string();
    ids.push_str(&ids4);
    ids.push(']');

    let mut data: Value = json!({
        "c" : c,
        "ids" : ids,
    });

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/v3/song/detail".to_string();

    ForwordRequest::new("song_detail")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}
