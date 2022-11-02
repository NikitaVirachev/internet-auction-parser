use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        String::from("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA");
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("div.iva-item-titleStep-pdebR h3").unwrap();
    let mut book_titles: Vec<Vec<String>> = Vec::new();
    for element in document.select(&selector) {
        book_titles.push(
            element
                .inner_html()
                .split(' ')
                .map(String::from)
                .filter(|x| !x.contains("Артбук"))
                .collect(),
        );
    }
    for tittle in book_titles {
        println!("{:?}", tittle)
    }
    Ok(())
}
