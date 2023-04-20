use tracing::error;

// get html content use reqwest
pub async fn get_html_content(url: &str) -> Option<String> {
    let client = reqwest::Client::new();
    // disable the javascript for the page

    let res = client
        .get(url)
        .header("X-JAVASCRIPT-ENABLED", "false")
        .send()
        .await;

    match res {
        Ok(res) => {
            let text = res.text().await;
            match text {
                Ok(text) => Some(text),
                Err(e) => {
                    error!("error: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            error!("error: {}", e);
            None
        }
    }
}
