use reqwest::{Client, Error};
use serde_json::json;
use flate2::read::GzDecoder;
use std::{io::Read, str::from_utf8};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let request_data = json!({
        "question": "how to make a web request in rust",
        "codeContext": "",
        "options": {
            "skill": "intermediate",
            "date": "16/04/2023",
            "language": "en-GB",
            "detailed": false,
            "creative": true
        }
    });

    let response = client
        .post("https://www.phind.com/api/infer/creative")
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-GB,en-US;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Origin", "https://www.phind.com")
        .header("Referer", "https://www.phind.com/search?q=Go+vs+Rust+vs+C%2B%2B&c=&source=searchbox&init=true")
        .header("sec-ch-ua", "\"Chromium\";v=\"112\", \"Google Chrome\";v=\"112\", \";Not A Brand\";v=\"99\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("Connection", "keep-alive")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-site")
        .header("cookie", "mp_{id}_mixpanel=text;__cf_bm=text")
        .json(&request_data)
        .send()
        .await?;

    let data: &[u8] = &response.bytes().await?.to_vec().as_slice().to_owned();
    let mut decoder = GzDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();
    println!("{:?}", from_utf8(&decompressed_data));
    Ok(())
}
