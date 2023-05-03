use reqwest;
use scraper;

fn main() {
    get_imdb()
}

fn get_imdb() {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    let list: Vec<String> = titles
        .zip(1..101)
        .map(|(title, number)| format!("{}: {}", number, title))
        .collect();

    println!("{:?}", list);
}
