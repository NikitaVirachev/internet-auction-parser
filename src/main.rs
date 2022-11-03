use rusqlite::Connection;
use scraper::{Html, Selector};

mod book;
pub use book::Book;

mod lot;
pub use lot::Lot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        String::from("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA");
    let mut lots = Vec::new();
    match page_parsing(url) {
        Ok(n) => lots = n,
        Err(e) => println!("Ошибка с запросом: {}", e),
    }
    for lot in &lots {
        println!("{:?}", lot.title)
    }

    let path = "D:/DB/auctions.db";
    let connection = Connection::open(path)?;
    let mut stmt = connection.prepare("SELECT ISBN, Name FROM Book")?;
    let book_iter = stmt.query_map([], |row| {
        Ok(Book {
            isbn: row.get(0)?,
            title: row.get(1)?,
            count: 0,
        })
    })?;
    let mut books: Vec<Book> = Vec::new();
    for book in book_iter {
        books.push(book.unwrap());
    }

    ratio_lots_with_books(&mut lots, &mut books);

    for book in books {
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
                if book.title.contains(word) {
                    lot.count += 1;
                }
            }
        });

        let max = lots
            .iter()
            .reduce(|accum, item| {
                if accum.count >= item.count {
                    accum
                } else {
                    item
                }
            })
            .unwrap()
            .count;

        if max > 0 {
            book.count += 1
        }

        lots.iter_mut().for_each(|lot| lot.count = 0);
    }
}
