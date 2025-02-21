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
