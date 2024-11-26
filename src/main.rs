use std::{
    env, io::{self, Write}, path::PathBuf,
};
use lopdf::Document;
use recon::inverted_index::InvertedIndex;
use recon::arena::Arena;

fn pdf2text<P: AsRef<std::path::Path>>(pdf_path: P) -> Result<String, String> {
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

fn get_binary_dir_path() ->  PathBuf {
    env::current_exe()
        .expect("failed to get current exe path")
        .parent()
        .expect("failed to get binary directory")
        .to_path_buf()
}

fn display_results(results: Vec<(String, f64)>) {
    let max_width = results.iter().map(|(name, _)| name.len()).max().unwrap_or(0) + 4;

    for (file_name, score) in results {
        println!("{:<max_width$} : {:.6}", file_name, score, max_width = max_width);
    }
}

fn print_manual() {
    println!("\
`load_ii` or `l`:
loads inverted index (a json file) from the directory where the binary lies
usage: `load_ii <filename>`

`save_ii` or `s`:
saves inverted index (a json file) to the directory where the binary lies
usage: `save_ii <filename>`

`build_ii` or `b`:
builds inverted index from a corpus, which has to be the current working directory
usage: `build_ii`

`query` or `?`:
keywords to search in the inverted index (currently supports only single query)
usage: `query <query>`

`exit` or `e`:
exits the program

`help` or `h`:
prints out a manual")
}

fn command_loop(
    arena: &mut Arena, 
    _inverted_index: &mut InvertedIndex,
) -> io::Result<()>{
    loop {
        print!("recon>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        let input = input.trim();
        if input.is_empty() { continue };
        let mut tokens = input.split_whitespace();
        let command = unsafe { tokens.next().unwrap_unchecked() };

        match command {
            "load_ii" | "l" => {
                if let Some(filename)  = tokens.next() {
                    let binary_dir = get_binary_dir_path();
                    let load_path = binary_dir.join(filename);
                    _inverted_index.load(load_path)?;
                } else {
                    println!("missing filename")
                }
            },
            "build_ii" | "b" => {
                if let Some(token) = tokens.next() {
                    println!("unexpected token {token}: build_ii takes 0 arguments");
                    continue;
                }
                arena.clear();
                let current_dir = env::current_dir().expect("failed to get current working directory");
                _inverted_index.build(current_dir, arena)?;
            },
            "save_ii" | "s"=> {
                if let Some(filename) = tokens.next() {
                    let binary_dir = get_binary_dir_path();
                    let save_path = binary_dir.join(filename);
                    _inverted_index.save(save_path)?;
                }
            },
            "prune_ii" | "p" => {
                if let Some(threshold) = tokens.next() {
                    let threshold_float = match threshold.parse::<f32>() {
                        Ok(value) => value,
                        Err(e) => {
                            println!("{e}");
                            continue; 
                        }
                    };

                    if threshold_float < 0.0 || threshold_float > 1.0 {
                        println!("threshold must be between 0 and 1");
                        continue;
                    }

                    _inverted_index.prune(threshold_float);
                } else {
                    println!("missing threshold");
                }
            }
            "query" | "?" => {
                if _inverted_index.ii.is_empty() {
                    println!("load or build an inverted index first");
                    continue;
                }
                let mut queries = Vec::new();
                while let Some(query) = tokens.next() {
                    let query = query.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                    queries.push(query);
                }
                if queries.is_empty() {
                    println!("missing query/ies");
                    continue;
                }
                let scores = _inverted_index.recon(queries);
                if scores.is_empty() {
                    println!("no results found");
                    continue;
                }
                display_results(scores);
            },
            "pdf2text" => {
                if let Some(filename) = tokens.next() {
                    let extracted_text = pdf2text(filename).unwrap();
                    println!("{extracted_text}");
                }
            }
            "help" | "h" => print_manual(),
            "exit" | "e" => std::process::exit(0),
            _ => println!("kys"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut arena = Arena::new();
    let mut _inverted_index = InvertedIndex::new();
    command_loop(&mut arena, &mut _inverted_index)?;
    Ok(())
}
