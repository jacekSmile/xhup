use anyhow::Error;
use crypto::digest::Digest;
use crypto::md5::Md5;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub msg: String,
    #[serde(rename = "list_dz")]
    pub list_dz: Vec<Vec<String>>,
    #[serde(rename = "list_ci")]
    pub list_ci: Vec<Value>,
    pub not_in_mb_list: Vec<Value>,
    pub no_result_count: i64,
}

fn search_word(word: String) -> Result<Info, Error> {
    let url = "http://www.xhup.club/Xhup/Search/searchCode";
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.0.0"));
    headers.insert("Accept", header::HeaderValue::from_static("*/*"));
    headers.insert(
        "Accept-Language",
        header::HeaderValue::from_static("en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7"),
    );

    let key = "fjc_xhup";
    let mut hasher = Md5::new();
    hasher.input_str(&format!("{key}{word}"));

    let client = reqwest::blocking::Client::new();
    let response: Info = client
        .post(url)
        .headers(headers)
        .form(&[("search_word", word), ("sign", hasher.result_str())])
        .send()?
        .json()?;

    Ok(response)
}

fn main() -> Result<(), Error> {
    let result = search_word(String::from("尔"))?;
    println!("{:?}", result);
    Ok(())
}
