use std::{
    env, io::{self, Write}, path::PathBuf,
};
use recon::inverted_index::InvertedIndex;
use recon::arena::Arena;

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
            "query" | "?" => {
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
                    display_results(scores);

                } else {
                    println!("missing query");
                }
            }
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
