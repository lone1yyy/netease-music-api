use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::HashMap;
use url::form_urlencoded;

use actix_web::http::Method;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use std::time::Duration;

use super::crypto;
use super::forward;
use super::CheckNull;
use super::CookieString;

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

use lazy_static::*;
lazy_static! {
    static ref HASHMAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        let mobile_list = vec![
            // iOS 13.5.1 14.0 beta with safari
            "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.",
            // iOS with qq micromsg
            "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/602.1.50 (KHTML like Gecko) Mobile/14A456 QQ/6.5.7.408 V1_IPH_SQ_6.5.7_1_APP_A Pixel/750 Core/UIWebView NetType/4G Mem/103",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/7.0.15(0x17000f27) NetType/WIFI Language/zh",
            // Android -> Huawei Xiaomi
            "Mozilla/5.0 (Linux; Android 9; PCT-AL10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.64 HuaweiBrowser/10.0.3.311 Mobile Safari/537.36",
            "Mozilla/5.0 (Linux; U; Android 9; zh-cn; Redmi Note 8 Build/PKQ1.190616.001) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/71.0.3578.141 Mobile Safari/537.36 XiaoMi/MiuiBrowser/12.5.22",
            // Android + qq micromsg
            "Mozilla/5.0 (Linux; Android 10; YAL-AL00 Build/HUAWEIYAL-AL00; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/78.0.3904.62 XWEB/2581 MMWEBSDK/200801 Mobile Safari/537.36 MMWEBID/3027 MicroMessenger/7.0.18.1740(0x27001235) Process/toolsmp WeChat/arm64 NetType/WIFI Language/zh_CN ABI/arm64",
            "Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BKK-AL10 Build/HONORBKK-AL10) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/66.0.3359.126 MQQBrowser/10.6 Mobile Safari/537.36",
        ];

        let pc_list = vec![
        // macOS 10.15.6  Firefox / Chrome / Safari
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:80.0) Gecko/20100101 Firefox/80.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.30 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.2 Safari/605.1.15",
        // Windows 10 Firefox / Chrome / Edge
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:80.0) Gecko/20100101 Firefox/80.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.30 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.10586",
        // Linux 就算了
        ];

        let mut all_list = Vec::new();
        all_list.append(&mut mobile_list.clone());
        all_list.append(&mut pc_list.clone());

        m.insert("mobile",mobile_list);
        m.insert("pc",pc_list);
        m.insert("all",all_list);

        m
    };
    static ref COUNT: usize = HASHMAP.len();

    //headers
    static ref HEADER_USER_AGENT: HeaderName = HeaderName::from_lowercase(b"user-agent").unwrap();
    static ref HEADER_CONTENT_TYPE: HeaderName = HeaderName::from_lowercase(b"content-type").unwrap();
    static ref HEADER_REFERER: HeaderName = HeaderName::from_lowercase(b"referer").unwrap();
    static ref HEADER_COOKIE: HeaderName = HeaderName::from_lowercase(b"cookie").unwrap();
    static ref HEADER_X_REAL_IP: HeaderName = HeaderName::from_lowercase(b"x-real-ip").unwrap();
    static ref HEADER_MUSIC_U: HeaderName = HeaderName::from_lowercase(b"music_u").unwrap();
    static ref HEADER_MUSIC_A:HeaderName = HeaderName::from_lowercase(b"music_a").unwrap();
}

impl CheckNull for Value {
    fn check_null(&self, key: &str) -> bool {
        if !self.is_null() && self.is_object() && self.as_object().unwrap().contains_key(key) {
            return self.as_object().unwrap()[key].is_null();
        }
        true
    }
}

impl CookieString for Value {
    fn as_cookie_string(&self) -> String {
        if self.is_object() {
            let mut encoded_cookie = String::new();
            for pair in self.as_object().unwrap() {
                if pair.1.is_null() {
                    continue;
                }
                let key = pair.0;
                let value = pair.1.as_str().unwrap();
                encoded_cookie.push_str(key);
                encoded_cookie.push('=');
                encoded_cookie.push_str(value);
                encoded_cookie.push(';');
            }
            encoded_cookie
        } else{
            panic!("not implemented yet");
        }
    }
}

pub struct ForwordRequest {
    _req_name: &'static str,
}

impl ForwordRequest {
    pub fn new(req_name: &'static str) -> Self {
        ForwordRequest {
            _req_name: req_name,
        }
    }

    pub async fn forward(
        &self,
        method: Method,
        url: &mut String,
        data: &mut Value,
        options: &mut Value,
        req: HttpRequest,
        stream: web::Payload,
    ) -> Result<HttpResponse, Error> {
        let mut header = HeaderMap::new();
        let query_body = self.create_query_body(method.as_str().to_string(), url, data, options, &mut header);
        

        forward::ReverseProxy::new(url, self._req_name)
            .timeout(Duration::from_secs(60))
            .method(method)
            .header(header)
            .query_body(query_body)
            .forward(req, stream)
            .await
    }

    fn choose_user_agent(&self, ua: &str) -> &'static str {
        if !HASHMAP.contains_key(ua) {
            panic!("impossible")
        }
        HASHMAP[ua].choose(&mut rand::thread_rng()).unwrap()
    }
    fn create_query_body(
        &self,
        method: String,
        url: &mut String,
        data: &mut Value,
        options: &mut Value,
        headers: &mut HeaderMap,
    ) -> String {
        let user_agent = self.choose_user_agent(options["ua"].as_str().unwrap_or_else(|| "all"));

        headers.insert(
            HEADER_USER_AGENT.clone(),
            HeaderValue::from_static(user_agent),
        );
        if method.to_uppercase().eq("POST") {
            headers.insert(
                HEADER_CONTENT_TYPE.clone(),
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        if url.contains("music.163.com") {
            headers.insert(
                HEADER_REFERER.clone(),
                HeaderValue::from_static("https://music.163.com"),
            );
        }
        if !options["realIp"].is_null() {
            headers.insert(
                HEADER_X_REAL_IP.clone(),
                HeaderValue::from_str(&options["realIp"].as_str().unwrap()).unwrap(),
            );
        }

        if options["cookie"].is_object() {
            let encoded_cookie = options["cookie"].as_cookie_string();
            headers.insert(
                HEADER_COOKIE.clone(),
                HeaderValue::from_str(&encoded_cookie).unwrap(),
            );
        } else if options["cookie"].is_string() {
            headers.insert(
                HEADER_COOKIE.clone(),
                HeaderValue::from_str(options["cookie"].as_str().unwrap()).unwrap(),
            );
        }

        if !headers.contains_key(&(*HEADER_COOKIE)) && options["token"].is_string() {
            headers.insert(
                HEADER_COOKIE.clone(),
                HeaderValue::from_str(options["token"].as_str().unwrap()).unwrap(),
            );
        }

        if !options["crypto"].is_null() {
            let crypto = options["crypto"].as_str().unwrap();
            match crypto {
                "weapi" => {
                    let cookie = match headers.get(&(*HEADER_COOKIE)) {
                        Some(c) => c.to_str().unwrap(),
                        _ => "",
                    };
                    let csrf_token =
                        match Regex::new(r"_csrf=([^(;|$)]+)").unwrap().captures(cookie) {
                            Some(caps) => caps.get(0).unwrap().as_str().to_string(),
                            _ => "".to_string(),
                        };

                    data["csrf_token"] = Value::String(csrf_token);
                    *data = crypto::weapi(data.clone());
                    *url = Regex::new(r"\w*api")
                        .unwrap()
                        .replace_all(&url, "weapi")
                        .into_owned();
                }
                "linuxapi" => {
                    *data = crypto::linuxapi(json!({
                        "method" : method,
                        "url" : Regex::new(r"\w*api").unwrap().replace_all(&url,"api").into_owned(),
                        "params" : data.clone()
                    }));

                    headers.insert(HEADER_USER_AGENT.clone(),HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36"));
                    *url = String::from("https://music.163.com/api/linux/forward");
                }
                "eapi" => {
                    let mut cookie = json!({});
                    if headers.contains_key(&(*HEADER_COOKIE)) {
                        cookie = options["cookie"].clone();
                    }

                    let mut csrf_token = Value::from("");
                    if !cookie.check_null("__csrf") {
                        csrf_token = cookie["__csrf"].clone();
                    }

                    let mut now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        .to_string();
                    now.push('_');

                    let random = rand::thread_rng().gen_range(0, 1000);
                    let random = format!("{:0>4}", random);

                    now.push_str(&random);

                    let header = json!({
                        "osver" : cookie["osver"],
                        "deviceId" : cookie["deviceId"],
                        "appver" :
                            if cookie.check_null("appver") {
                                Value::from("6.1.1")
                            }
                            else {
                                cookie["appver"].clone()
                            }
                        ,
                        "versioncode" :
                            if cookie.check_null("versioncode") {
                                Value::from("140")
                            }
                            else {
                                cookie["versioncode"].clone()
                            }
                        ,
                        "mobilename" : cookie["mobilename"],
                        "buildver" :
                            if cookie.check_null("buildver") {
                                Value::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string())
                            }
                            else{
                                cookie["buildver"].clone()
                            }
                        ,
                        "resolution" :
                            if cookie.check_null("resolution") {
                                Value::from("1920x1080")
                            }
                            else{
                                cookie["resolution"].clone()
                            }
                        ,
                        "__csrf" : csrf_token,
                        "os" :
                            if cookie.check_null("os") {
                                Value::from("android")
                            }
                            else {
                                cookie["os"].clone()
                            }
                        ,
                        "channels" : cookie["channels"],
                        "requestId" : now,
                    });

                    if !cookie.check_null("MUSIC_U") {
                        headers.insert(
                            HEADER_MUSIC_U.clone(),
                            HeaderValue::from_str(cookie["MUSIC_U"].as_str().unwrap()).unwrap(),
                        );
                    }

                    if !cookie.check_null("MUSIC_A") {
                        headers.insert(
                            HEADER_MUSIC_A.clone(),
                            HeaderValue::from_str(cookie["MUSIC_A"].as_str().unwrap()).unwrap(),
                        );
                    }

                    let encoded_cookie = header.as_cookie_string();
                    headers.insert(
                        HEADER_COOKIE.clone(),
                        HeaderValue::from_str(&encoded_cookie).unwrap(),
                    );

                    data["header"] = header;
                    *data =
                        crypto::eapi(data.clone(), &options["url"].as_str().unwrap().to_string());
                    *url = Regex::new(r"\w*api")
                        .unwrap()
                        .replace_all(&url, "eapi")
                        .into_owned();
                }
                // _ => panic!("impossible"),
                _ => println!("what happend?"),
            };
        }
        
        let query_body = if data.as_object().is_some() {
            let query = data.as_object().unwrap();
            let mut encoded_query = form_urlencoded::Serializer::new(String::new());
            for (key, value) in query.iter() {
                encoded_query.append_pair(key, value.as_str().unwrap());
            }
            encoded_query.finish()
        } else {
            String::new()
        };
        query_body
    }
}

pub fn _json_test() {
    //create_
    let s1 = json!({
        "a" : 100,
        "v" : false
    });

    //serialize
    let response = s1.to_string();
    println!("[返回数据]：{}", response)
}
