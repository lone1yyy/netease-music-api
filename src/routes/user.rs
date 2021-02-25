use super::*;

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_account).service(user_playlist);
}

//用户账户
#[get("/user/account")]
async fn user_account(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({});
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/nuser/account/get".to_string();

    ForwordRequest::new("artist")
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
