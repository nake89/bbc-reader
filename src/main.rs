use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BBCJSON {
    pub status: String,
    pub total_results: i64,
    pub articles: Vec<Article>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub source: Source,
    pub author: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub url_to_image: String,
    pub published_at: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: String,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=2a571f1425d44f8ab56de12e0e0e9b1b")
//        .get("https://httpbin.org/ip")
        .send()
        .await?;

    // Parse the response body as Json in this case
    let bbc_json = res
        .json::<BBCJSON>()
        .await?;

    println!("Hello {:?}", bbc_json.status);
    Ok(())
}

