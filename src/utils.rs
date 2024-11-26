use lopdf::Document;

pub fn get_binary_dir_path() -> std::path::PathBuf {
    std::env::current_exe()
        .expect("failed to get current exe path")
        .parent()
        .expect("failed to get binary directory")
        .to_path_buf()
}

pub fn display_results(results: Vec<(String, f64)>) {
    let max_width = results.iter().map(|(name, _)| name.len()).max().unwrap_or(0) + 4;

    for (file_name, score) in results {
        println!("{:<max_width$} : {:.6}", file_name, score, max_width = max_width);
    }
}

pub fn pdf2text<P: AsRef<std::path::Path>>(pdf_path: P) -> Result<String, String> {
    let doc = Document::load(&pdf_path).map_err(|e| format!("Failed to load PDF: {}", e))?;
    let mut extracted_text = String::new();

    for (page_num, _page_obj) in doc.get_pages() {
        match doc.extract_text(&[page_num]) {
            Ok(text) => extracted_text.push_str(&text),
            Err(e) => eprintln!("Failed to extract text from page {}: {}", page_num, e),
        }
    }

    Ok(extracted_text)
}