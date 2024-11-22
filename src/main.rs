use std::{
    collections::HashMap, fs, io::{self, Read}
};

type TermFreqMap = HashMap<String, usize>; 
type DocumentMap = HashMap<String, TermFreqMap>;

struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.chars.peek().map_or(false, |c| c.is_whitespace()) {
            self.chars.next();
        }
    }

    fn extract_word(&mut self) -> String {
        self.chars
            .by_ref()
            .take_while(|c| c.is_alphanumeric() || *c == '\'')
            .collect()
    }

    fn next_token(&mut self) -> Option<String> {
        self.skip_whitespace();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '\'' {
                let mut word = String::new();
                word.push(self.chars.next().unwrap()); 
                word.push_str(&self.extract_word());
                return Some(word);
            } else {
                self.chars.next();
            }
        }
        None
    }
}

fn compute_tf(doc: &HashMap<String, usize>, query: &str) -> f64 {
    let total_words: usize = doc.values().sum();
    let query_count = *doc.get(query).unwrap_or(&0); 
    query_count as f64 / total_words as f64 
}

fn compute_idf(documents: &HashMap<String, TermFreqMap>, query: &str) -> f64 {
    let doc_count = documents.len() as f64;
    let containing_docs = documents
        .values()
        .filter(|doc| doc.contains_key(query))
        .count() as f64;

    if containing_docs == 0.0 {
        0.0 
    } else {
        (doc_count / containing_docs as f64).ln() 
    }
}

#[inline]
fn compute_tf_idf(tf: f64, idf: f64) -> f64 {
    tf * idf
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <query>", args[0]);
        return Ok(());
    }

    let query = args[1].to_lowercase();

    let current_dir = r"data\blonde_plaintext";

    let mut documents: DocumentMap = HashMap::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "txt") {
            let mut file_content = String::new();
            let mut file = fs::File::open(&path)?;
            file.read_to_string(&mut file_content)?;

            let file_content = file_content.to_lowercase();

            let mut freq = HashMap::new();
            let mut lexer = Lexer::new(&file_content);

            while let Some(word) = lexer.next_token() {
                *freq.entry(word).or_insert(0) += 1;
            }
            documents.insert(
                path.file_name().unwrap().to_string_lossy().to_string(),
                freq,
            );
        }
    }
    
    // println!("{:#?}", documents); // uncomment to debug what the lexer outputs

    let mut scores = Vec::new();
    let idf = compute_idf(&documents, &query);
    println!("idf of {query}: {idf}\n");

    for (doc_name, term_freq_map) in &documents {
        let tf = compute_tf(term_freq_map, &query); 
        let tf_idf = compute_tf_idf(tf, idf); 
        println!("{doc_name}:\ntf:{tf}\ttf*idf: {tf_idf}");
        if tf_idf > 0.0 {
            scores.push((doc_name.clone(), tf_idf));
        }
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("reconned:");
    for (doc_name, score) in scores {
        println!("{}: {:.4}", doc_name, score);
    }

    Ok(())
}
