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

#[test]
fn test_light_msg_decryption() {
    use serde_json;
    use light_writer_rs::{light_msg_encryption, light_msg_decryption};
    
    let book_metadata = book_types::BookMetadata {
        title: "Rust Programming".to_string(),
        authors: "Ferris the Crab".to_string(),
        production: "Open Source".to_string(),
        subtitle: "A Safe Systems Language".to_string(),
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
    let encryption_key = "ciao1234567890123456789012345678";
    let encrypted_data = light_msg_encryption(encryption_key, &json_metadata).expect("Encryption failed");

    eprintln!("✨ Debug: Encrypted data length: {:?}", encrypted_data.len());

    let decrypted_metadata = light_msg_decryption(encryption_key, encrypted_data).expect("Decryption failed");

    eprintln!("✨ Debug: Decrypted metadata: {:?}", decrypted_metadata);
    
    assert_eq!(decrypted_metadata.title, book_metadata.title, "Decryption failed: Title mismatch");
    assert_eq!(decrypted_metadata.authors, book_metadata.authors, "Decryption failed: Authors mismatch");
    assert_eq!(decrypted_metadata.page_count, book_metadata.page_count, "Decryption failed: Page count mismatch");
    assert_eq!(decrypted_metadata.content.len(), book_metadata.content.len(), "Decryption failed: Content mismatch");
}
