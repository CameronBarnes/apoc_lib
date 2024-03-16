pub mod size;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use anyhow::Result;

/// Makes a HTTP(s) web request to get the content of a page
///
/// # Errors
///
/// Will return 'Err' if it fails to get the page or the page content
pub fn get_page_from_path(path: &str) -> Result<String> {
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build()
            .unwrap()
    });
    Ok(CLIENT.get(path).send()?.text()?)
}
