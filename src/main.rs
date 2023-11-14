use reqwest;
use scraper::{Html, Selector};
use regex::Regex;
use std::fs::File;
use std::io::{Write, BufWriter};

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    // '?' = shorthand for Result enum
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    Ok(body) // no trailing semi-colons bc Rust is an expression oriented language
}

async fn scrape(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html = fetch_html(url).await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("body").unwrap();

    let re = Regex::new(r"\b(f)(r)(e)(e)\b|\b(p)(a)(l)(e)(s)(t)(i)(n)(e)\b")?; //this finds pattern matches to scrape
    let mut results = Vec::new();

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join("");
        for cap in re.captures_iter(&text) {
            results.push(cap[0].to_string());
        }
    }

    let mut file = BufWriter::new(File::create("scraped.txt")?);
    for word in results {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    let url = "https://en.wikipedia.org/wiki/State_of_Palestine"; //this is the target URL
    // match control flow to run the show
    match scrape(url).await {
        Ok(_) => println!("Success!!"),
        Err(e) => println!("Error , {}", e),
    }
}

