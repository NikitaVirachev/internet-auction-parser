use rusqlite::Connection;
use scraper::{Html, Selector};

mod book;
pub use book::Book;

mod lot;
pub use lot::Lot;

mod db;
pub use db::DB;

struct Storage {
    lots: Vec<Lot>,
    books: Vec<Book>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        String::from("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA");
    let mut storage = Storage {
        lots: Vec::new(),
        books: Vec::new(),
    };
    match page_parsing(url) {
        Ok(n) => storage.lots = n,
        Err(e) => panic!("Ошибка с запросом: {}", e),
    }
    // for lot in &storage.lots {
    //     println!("{:?}", lot.title)
    // }

    let path = "D:/DB/auctions.db";
    let db = DB {
        connection: {
            match Connection::open(path) {
                Ok(connection) => connection,
                Err(e) => panic!("Ошибка с подключением к БД: {}", e),
            }
        },
    };
    match db.select_books() {
        Ok(books) => storage.books = books,
        Err(e) => panic!("Ошибка в запросе к БД: {}", e),
    }

    ratio_lots_with_books(&mut storage.lots, &mut storage.books);

    for book in storage.books {
        println!("{:?}", book)
    }

    Ok(())
}

#[tokio::main]
async fn page_parsing(url: String) -> Result<Vec<Lot>, Box<dyn std::error::Error>> {
    let mut lots: Vec<Lot> = Vec::new();
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("div.iva-item-titleStep-pdebR h3").unwrap();
    for element in document.select(&selector) {
        lots.push(Lot {
            title: element.inner_html(),
            count: 0,
        });
    }
    return Ok(lots);
}

fn ratio_lots_with_books(lots: &mut Vec<Lot>, books: &mut Vec<Book>) {
    for book in books {
        lots.into_iter().for_each(|lot| {
            for word in &lot.get_keywords() {
                if book.title.to_lowercase().contains(word) {
                    lot.count += 1;
                }
            }
            if book.title == "Искусство Battlefield 1" {
                println!(
                    "words: {:?}, lot title: {}, count: {}",
                    &lot.get_keywords(),
                    lot.title,
                    lot.count
                );
            }
        });

        let sum = lots.iter().map(|lot| lot.count).sum::<i32>() as f32;
        let count = lots.iter().filter(|lot| lot.count > 0).count();

        let mean = match count {
            positive if positive > 0 => sum / count as f32,
            _ => 0.0,
        };

        if book.title == "Искусство Battlefield 1" {
            println!("mean: {}", mean);
        }

        if mean != 0.0 {
            lots.iter()
                .filter(|lot| lot.count as f32 >= mean)
                .for_each(|_| book.count += 1);
        }

        lots.iter_mut().for_each(|lot| lot.count = 0);
    }
}
