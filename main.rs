use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://www.google.com";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response body:\n{}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}