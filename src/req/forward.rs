use actix_web::http;
use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
use actix_web::http::{Method, StatusCode};
use actix_web::{client, web, Error, HttpMessage, HttpRequest, HttpResponse};

use std::net::SocketAddr;
use std::time::Duration;

// use std::future::Future;
// use futures_core::stream::Stream;
// use futures_util::StreamExt;

use lazy_static::*;
lazy_static! {
    static ref HEADER_X_FORWARDED_FOR: HeaderName =
        HeaderName::from_lowercase(b"x-forwarded-for").unwrap();
    static ref HOP_BY_HOP_HEADERS: Vec<HeaderName> = vec![
        HeaderName::from_lowercase(b"connection").unwrap(),
        HeaderName::from_lowercase(b"proxy-connection").unwrap(),
        HeaderName::from_lowercase(b"keep-alive").unwrap(),
        HeaderName::from_lowercase(b"proxy-authenticate").unwrap(),
        HeaderName::from_lowercase(b"proxy-authorization").unwrap(),
        HeaderName::from_lowercase(b"te").unwrap(),
        HeaderName::from_lowercase(b"trailer").unwrap(),
        HeaderName::from_lowercase(b"transfer-encoding").unwrap(),
        HeaderName::from_lowercase(b"upgrade").unwrap(),
    ];
    static ref HEADER_TE: HeaderName = HeaderName::from_lowercase(b"te").unwrap();
    static ref HEADER_CONNECTION: HeaderName = HeaderName::from_lowercase(b"connection").unwrap();
    static ref HEADER_CREDENTIALS: HeaderName =
        HeaderName::from_lowercase(b"access-control-allow-credentials").unwrap();
    static ref HEADER_ORIGIN: HeaderName =
        HeaderName::from_lowercase(b"access-control-allow-origin").unwrap();
    static ref HEADER_HEADERS: HeaderName =
        HeaderName::from_lowercase(b"access-control-allow-headers").unwrap();
    static ref HEADER_METHODS: HeaderName =
        HeaderName::from_lowercase(b"access-control-allow-methods").unwrap();
    static ref HEADER_TYPE: HeaderName = HeaderName::from_lowercase(b"content-type").unwrap();
    static ref HEADER_LENGTH: HeaderName = HeaderName::from_lowercase(b"content-length").unwrap();
    static ref HEADER_COOKIE: HeaderName = HeaderName::from_lowercase(b"cookie").unwrap();
    static ref HEADER_SET_COOKIE: HeaderName = HeaderName::from_lowercase(b"set-cookie").unwrap();
}

static DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

pub struct ReverseProxy<'a> {
    _req_name: &'a str,
    forward_url: &'a str,
    timeout: Duration,
    method: Method,
    header: HeaderMap,
    query_body:String,
}

fn _add_client_ip(fwd_header_value: &mut String, client_addr: SocketAddr) {
    if !fwd_header_value.is_empty() {
        fwd_header_value.push_str(", ");
    }

    let client_ip_str = &format!("{}", client_addr.ip());
    fwd_header_value.push_str(client_ip_str);
}

fn _remove_connection_headers(headers: &mut HeaderMap) {
    let mut headers_to_delete: Vec<String> = Vec::new();
    let header_connection = &(*HEADER_CONNECTION);

    if headers.contains_key(header_connection) {
        if let Ok(connection_header_value) = headers.get(header_connection).unwrap().to_str() {
            for h in connection_header_value.split(',').map(|s| s.trim()) {
                headers_to_delete.push(String::from(h));
            }
        }
    }

    for h in headers_to_delete {
        headers.remove(h);
    }
}

fn _remove_request_hop_by_hop_headers(headers: &mut HeaderMap) {
    for h in HOP_BY_HOP_HEADERS.iter() {
        if headers.contains_key(h)
            && (headers.get(h).unwrap().to_str().unwrap() == ""
                || (h == *HEADER_TE && headers.get(h).unwrap().to_str().unwrap() == "trailers"))
        {
            continue;
        }
        headers.remove(h);
    }
}

fn add_custom_headers(headers: &mut HeaderMap, custom_headers: &HeaderMap) {
    headers.clear();
    for (k, v) in custom_headers {
        headers.insert(k.clone(), v.clone());
    }
}

fn add_response_preflight_headers(headers: &mut HeaderMap) {
    headers.insert(HEADER_CONNECTION.clone(), HeaderValue::from_static("true"));
    headers.insert(HEADER_ORIGIN.clone(), HeaderValue::from_static("*"));
    headers.insert(
        HEADER_HEADERS.clone(),
        HeaderValue::from_static("X-Requested-With,Content-Type"),
    );
    headers.insert(
        HEADER_METHODS.clone(),
        HeaderValue::from_static("PUT,POST,GET,DELETE,OPTIONS"),
    );
    headers.insert(
        HEADER_TYPE.clone(),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
}

impl<'a> ReverseProxy<'a> {
    pub fn new(forward_url: &'a str, _req_name: &'a str) -> ReverseProxy<'a> {
        ReverseProxy {
            forward_url,
            _req_name,
            timeout: DEFAULT_TIMEOUT,
            method: Method::default(),
            header: HeaderMap::new(),
            query_body: String::new(),
        }
    }

    pub fn timeout(mut self, duration: Duration) -> ReverseProxy<'a> {
        self.timeout = duration;
        self
    }

    pub fn method(mut self, method: Method) -> ReverseProxy<'a> {
        self.method = method;
        self
    }

    pub fn header(mut self, header: HeaderMap) -> ReverseProxy<'a> {
        self.header = header;
        self
    }

    pub fn query_body(mut self, query_body: String) -> ReverseProxy<'a> {
        self.query_body = query_body;
        self
    }

    fn _x_forwarded_for_value(&self, req: &HttpRequest) -> String {
        let mut result = String::new();

        for (key, value) in req.headers() {
            if key == *HEADER_X_FORWARDED_FOR {
                result.push_str(value.to_str().unwrap());
                break;
            }
        }

        if let Some(peer_addr) = req.peer_addr() {
            _add_client_ip(&mut result, peer_addr);
        }

        result
    }

    fn _forward_uri(&self, req: &HttpRequest) -> String {
        let forward_url: &str = self.forward_url;
        let forward_uri = match req.uri().query() {
            Some(query) => format!("{}&{}", forward_url, query),
            None => forward_url.to_string(),
        };
        forward_uri.to_string()
    }

    pub async fn forward(
        &self,
        req: HttpRequest,
        _stream: web::Payload,
    ) -> Result<HttpResponse, Error> {
        if req.method() == Method::OPTIONS {
            Ok(HttpResponse::build(StatusCode::NO_CONTENT).finish())
        } else {
            let connector = client::Connector::new().timeout(self.timeout).finish();

            let mut forward_req = client::ClientBuilder::new()
                .no_default_headers()
                .connector(connector)
                .timeout(self.timeout)
                .finish()
                .request_from(self.forward_url, req.head())
                .method(self.method.clone());

            add_custom_headers(forward_req.headers_mut(), &self.header);

            if cfg!(debug_assertions) {
                println!("forward_uri is : {}", self.forward_url);
            }

            if cfg!(debug_assertions) {
                println!("\n\r#### REVERSE PROXY REQUEST HEADERS ####");
                for (key, value) in forward_req.headers() {
                    println!("[{:?}] = {:?}", key, value);
                }
                println!("#### REVERSE PROXY REQUEST HEADERS ####\n\r");
            }
            
            // let mut bytes = web::BytesMut::new();
            // while let Some(item) = stream.next().await {
            //     bytes.extend_from_slice(&item?);
            // }
            // let r = format!("Body {:?}!", bytes);
            // println!("body is {}", r);

            forward_req
                .send_body(self.query_body.clone())
                .await
                .map_err(|error| {
                    println!("Error: {}", error);
                    error.into()
                })
                .and_then(|mut resp| {
                    let mut back_rsp = HttpResponse::build(resp.status());

                    //extract and set cookie
                    let mut set_cookie = resp
                    .headers()
                    .get_all(http::header::SET_COOKIE);
                
                    let mut str_cookie = String::new();
                    while let Some(ck) = set_cookie.next() {
                        let ck = ck.to_str().unwrap();
                        let cookie = regex::Regex::new(r"\s*Domain=[^(;|$)]+;*")
                        .unwrap()
                        .replace_all(ck, "")
                        .into_owned();
                        str_cookie.push_str(&cookie);
                    };
                    if !str_cookie.is_empty() {
                        back_rsp.set_header(http::header::COOKIE, str_cookie);
                    }

                    //copy header
                    // for (key, value) in resp.headers() {
                    //     back_rsp.header(key.clone(), value.clone());
                    // }

                    let mut back_rsp = back_rsp.streaming(resp.take_payload());
                    add_response_preflight_headers(back_rsp.headers_mut());

                    if cfg!(debug_assertions) {
                        println!("\n\r#### REVERSE PROXY RESPONSE HEADERS ####");
                        for (key, value) in back_rsp.headers() {
                            println!("[{:?}] = {:?}", key, value);
                        }
                        println!("#### REVERSE PROXY RESPONSE HEADERS ####\n\r");
                    }

                    Ok(back_rsp)
                })
        }
    }
}
