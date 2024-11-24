use std::{
    borrow::Cow, collections::HashMap, env, fs, io::{self, Read, Write}, path::PathBuf
};
use dotenv::dotenv;
use recon::computations::{compute_tf, compute_idf, compute_tf_idf};
use recon::crawler::Crawler;
use recon::arena::Arena;

type TermFreqMap<'a> = HashMap<Cow<'a, str>, usize>;
type DocumentMap<'a> = HashMap<String, TermFreqMap<'a>>;

fn get_binary_dir_env_path() -> PathBuf {
    let binary_dir = env::current_exe().expect("failed to get current exe path");
    let binary_dir = binary_dir.parent().expect("failed to get binary directory");
    binary_dir.join(".env")
}

fn get_binary_dir_path() ->  PathBuf {
    env::current_exe()
        .expect("failed to get current exe path")
        .parent()
        .expect("failed to get binary directory")
        .to_path_buf()
}

fn save_inverted_index(file_path: &str, inverted_index: &DocumentMap) -> io::Result<()> {
    let serialized = serde_json::to_string(inverted_index)
        .expect("failed to serialize the inverted index");
    fs::write(file_path, serialized)?;
    Ok(())
}

fn load_inverted_index(file_path: &str) -> io::Result<DocumentMap<'static>> {
    let content = fs::read_to_string(file_path)?;
    let deserialized: DocumentMap<'static> = serde_json::from_str(&content)
        .expect("failed to deserialize the inverted index");
    Ok(deserialized)
}

fn main() -> io::Result<()> {
    let env_path = get_binary_dir_env_path();
    dotenv().unwrap_or_else(|_| {
        let default_paths = "II_SAVE_PATH=./ii.json\nII_LOAD_PATH=./ii.json\n";
        fs::write(&env_path, default_paths).expect("failed to write default .env");
        env_path.clone()
    });
    
    let mut inverted_index: DocumentMap = HashMap::new();
    let mut _ii_save_path = unsafe { env::var("II_SAVE_PATH").unwrap_unchecked() };
    let mut _ii_load_path = unsafe { env::var("II_LOAD_PATH").unwrap_unchecked() };
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
            "set_save_path" => {
                if let Some(path) = tokens.next() {
                    _ii_save_path = path.to_string();
                    println!("II_SAVE_PATH changed to: {}", _ii_save_path);
                } else {
                    println!("missing path");
                }
            }
            "set_load_path" => {
                if let Some(path) = tokens.next() {
                    _ii_load_path = path.to_string();
                    println!("II_LOAD_PATH changed to: {}", _ii_load_path);
                } else {
                    println!("missing path");
                }
            }
            "build" => {
                let binary_dir = get_binary_dir_path();
                let load_path = binary_dir.join(&_ii_load_path);
                match load_inverted_index(load_path.to_str().unwrap()) {
                    Ok(ii) => {
                        inverted_index = ii;
                        println!("loaded {}", load_path);
                    }
                    Err(e) => {
                        println!("failed to load inverted index: {}", e);
                    }
                }
            },
            "query" => {},
            "save_index" => {},
            "load_index" => {},
            "exit" => std::process::exit(1),
            _ => println!("kys"),
        }
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} \"<query>\"", args[0]);
        return Ok(());
    }

    let mut arena = Arena::new();
    let query = args[1].to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>(); // suite of pre processing functions for now
    let current_dir = r"data\blonde_plaintext";

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "txt") {
            let mut file_content = String::new();
            let mut file = fs::File::open(&path)?;
            file.read_to_string(&mut file_content)?;

            // reason for unsafe: String is moved into arena so file_content is owned by arena, which has a static lifetime
            let file_content: &'static str = unsafe {
                let content = file_content.to_lowercase();
                let reference = arena.alloc(content);
                std::mem::transmute::<&str, &'static str>(reference)
            };

            let mut lexer = Crawler::new(file_content);
            let mut freq: TermFreqMap = HashMap::new();

            while let Some(word) = lexer.next_token() {
                *freq.entry(word).or_insert(0) += 1;
            }

            inverted_index.insert(
                path.file_name().unwrap().to_string_lossy().to_string(),
                freq,
            );
        }
    }
    // println!("{:#?}", inverted_index); // uncomment to debug what the lexer outputs

    let mut scores = Vec::new();
    let idf = compute_idf(&inverted_index, &query);
    println!("idf of {query}: {:.6}\n", idf);

    for (doc_name, term_freq_map) in &inverted_index {
        let tf = compute_tf(term_freq_map, &query); 
        let tf_idf = compute_tf_idf(tf, idf); 
        // println!("{doc_name}:\ntf:{:.6}\ttf*idf: {:.6}", tf ,tf_idf);
        if tf_idf > 0.0 {
            scores.push((doc_name.clone(), tf_idf));
        }
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("reconned:");
    for (doc_name, score) in scores {
        println!("{}: {:.6}", doc_name, score);
    }

    save_inverted_index("inverted_index.json", &inverted_index)?;

    Ok(())
}
