use light_writer_rs::*;

#[test]
fn test_generate_isbn() {
    let isbn = generate_isbn("The Digital Archive", "4jNggaAqfahXvFHcz1QorcSaDKUaNDeY7SpYaHgXbDEU");
    eprintln!("📚 Debug: Generated ISBN for 'The Digital Archive' by 4jNggaAqfahXvFHcz1QorcSaDKUaNDeY7SpYaHgXbDEU: {}", isbn);
}


#[test]
fn test_populate_book_metadata() {
    let metadata = populate_book_metadata();
    eprintln!("✨ Debug: metadata: {:?}", metadata);
}

#[test]
fn test_light_msg_encryption() {
    use serde_json;
    
    let book_metadata = book_types::BookMetadata {
        title: "Rust Programming".to_string(),
        authors: "Ferris the Crab".to_string(),
        production: "Open Source".to_string(),
        subtitle: "A Safe Systems Language".to_string(),
        page_index: 1,
        isbn: "978-1593278281".to_string(),
        publisher: "Rustacean Press".to_string(),
        publication_date: "2025-01-01".to_string(),
        language: "English".to_string(),
        genres: vec!["Programming".to_string(), "Systems".to_string()],
        tags: vec!["Rust".to_string(), "Memory Safety".to_string()],
        edition: "First".to_string(),
        description: "A book about Rust programming language.".to_string(),
        table_of_contents: vec!["Introduction".to_string(), "Ownership".to_string()],
        cover_image_url: "https://example.com/rust_book.jpg".to_string(),
        page_count: 400,
        notes: "Some notes".to_string(),
        quotes: vec!["Rust is awesome!".to_string()],
        references: vec!["Rust Documentation".to_string()],
        modified_date: "2025-03-19".to_string(),
        categories: vec!["Technology".to_string()],
        illustrator: "John Doe".to_string(),
        editor: "Jane Doe".to_string(),
        translator: "N/A".to_string(),
        dedication: "To all Rustaceans".to_string(),
        acknowledgments: "Thanks to the Rust community".to_string(),
        introduction: "An introduction to Rust".to_string(),
        preface: "Preface text".to_string(),
        appendices: vec!["Appendix A".to_string()],
        index_terms: vec!["Ownership".to_string()],
        related_books: vec!["The Rust Book".to_string()],
        resources: vec!["https://doc.rust-lang.org/book/".to_string()],
        format: "Hardcover".to_string(),
        content: vec![book_types::Chapter {
            chapter: "Ownership".to_string(),
            pages: vec![book_types::Page {
                page_number: 1,
                content: "Rust prevents data races.".to_string(),
            }],
            notes: "Important concept".to_string(),
            quotes: vec!["Borrow checker is strict!".to_string()],
        }],
        total_chapters: 1,
    };
    let json_metadata = serde_json::to_string(&book_metadata).expect("Serialization failed");
    let metadata_result = light_msg_encryption("ciao1234567890123456789012345678", &json_metadata);

    eprintln!("✨ Debug: metadata: {:?}", metadata_result);
    
    assert!(metadata_result.is_ok(), "Encryption failed");
}
