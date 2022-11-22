mod imp;

use gio::SimpleAction;
use glib::{clone, Continue, MainContext, Object, PRIORITY_DEFAULT};
use gtk::traits::{TextBufferExt, TextViewExt, WidgetExt};
use gtk::{
    gio, glib, prelude::ActionMapExt, subclass::prelude::ObjectSubclassIsExt, Application,
    TextBuffer,
};
use rusqlite::Connection;
use std::thread;

use crate::avito_parser;
use crate::DB;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)])
    }
    fn setup_actions(&self) {
        let label = self.imp().date_label.get();
        let update_btn = self.imp().update_btn.get();
        let logs = self.imp().logs.get();
        let spinner = self.imp().spinner.get();

        let path = "D:/DB/auctions.db";
        let db = DB {
            connection: {
                match Connection::open(path) {
                    Ok(connection) => connection,
                    Err(e) => panic!("Ошибка с подключением к БД: {}", e),
                }
            },
        };

        match db.last_update_date(&String::from("artbook")) {
            Ok(d) => label.set_text(&d),
            Err(e) => panic!("Ошибка в запросе к БД: {}", e),
        }

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let (ready_sender, ready_receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let buff = TextBuffer::builder().build();
        logs.set_buffer(Some(&buff));

        let action_date = SimpleAction::new("date", None);
        action_date.connect_activate(
            clone!(@weak label, @weak logs, @weak spinner => move |_, _| {
                let sender = sender.clone();
                let ready_sender = ready_sender.clone();
                spinner.start();

                thread::spawn(move || {
                    // Deactivate the button until the operation is done
                    sender.send(false).expect("Could not send through channel");

                    let mut storage = avito_parser::Storage {
                        lots: Vec::new(),
                        books: Vec::new(),
                    };

                    let client = reqwest::Client::new();
                    let path = "D:/DB/auctions.db";
                    let db = DB {
                        connection: {
                            match Connection::open(path) {
                                Ok(connection) => connection,
                                Err(e) => panic!("Ошибка с подключением к БД: {}", e),
                            }
                        },
                    };

                    avito_parser::parsing_pages(
                        &avito_parser::count_number_of_pages_to_parse(&db, &client),
                        &client,
                        &mut storage,
                        &ready_sender
                    );

                    match db.set_current_date(&String::from("artbook")) {
                        Ok(_) => (),
                        Err(e) => panic!("Ошибка в запросе к БД: {}", e),
                    }

                    match db.select_books() {
                        Ok(books) => storage.books = books,
                        Err(e) => panic!("Ошибка в запросе к БД: {}", e),
                    }

                    avito_parser::ratio_lots_with_books(&mut storage.lots, &mut storage.books);

                    match db.update_lots(&storage.lots) {
                        Ok(_) => (),
                        Err(e) => panic!("Ошибка с добавлением лотов в БД: {}", e),
                    }

                    for lot in storage.lots {
                        ready_sender.send(format!("Title: {}\n", lot.title)).unwrap();
                    }

                    // Activate the button again
                    sender.send(true).expect("Could not send through channel");
                });
            }),
        );

        ready_receiver.attach(
            None,
            clone!(@weak buff => @default-return Continue(false), move |msg| {
                buff.insert_at_cursor(&msg);
                Continue(true)
            }),
        );

        // The main loop executes the closure as soon as it receives the message
        receiver.attach(
            None,
            clone!(@weak update_btn, @weak spinner => @default-return Continue(false),
                        move |enable_button| {
                            update_btn.set_sensitive(enable_button);
                            if enable_button == true {
                                spinner.stop();
                            }
                            Continue(true)
                        }
            ),
        );

        self.add_action(&action_date);
    }
}
