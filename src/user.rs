use std::future::Future;

use reqwest::header::USER_AGENT;

pub async fn get() -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "https://api.github.com/users/steveklabnik/repos";
    client.get(url).header(USER_AGENT, "reqwest").send().await
}
