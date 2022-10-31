use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        //String::from("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA");
        String::from("https://ru.wikipedia.org/wiki/%D0%A1%D0%B0%D0%B9%D1%82-%D0%B2%D0%B8%D0%B7%D0%B8%D1%82%D0%BA%D0%B0");
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse(".firstHeading").unwrap();
    for element in document.select(&selector) {
        println!("{:#?}", element.value());
    }
    Ok(())
}
