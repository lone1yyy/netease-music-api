use super::*;

#[get("/lyric")]
async fn get_lyric(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    let mut data: Value = json!({
        "id":query["id"],
        "lv":-1,
        "kv":-1,
        "tv":-1
    });

    let mut options = json!({
        "crypto": "linuxapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/logout".to_string();

    ForwordRequest::new("lyric")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}
