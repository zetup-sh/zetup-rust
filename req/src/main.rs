#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json = reqwest::Client::new()
        .post("https://api.github.com/authorizations")
        .header("user-agent", "hallo")
        .json(&serde_json::json!({
            "note": "zetup",
            "scopes": [
                "repo",
                "admin:org",
                "admin:public_key",
                "admin:repo_hook",
                "gist",
                "notifications",
                "user",
                "delete_repo",
                "write:discussion",
                "admin:gpg_key"
            ],
        }))
        .basic_auth("zwhitchcox", Some("Tiger#508"))
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", echo_json);
    Ok(())
}