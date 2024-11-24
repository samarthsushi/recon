use std::{
    env, fs, io::{self, Write}, path::PathBuf,
};
use dotenv::from_path;
use recon::inverted_index::InvertedIndex;
use recon::arena::Arena;

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

fn command_loop(
    arena: &mut Arena, 
    _inverted_index: &mut InvertedIndex, 
    mut _ii_load_path: String, 
    mut _ii_save_path: String
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
            "load_ii" => {
                let binary_dir = get_binary_dir_path();
                let load_path = binary_dir.join(&_ii_load_path);
                _inverted_index.load(load_path)?;
            },
            "build_ii" => {
                arena.clear();
                let current_dir = env::current_dir().expect("failed to get current working directory");
                _inverted_index.build(current_dir, arena)?;
            }
            "query" => {
                if let Some(query) = tokens.next() {
                    if _inverted_index.ii.is_empty() {
                        println!("load or build an inverted index first");
                        continue;
                    }
                    let query = query.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                    let scores = _inverted_index.recon(query);
                    if scores.is_empty() {
                        println!("no results found");
                        continue;
                    }
                    for (doc_name, score) in scores { println!("{}: {:.6}", doc_name, score) };

                } else {
                    println!("missing query");
                }
            },
            "save_ii" => {
                let binary_dir = get_binary_dir_path();
                let save_path = binary_dir.join(&_ii_save_path);
                _inverted_index.save(save_path)?;
            }
            "exit" => std::process::exit(0),
            _ => println!("kys"),
        }
    }
}

fn main() -> io::Result<()> {
    let env_path = get_binary_dir_env_path();
    from_path(&env_path).unwrap_or_else(|_| {
        let default_paths = "II_SAVE_PATH=./ii.json\nII_LOAD_PATH=./ii.json\n";
        fs::write(&env_path, default_paths).expect("failed to write default .env");
    });
    let mut arena = Arena::new();
    let mut _inverted_index = InvertedIndex::new();
    let mut _ii_save_path = env::var("II_SAVE_PATH").unwrap();
    let mut _ii_load_path = env::var("II_LOAD_PATH").unwrap();
    command_loop(&mut arena, &mut _inverted_index, _ii_load_path, _ii_save_path)?;
    Ok(())
}
