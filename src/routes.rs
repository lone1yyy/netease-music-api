pub(crate) mod album;
pub(crate) mod artist;
pub(crate) mod login;
pub(crate) mod lyric;
pub(crate) mod ontest;
pub(crate) mod others;
pub(crate) mod user;

use super::ForwordRequest;
use actix_web::http::Method;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use queryst::parse;
use rustc_serialize::hex::ToHex;
use serde_json::{json, Value};

pub trait UrlQuery {
    fn query(&self) -> serde_json::Value;
}

impl UrlQuery for HttpRequest {
    fn query(&self) -> serde_json::Value {
        let query = self.uri().query().unwrap_or_else(|| "");
        let mut query = parse(query).unwrap_or_else(|_| json!({}));

        //cookie以参数形式提供,为了避免后面所有接口都要判断 这里就直接转成object吧
        if let Some(query) = query.as_object_mut() {
            if !query.contains_key("cookie") || query["cookie"].is_null() {
                query.insert("cookie".to_string(), json!(null));
            }

            if query["cookie"].is_string() {
                let mut res = json!(null);
                let cookie = query["cookie"].as_str().unwrap();
                if !cookie.is_empty() {
                    let cookie_arr = cookie.split(';').collect::<Vec<_>>();
                    for item in cookie_arr {
                        let arr = item.split('=').collect::<Vec<_>>();
                        if arr.len() == 2 {
                            res[arr[0]] = Value::from(arr[1]);
                        }
                    }
                }
                query["cookie"] = res;
            }
        }

        query
    }
}

pub trait CustomConfig {
    fn custom_config<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut web::ServiceConfig);
}

impl CustomConfig for web::ServiceConfig {
    fn custom_config<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut web::ServiceConfig),
    {
        f(self);
        self
    }
}
fn _whole_route_config(cfg: &mut web::ServiceConfig) {
    cfg.custom_config(login::login_config)
        .custom_config(artist::artist_config)
        .custom_config(user::user_config);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.custom_config(whole_route_config);

    const ON_TEST: bool = true;

    if ON_TEST {
        //流程测试接口
        cfg.custom_config(ontest::ontest_config);
    } else {
        cfg.custom_config(login::login_config)
            .custom_config(artist::artist_config)
            .custom_config(user::user_config)
            .custom_config(others::others_config)
            .custom_config(album::album_config);
    }
}
