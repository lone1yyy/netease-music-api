use super::*;

const DEFAULT_ID: &'static str = "32311";

pub fn album_config(cfg: &mut web::ServiceConfig) {
    cfg.service(album_detail_dynamic)
        .service(album_detail)
        .service(album_list_style)
        .service(album_list)
        .service(album_new)
        .service(album_newest)
        .service(album_songsale_board)
        .service(album_sub)
        .service(album_sublist)
        .service(album);
}

#[get("/album/detail/dynamic")]
async fn album_detail_dynamic(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"]
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/album/detail/dynamic".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/album/detail")]
async fn album_detail(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"]
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/vipmall/albumproduct/detail".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/album/list/style")]
async fn album_list_style(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "limit":query["limit"].as_u64().unwrap_or_else(||10),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true,
        "area":query["area"].as_str().unwrap_or_else(||"z_H")
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/vipmall/appalbum/album/style".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/album/list")]
async fn album_list(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "limit":query["limit"].as_u64().unwrap_or_else(||30),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true,
        "area":query["area"].as_str().unwrap_or_else(||"ALL"), //ALL:全部,ZH:华语,EA:欧美,KR:韩国,JP:日本
        "type":query["type"]
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/vipmall/albumproduct/list".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/album/new")]
async fn album_new(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "limit":query["limit"].as_u64().unwrap_or_else(||30),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true,
        "area":query["area"].as_str().unwrap_or_else(||"ALL") //ALL:全部,ZH:华语,EA:欧美,KR:韩国,JP:日本
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/album/new".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/album/newest")]
async fn album_newest(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({});

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/discovery/newAlbum".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/album/songsale/board")]
async fn album_songsale_board(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "albumType":query["albumType"].as_u64().unwrap_or_else(||0)
    });

    let queyy_type = query["type"].as_str().unwrap_or_else(|| "daily");
    if queyy_type == "year" {
        let obj = data.as_object_mut().unwrap();
        obj["year"] = query["year"].clone();
    }

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let url = "https://music.163.com/api/feealbum/songsaleboard/";
    let mut url = format!("{}/{}/type", url, queyy_type);

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

// 收藏/取消收藏专辑
#[get("/album/sub")]
async fn album_sub(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"]
    });

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let id = match query["t"].as_u64() {
        Some(id) => {
            if id == 1 {
                "sub"
            } else {
                "unsub"
            }
        }
        _ => "sub",
    };

    let url = "https://music.163.com/api/album";
    let mut url = format!("{}/{}", url, id);

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//已收藏专辑列表
#[get("/album/sublist")]
async fn album_sublist(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "limit":query["limit"].as_u64().unwrap_or_else(||25),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/album/sublist".to_string();

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//专辑内容
#[get("/album")]
async fn album(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({});

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let id = query["id"].as_str().unwrap_or_else(|| DEFAULT_ID);
    let url = "https://music.163.com/weapi/v1/album/";
    let mut url = format!("{}/{}", url, id);

    ForwordRequest::new("album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}
