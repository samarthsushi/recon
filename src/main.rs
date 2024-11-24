use std::{
    env, io::{self, Write}, path::PathBuf,
};
use recon::forward_index::ForwardIndex;
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

fn print_manual() {
    println!("\
`load_fi` or `l`:
loads forward index (a json file) from the directory where the binary lies
usage: `load_fi <filename>`

`save_fi` or `s`:
saves forward index (a json file) to the directory where the binary lies
usage: `save_fi <filename>`

`build_fi` or `b`:
builds forward index from a corpus, which has to be the current working directory
usage: `build_fi`

`query` or `?`:
keywords to search in the forward index (currently supports only single query)
usage: `query <query>`

`exit` or `e`:
exits the program

`help` or `h`:
prints out a manual")
}

fn command_loop(
    arena: &mut Arena, 
    _forward_index: &mut ForwardIndex,
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
            "load_fi" | "l" => {
                if let Some(filename)  = tokens.next() {
                    let binary_dir = get_binary_dir_path();
                    let load_path = binary_dir.join(filename);
                    _forward_index.load(load_path)?;
                } else {
                    println!("missing filename")
                }
            },
            "build_fi" | "b" => {
                arena.clear();
                let current_dir = env::current_dir().expect("failed to get current working directory");
                _forward_index.build(current_dir, arena)?;
            },
            "save_fi" | "s"=> {
                if let Some(filename) = tokens.next() {
                    let binary_dir = get_binary_dir_path();
                    let save_path = binary_dir.join(filename);
                    _forward_index.save(save_path)?;
                }
            },
            "query" | "?" => {
                if let Some(query) = tokens.next() {
                    if _forward_index.fi.is_empty() {
                        println!("load or build an forward index first");
                        continue;
                    }
                    let query = query.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                    let scores = _forward_index.recon(query);
                    if scores.is_empty() {
                        println!("no results found");
                        continue;
                    }
                    display_results(scores);

                } else {
                    println!("missing query");
                }
            },
            "help" | "h" => print_manual(),
            "exit" | "e" => std::process::exit(0),
            _ => println!("kys"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut arena = Arena::new();
    let mut _forward_index = ForwardIndex::new();
    command_loop(&mut arena, &mut _forward_index)?;
    Ok(())
}
