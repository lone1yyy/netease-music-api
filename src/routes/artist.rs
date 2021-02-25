use super::*;

const DEFAULT_ID: &'static str = "6452";

pub fn artist_config(cfg: &mut web::ServiceConfig) {
    cfg.service(artist)
        .service(artist_album)
        .service(artist_desc)
        .service(artist_detail)
        .service(artist_list)
        .service(artist_mvs)
        .service(artist_songs)
        .service(artist_sub)
        .service(artist_sublist)
        .service(artist_top_song);
}

//歌手单曲
#[get("/artist")]
async fn artist(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({});
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let id = query["id"].as_str().unwrap_or_else(|| DEFAULT_ID);
    let url = "https://music.163.com/weapi/v1/artist";
    let mut url = format!("{}/{}", url, id);

    ForwordRequest::new("artist")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//歌手专辑列表
#[get("/artist/album")]
async fn artist_album(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "limit":query["limit"].as_str().unwrap_or_else(||"30"),
        "offset":query["offset"].as_str().unwrap_or_else(||"0"),
        "total":"true"
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let id = query["id"].as_str().unwrap_or_else(|| DEFAULT_ID);
    let url = "https://music.163.com/weapi/artist/albums";
    let mut url = format!("{}/{}", url, id);

    ForwordRequest::new("artist_album")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//歌手介绍
#[get("/artist/desc")]
async fn artist_desc(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"].as_str().unwrap_or_else(||DEFAULT_ID),
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/artist/introduction".to_string();

    ForwordRequest::new("artist_desc")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//详情
#[get("/artist/detail")]
async fn artist_detail(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"].as_str().unwrap_or_else(||DEFAULT_ID),
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/artist/head/info/get".to_string();

    ForwordRequest::new("artist_detail")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

// 歌手分类

/*
    type 取值
    1:男歌手
    2:女歌手
    3:乐队

    area 取值
    -1:全部
    7华语
    96欧美
    8:日本
    16韩国
    0:其他

    initial 取值 a-z/A-Z
*/
#[get("/artist/list")]
async fn artist_list(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let initial = query["initial"]
        .as_str()
        .unwrap_or_else(|| "A")
        .to_uppercase()
        .into_bytes()
        .first()
        .unwrap()
        .clone();

    let mut data: Value = json!({
        "initial":initial,
        "limit":query["limit"].as_u64().unwrap_or_else(||30),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true,
        "type":query["type"].as_str().unwrap_or_else(||"1"),
        "area":query["area"]
    });

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/v1/artist/list".to_string();

    ForwordRequest::new("artist_list")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//歌手相关mv
#[get("/artist/mvs")]
async fn artist_mvs(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"].as_str().unwrap_or_else(||DEFAULT_ID),
        "limit":query["limit"].as_u64().unwrap_or_else(||30),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true,
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/artist/mvs".to_string();

    ForwordRequest::new("artist_mv")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/artist/songs")]
async fn artist_songs(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    let mut data: Value = json!({
        "id":query["id"].as_str().unwrap_or_else(||DEFAULT_ID),
        "private_cloud":"true",
        "work_type":1,
        "order":query["order"].as_str().unwrap_or_else(||"hot"),
        "limit":query["limit"].as_u64().unwrap_or_else(||100),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "`https://music.163.com/api/v1/artist/songs".to_string();

    ForwordRequest::new("artist_songs")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//收藏与取消收藏歌手
#[get("/artist/sub")]
async fn artist_sub(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "artistId":query["id"].as_str().unwrap_or_else(||DEFAULT_ID),
        "artistIds":format!("[{}]",query["id"].as_str().unwrap_or_else(||DEFAULT_ID)),
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let t = query["t"].as_str().unwrap_or_else(|| "") == "1";
    let t = if t { "sub" } else { "unsub" };
    let url = "https://music.163.com/weapi/artist";
    let mut url = format!("{}/{}", url, t);

    ForwordRequest::new("artist_sub")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//关注歌手列表
#[get("/artist/sublist")]
async fn artist_sublist(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "limit":query["limit"].as_u64().unwrap_or_else(||25),
        "offset":query["offset"].as_u64().unwrap_or_else(||0),
        "total":true,
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/artist/sublist".to_string();

    ForwordRequest::new("artist_sublist")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//歌手热门50首歌曲
#[get("/artist/top/song")]
async fn artist_top_song(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "id":query["id"].as_str().unwrap_or_else(||DEFAULT_ID),
    });
    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/artist/top/song".to_string();

    ForwordRequest::new("artist_top_song")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}
