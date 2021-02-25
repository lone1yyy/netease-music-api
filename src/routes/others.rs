use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

const RAW_DATA:&'static str = "eJx10mtIU2EcBvDtnCwNMfO2klUSmSQ5ZugKW/v/0TIjJVdhDStbXpqXrhY5Kwhtrcwiut9VSqMUMxX6IFqsD92sD1YgWGHRBcowKrpnPa/v+drg4flt572ds2PQ6XQut7MwJ940w2TOyS0pzF+/BV/MJrNO+3TVLOHUzKx5iw3/H5uZ7yxegct3tTl7Cr6QEa0gZ/dZOFsvfe5YHe1D+yFZxpncqEj/cCdwoirdVxHNnZrX3xygU5g7Eh6I9uOx8Ch4y9FQjlKkDz1pYrFXIJLUOovFGcYivqJgXqaXDqu7Rzc0XzmZxG81B/fF8wRVusn2jN5rDnwca8tFhyAJP4L4qiI9vX8cWzEmVKzT/46qxNpIdZOZz2HNcHhSkZ3D4AjYFpfGFkX6+dB+FvcSBe/SWbkLPVnEOJ1DFelXxVVci/Wj4TsBLhrQ/LGoaU4HxsTA28L76Cc8Dfau/U6F6FgkyBDDJar0g8tesmOvOHioWeXXmme6l3MLbIIre6wciU5E2t/k8WVxHfHvuUWXsH4SPCv1NW1Cz0aivgYO34vw1AEvi3MlIw0xHl6JNVPEGW41UJsqPaXYYTuEnotMdHwYfv7CFR/i+aXmrY5wrlSkEwr+0EJ0GvLmdw4/RS9Amj93UAbGZMIF40ezE3PtcG/yBWrT3L6oh66hFyMXK4xsUKT7aufzapxnFTwiNc3Wis5Bdm+OYCvmOuHj/ZeoQPOI00PUrUjXpG+kMFU61tFFDvQaZOn5DH4mzoLw4Hsaj14rzu/K4jF66fSWTnJinW3wBvcveqjZN3iFjKp0qKuF1mi21keST3NtTcbwu1eG3Dussr9eemljLIco0tVH7HwA493wOr+FlIjfy+GvkR4uwfjt4v/6G8K3NX8K38lt6B1ISa+Bv2O8Fy69foZOovci2S4Lr1aku4P9OEWVTt9wgMQ7exgJ8JXyI0W694WFyuBjcH75XyrEXsfhg+ZSvqZIf/Lct8Wp0md2tJN4PifEfjcm8gu02Ptbj459eum8eg8bFWlLXTb/A+uo9bM=";

pub fn others_config(cfg: &mut web::ServiceConfig) {
    cfg.service(activate_init_profile)
        .service(audio_match)
        .service(banner)
        .service(calendar);
}

#[get("/activate/initProfile")]
async fn activate_init_profile(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "phone":query["nickanme"]
    });

    let mut options = json!({
        "crypto": "eapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
        "url": "/api/activate/initProfile",
    });

    let mut url = "https://music.163.com/eapi/activate/initProfile".to_string();

    ForwordRequest::new("activate_init_profile")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/audio/match")]
async fn audio_match(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let mut query = req.query();

    if let Some(cookie) = query["cookie"].as_object_mut() {
        cookie.insert("os".to_string(), Value::String("pc".to_string()));
    } else {
        query["cookie"] = json!({
            "os":"pc"
        });
    }

    let mut data: Value = json!({
        "algorithmCode":"shazam_v2",
        "times":1,
        "sessionId":"C999431ACDC84EDBB984763654E6F8D7",
        "duration":3.3066249999999995,
        "from":"recognize-song",
        "rawdata":RAW_DATA
    });

    let mut options = json!({
        "crypto": "eapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/music/audio/match".to_string();

    ForwordRequest::new("audio_match")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//TODO avatar_upload

//首页轮播图
#[get("/banner")]
async fn banner(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let stype_vec = vec!["pc", "android", "iphone", "ipad"];

    let mut stype = "pc";
    if let Some(index) = query["type"].as_u64() {
        let mut index = index as usize;
        if index >= stype_vec.len() {
            index = 0;
        }
        stype = stype_vec[index];
    }

    let mut data: Value = json!({
        "clientType":stype,
    });

    let mut options = json!({
        "crypto": "weapi",
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/v2/banner/get".to_string();

    ForwordRequest::new("banner")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

//TODO batch

#[get("/calendar")]
async fn calendar(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "startTime":query["startTime"].as_u64().unwrap_or_else(||SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64),
        "endTime":query["endTime"].as_u64().unwrap_or_else(||SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
    });

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/api/mcalendar/detail".to_string();

    ForwordRequest::new("calendar")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}

#[get("/captcha_sent")]
async fn captcha_sent(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query = req.query();

    let mut data: Value = json!({
        "ctcode":query["ctcode"].as_str().unwrap_or_else(||"86"),
        "cellphone":query["cellphone"]
    });

    let mut options = json!({
        "crypto": "weapi",
        "cookie":query["cookie"],
        "proxy":query["proxy"],
        "realIp":query["realIp"],
    });

    let mut url = "https://music.163.com/weapi/sms/captcha/sent".to_string();

    ForwordRequest::new("captcha_sent")
        .forward(Method::POST, &mut url, &mut data, &mut options, req, stream)
        .await
}
