//use chrono::{Datelike, UTC};
use rusqlite::Connection;

use crate::Book;

pub struct DB {
    pub connection: Connection,
}

impl DB {
    // pub fn open_connection(&mut self, path: String) {
    //     match Connection::open(path) {
    //         Ok(connection) => self.connection = connection,
    //         Err(e) => println!("Ошибка с подключением к БД: {}", e),
    //     };
    // }
    pub fn select_books(&self) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let mut books: Vec<Book> = Vec::new();
        let mut stmt = self.connection.prepare("SELECT ISBN, Name FROM Book")?;
        let book_iter = stmt.query_map([], |row| {
            Ok(Book {
                isbn: row.get(0)?,
                title: row.get(1)?,
                count: 0,
            })
        })?;
        for book in book_iter {
            books.push(book.unwrap());
        }
        Ok(books)
    }
    pub fn set_current_date(&self, category: &String) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute(
            "UPDATE State SET Last_update = date('now') WHERE Сategory = :category",
            &[(":category", category)],
        )?;
        Ok(())
    }
    pub fn last_update_date(
        &self,
        category: &String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut date = String::from("");
        let mut stmt = self
            .connection
            .prepare("SELECT Last_update FROM State WHERE Сategory = :category")?;
        let mut rows = stmt.query(rusqlite::named_params! { ":category": category })?;
        while let Some(row) = rows.next()? {
            date = row.get(0).expect("get row failed");
        }
        Ok(date)
    }
}
