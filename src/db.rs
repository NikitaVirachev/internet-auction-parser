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
}
