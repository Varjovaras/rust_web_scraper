use reqwest::blocking;

fn main() {
    let response = blocking::get("https://scrapeme.live/shop/");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = scraper::Selector::parse("listitem").unwrap();
    let html_products = document.select(&html_product_selector);
    println!("{}",);
}
