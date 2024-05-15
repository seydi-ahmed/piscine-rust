pub mod library {
    pub mod writers {
        use super::books::Book;
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>
        }
    }
    
    pub mod books {
        pub struct Book {
            pub title: String,
            pub year: u64
        }
    }
    
}

use crate::library::writers::Writer;
pub fn order_books(writer: &mut Writer) {
    writer.books.sort_by_key(|a| a.title.clone());
}


// ***************************************************************************


pub use library::books::Book;
fn main() {
    let mut writer_a = Writer {
        first_name: "William".to_string(),
        last_name: "Shakespeare".to_string(),
        books: vec![
            Book {
                title: "Hamlet".to_string(),
                year: 1600,
            },
            Book {
                title: "Othelo".to_string(),
                year: 1603,
            },
            Book {
                title: "Romeo and Juliet".to_string(),
                year: 1593,
            },
            Book {
                title: "MacBeth".to_string(),
                year: 1605,
            },
        ],
    };

    println!("Before ordering");
    for b in &writer_a.books {
        println!("{:?}", b.title);
    }

    order_books(&mut writer_a);

    println!("\nAfter ordering");
    for b in writer_a.books {
        println!("{:?}", b.title);
    }
}