use rusqlite::Connection;
use scraper::{Html, Selector};

#[derive(Debug)]
struct Book {
    isbn: String,
    title: String,
    count: i32,
}

struct Lot {
    title: Vec<String>,
    count: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        String::from("https://www.avito.ru/all/knigi_i_zhurnaly/knigi-ASgBAgICAUTOAuoK?cd=1&q=%D0%B0%D1%80%D1%82%D0%B1%D1%83%D0%BA");
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("div.iva-item-titleStep-pdebR h3").unwrap();
    let mut lots: Vec<Lot> = Vec::new();
    for element in document.select(&selector) {
        lots.push(Lot {
            title: element
                .inner_html()
                .split(' ')
                .map(String::from)
                .filter(|x| {
                    !x.to_lowercase().contains("артбук")
                        && !x.to_lowercase().contains("мир")
                        && !x.to_lowercase().contains("игр")
                        && !x.to_lowercase().contains("искусство")
                })
                .collect(),
            count: 0,
        });
    }
    for lot in &lots {
        println!("{:?}", lot.title)
    }

    let path = "D:/DB/auctions.db";
    let connection = Connection::open(path)?;
    let mut stmt = connection.prepare("SELECT ISBN, Name, Count FROM Book")?;
    let book_iter = stmt.query_map([], |row| {
        Ok(Book {
            isbn: row.get(0)?,
            title: row.get(1)?,
            count: row.get(2)?,
        })
    })?;
    let mut books: Vec<Book> = Vec::new();
    for book in book_iter {
        books.push(book.unwrap());
    }

    for book in &mut books {
        for mut lot in &mut lots {
            for word in &lot.title {
                if book.title.contains(word) {
                    lot.count += 1;
                }
            }
        }
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

    for book in books {
        println!("{:?}", book)
    }

    Ok(())
}
