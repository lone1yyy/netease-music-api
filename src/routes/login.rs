use super::*;

pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(do_login_phone)
        .service(do_login_refresh)
        .service(do_login_status)
        .service(do_logout);
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

    ForwordRequest::new("login")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
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

    ForwordRequest::new("login")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
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

    let mut url = "https://music.163.com".to_string();

    ForwordRequest::new("do_login_status")
        .forward(Method::GET, &mut url, &mut data, &mut options, req, stream)
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
