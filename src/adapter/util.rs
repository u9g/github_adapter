use std::sync::OnceLock;

use octorust::http_cache::HttpCache;
use octorust::{auth::Credentials, Client};

static INSTANCE: OnceLock<Client> = OnceLock::new();

pub fn client() -> &'static Client {
    let http_cache = <dyn HttpCache>::in_home_dir();

    INSTANCE.get_or_init(|| {
        Client::custom(
            String::from("trustfall-github-adapter-u9g.dev"),
            Credentials::Token(
                std::env::var("GITHUB_TOKEN").expect("to get 'GITHUB_TOKEN' from env"),
            ),
            reqwest::Client::builder().build().unwrap().into(),
            http_cache,
        )
    })
}
