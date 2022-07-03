use crate::server::entity::NewsArticle;
use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize)]
struct Response {
    data: Vec<Data>,
    included: Vec<Value>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    id: String,
    attributes: Value,
}

#[cfg(target_arch = "x86_64")]
pub fn fetch_articles_ureq() -> Vec<NewsArticle> {
    let url = "https://www.gcores.com/gapi/v1/articles?page[limit]=10&sort=-published-at&include=category,user&filter[is-news]=true&fields[articles]=title,desc,is-published,thumb,app-cover,cover,comments-count,likes-count,bookmarks-count,is-verified,published-at,option-is-official,option-is-focus-showcase,duration,category,user";
    let resp = ureq::get(url)
        .call()
        .unwrap()
        .into_json::<Response>()
        .unwrap();
    let res: Vec<NewsArticle> = resp.into();
    tracing::info!("获取 {} 条资讯", res.len());
    res
}

#[cfg(target_arch = "x86_64")]
pub async fn fetch_articles() -> Vec<NewsArticle> {
    let url = "https://www.gcores.com/gapi/v1/articles?page[limit]=10&sort=-published-at&include=category,user&filter[is-news]=true&fields[articles]=title,desc,is-published,thumb,app-cover,cover,comments-count,likes-count,bookmarks-count,is-verified,published-at,option-is-official,option-is-focus-showcase,duration,category,user";
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();
    let res: Vec<NewsArticle> = resp.into();
    tracing::info!("获取 {} 条资讯", res.len());
    res
}

impl From<Response> for Vec<NewsArticle> {
    fn from(resp: Response) -> Self {
        let mut res = vec![];
        for (data, included) in resp.data.iter().zip(resp.included.iter()) {
            let url = format!("https://www.gcores.com/articles/{}", data.id);
            let article = NewsArticle {
                id: data.id.clone(),
                title: data.attributes["title"].as_str().unwrap().to_owned(),
                desc: data.attributes["desc"].as_str().unwrap().to_owned(),
                url,
                author: included["attributes"]["nickname"]
                    .as_str()
                    .unwrap_or("default")
                    .to_owned(),
                time: data.attributes["published-at"].as_str().unwrap().to_owned(),
            };
            res.push(article);
        }
        res
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_articles() -> Vec<NewsArticle> {
    // let url = "https://www.gcores.com/gapi/v1/articles?page[limit]=10&sort=-published-at&include=category,user&filter[is-news]=true&fields[articles]=title,desc,is-published,thumb,app-cover,cover,comments-count,likes-count,bookmarks-count,is-verified,published-at,option-is-official,option-is-focus-showcase,duration,category,user";

    // let req = reqwasm::http::Request::get(&url);
    // let resp = req.send().await.unwrap();
    // let response: Vec<NewsArticle> = resp.json().await.unwrap();

    // let req = ureq::get(&url);
    // let response: Vec<NewsArticle> = req.call().unwrap().into_json().unwrap();

    let response = vec![
        NewsArticle::default(),
        NewsArticle::default(),
        NewsArticle::default(),
        NewsArticle::default(),
        NewsArticle::default(),
    ];
    response
}
