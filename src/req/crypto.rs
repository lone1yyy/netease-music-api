use openssl::rsa::{Padding, Rsa};
use openssl::symm::{Cipher, Crypter, Mode};

//base64
use base64;

//hex
use rustc_serialize::hex::ToHex;

//json
use serde_json::{json, Value};

//random
use rand::Rng;

use lazy_static::*;
lazy_static! {
    static ref IV: Vec<u8> = "0102030405060708".to_string().into_bytes();
    static ref PRESET_KEY: Vec<u8>  = "0CoJUm6Qyw8W8jud".to_string().into_bytes();
    static ref LINUX_API_KEY: Vec<u8>  = "rFgB&h#%2?^eDg:Q".to_string().into_bytes();
    static ref BASE62:String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    static ref PUBLIC_KEY:String = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----".to_string();
    static ref EAPI_KEY:Vec<u8> = "e82ckenh8dichen8".to_string().into_bytes();
    static ref EMPTY_IV:Vec<u8> = "".to_string().into_bytes();
}

pub fn aes_cbc_encrypt(buffer: Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    let mut encrypter = Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, key, Some(iv))
        .expect("fail to create encrypt");
    let block_size = Cipher::aes_128_cbc().block_size();
    let mut ciphertext = vec![0; buffer.len() + block_size];
    let mut count = encrypter
        .update(buffer.as_slice(), &mut ciphertext)
        .unwrap();
    count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);

    ciphertext
}

pub fn aes_ecb_encrypt(buffer: Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, Some(iv))
        .expect("fail to create encrypt");
    let block_size = Cipher::aes_128_ecb().block_size();
    let mut ciphertext = vec![0; buffer.len() + block_size];
    let mut count = encrypter
        .update(buffer.as_slice(), &mut ciphertext)
        .unwrap();
    count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);

    ciphertext
}

pub fn _aes_ecb_decrypt(buffer: Vec<u8>) -> Vec<u8> {
    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, &(*EAPI_KEY), None)
        .expect("fail to create decrypt");
    let block_size = Cipher::aes_128_ecb().block_size();
    let mut plaintext = vec![0; buffer.len() + block_size];
    let mut count = decrypter.update(buffer.as_slice(), &mut plaintext).unwrap();
    count += decrypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count);

    plaintext
}

pub fn _rsa_encrypt(buffer: Vec<u8>, key: &String) -> Vec<u8> {
    let rsa = Rsa::public_key_from_pem((key.clone()).into_bytes().as_slice()).unwrap();

    let mut encrypt_buffer = vec![0; 128 - buffer.len()];
    encrypt_buffer.append(&mut buffer.clone());
    let data = encrypt_buffer.as_slice();

    let mut buf = vec![0; rsa.size() as usize];
    rsa.public_encrypt(data, &mut buf, Padding::NONE).unwrap();
    buf
}

pub fn weapi(obj: Value) -> Value {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..16).map(|_| rng.gen_range(0, 255)).collect();

    let secret_key: String = bytes
        .iter()
        .map(|&n| BASE62.chars().nth(n as usize % 62).unwrap() as char)
        .collect();
    let r_key = secret_key.chars().rev().collect::<String>();

    json!({
            "params":base64::encode(aes_cbc_encrypt(
                base64::encode(aes_cbc_encrypt(obj.to_string().into_bytes(),&(*PRESET_KEY),&(*IV))).into_bytes(),
                &secret_key.into_bytes(),
                &(*IV)
            )),
            "encSecKey":_rsa_encrypt(r_key.into_bytes(),&(*PUBLIC_KEY)).to_hex()
    })
}

pub fn linuxapi(obj: Value) -> Value {
    json!({
        "eparams":aes_ecb_encrypt(obj.to_string().into_bytes(),&(*LINUX_API_KEY),&(*EMPTY_IV)).to_hex().to_uppercase()
    })
}

pub fn eapi(obj: Value, url: &String) -> Value {
    let text = obj.to_string();
    let message = format!("nobody{}use{}md5forencrypt", url, text);
    let digest = md5::compute(&message).to_hex();
    let data = format!("{}-36cd479b6b5-{}-36cd479b6b5-{}", url, text, digest);

    json!({
        "params":aes_ecb_encrypt(data.into_bytes(),&(*EAPI_KEY),&(*EMPTY_IV)).to_hex().to_uppercase(),
    })
}

pub fn crypto_test() {
    let _data: Value = json!({
        "phone":"17757156690",
        // "countrycode":"86",
        // "password":md5::compute("950926Wg").to_hex(),
        // "rememberLogin":"true",
        // "csrf_token":"",
    });

    let res = eapi(_data, &"www.baidu.com".to_string());
    println!("res is {:#?}", res);

    // let s = "{\"phone\":\"17757156690\",\"countrycode\":\"86\",\"password\":\"d16568c8aeeb4606f53d597408eec7f7\",\"rememberLogin\":\"true\",\"csrf_token\":\"09f260f44153079272eebc530f5ad769\"}".to_string();
    // let res = base64::encode(aes_cbc_encrypt(s.into_bytes(),&(*PRESET_KEY),&(*IV)));
    // println!("res is {:#?}",res);

    // let secret_key = "abcdefghijklmnop".to_string();
    // let r_key = secret_key.chars().rev().collect::<String>();
    // let res = base64::encode(aes_cbc_encrypt(res.into_bytes(),&secret_key.into_bytes(), &(*IV)));
    // println!("res is {:#?}", res);

    // let res2 = _rsa_encrypt(r_key.into_bytes(),&(*PUBLIC_KEY)).to_hex();
    // println!("res2 is {:#?}",res2);
}
