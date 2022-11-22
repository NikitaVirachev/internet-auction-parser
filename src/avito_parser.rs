use chrono::{NaiveDate, UTC};
use gtk::glib::Sender;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::Book;
use crate::Lot;
use crate::DB;

pub struct Storage {
    pub lots: Vec<Lot>,
    pub books: Vec<Book>,
}

pub fn parsing_pages(
    number_of_pages: &i64,
    client: &Client,
    storage: &mut Storage,
    tx: &Sender<String>,
) {
    for i in 1..number_of_pages + 1 {
        let url = format!("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&p={}&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA", i);
        match page_parsing(&url, &client) {
            Ok(lots) => {
                tx.send(format!("Запрос №{} выполнен\n", i))
                    .unwrap_or_else(|error| {
                        panic!("Receiving end of a channel is disconnected: {:?}", error.0);
                    });
                storage.lots.extend(lots);
            }
            Err(e) => {
                panic!("Ошибка с запросом: {}", e);
                // page_parsing(&url);
            }
        }
    }
}

#[tokio::main]
async fn page_parsing(
    url: &String,
    client: &Client,
) -> Result<Vec<Lot>, Box<dyn std::error::Error>> {
    let mut lots: Vec<Lot> = Vec::new();
    let resp = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&resp);

    let selector_lot = Selector::parse("div.iva-item-content-rejJg").unwrap();
    for element in document.select(&selector_lot) {
        let selector_title = Selector::parse("div.iva-item-titleStep-pdebR h3").unwrap();
        let selector_anchor = Selector::parse("a.link-link-MbQDP").unwrap();
        let title = element.select(&selector_title).next().unwrap().inner_html();
        let anchor = element
            .select(&selector_anchor)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
        let mut url = "https://www.avito.ru".to_string();
        url += anchor;
        let id = element
            .parent()
            .unwrap()
            .value()
            .as_element()
            .unwrap()
            .attr("data-item-id")
            .unwrap();
        let selector_price = Selector::parse("span.price-price-JP7qe meta:nth-child(2)").unwrap();
        let price = element
            .select(&selector_price)
            .next()
            .unwrap()
            .value()
            .attr("content")
            .unwrap();
        let selector_img = Selector::parse("li").unwrap();
        let data_marker = element
            .select(&selector_img)
            .next()
            .expect(&element.html())
            .value()
            .attr("data-marker")
            .unwrap();
        let first_i = data_marker.find("image-").unwrap() + 6;
        let img_url = &data_marker[first_i..];
        let img = client.get(img_url).send().await?.bytes().await?;
        let image =
            image::load_from_memory_with_format(&img.to_vec(), image::ImageFormat::Jpeg).unwrap();

        lots.push(Lot {
            id: id.to_string(),
            title,
            price: price.to_string(),
            url,
            preview: image.as_bytes().to_vec(),
            isbn: "".to_string(),
            count: 0,
        });
    }
    return Ok(lots);
}

#[tokio::main]
async fn get_number_of_pages(
    url: &String,
    client: &Client,
) -> Result<i64, Box<dyn std::error::Error>> {
    let resp = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&resp);

    let selector_pages = Selector::parse("span.pagination-item-JJq_j").unwrap();
    let mut pages: Vec<String> = Vec::new();
    for element in document.select(&selector_pages) {
        pages.push(element.inner_html());
    }
    let number_of_pages = pages[pages.len() - 2].parse().unwrap();
    return Ok(number_of_pages);
}

pub fn count_number_of_pages_to_parse(db: &DB, client: &Client) -> i64 {
    let last_update;
    match db.last_update_date(&String::from("artbook")) {
        Ok(date) => last_update = date,
        Err(e) => panic!("Ошибка в запросе к БД: {}", e),
    }

    let current_date = UTC::now().naive_utc();
    let last_update_date = NaiveDate::parse_from_str(&last_update, "%Y-%m-%d")
        .unwrap()
        .and_hms(0, 0, 0);
    let days_passed_count = (current_date - last_update_date).num_days();

    let number_of_pages;
    let url =
      String::from("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA");
    match get_number_of_pages(&url, &client) {
        Ok(number) => number_of_pages = number,
        Err(e) => panic!("Ошибка с запросом: {}", e),
    }

    if days_passed_count >= 1 && days_passed_count < number_of_pages {
        return days_passed_count;
    } else if days_passed_count == 0 {
        return 1;
    } else {
        return number_of_pages;
    }
}

pub fn ratio_lots_with_books(lots: &mut Vec<Lot>, books: &mut Vec<Book>) {
    for book in books {
        lots.into_iter().for_each(|lot| {
            for word in &lot.get_keywords() {
                if book.title.to_lowercase().contains(word) {
                    lot.count += 1;
                }
            }
            // if book.title == "Искусство Battlefield 1" {
            //     println!(
            //         "words: {:?}, lot title: {}, count: {}",
            //         &lot.get_keywords(),
            //         lot.title,
            //         lot.count
            //     );
            // }
        });

        let sum = lots.iter().map(|lot| lot.count).sum::<i32>() as f32;
        let count = lots.iter().filter(|lot| lot.count > 0).count();

        let mean = match count {
            positive if positive > 0 => sum / count as f32,
            _ => 0.0,
        };

        // if book.title == "Искусство Battlefield 1" {
        //     println!("mean: {}", mean);
        // }

        if mean != 0.0 {
            lots.iter_mut()
                .filter(|lot| lot.count as f32 >= mean)
                .for_each(|lot| lot.isbn = book.isbn.clone());
        }

        lots.iter_mut().for_each(|lot| lot.count = 0);
    }
}
