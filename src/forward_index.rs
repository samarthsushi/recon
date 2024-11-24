use crate::arena::Arena;
use crate::crawler::Crawler;
use crate::computations::{compute_idf, compute_tf, compute_tf_idf};
use std::collections::HashMap;
use std::io::Read;

type TermFreqMap<'a> = HashMap<std::borrow::Cow<'a, str>, usize>;
type DocumentMap<'a> = HashMap<String, TermFreqMap<'a>>;

#[derive(Debug)]
pub struct ForwardIndex<'a> {
    pub fi: DocumentMap<'a>
}

impl<'a> ForwardIndex<'a> {
    pub fn new() -> Self {
        Self { fi: DocumentMap::new() }
    }

    pub fn build(&mut self, current_dir: std::path::PathBuf, arena: &mut Arena) -> std::io::Result<()> {
        for entry in std::fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();
    
            if path.extension().map_or(false, |ext| ext == "txt") {
                let mut file_content = String::new();
                let mut file = std::fs::File::open(&path)?;
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
    
                self.fi.insert(
                    path.file_name().unwrap().to_string_lossy().to_string(),
                    freq,
                );
            }
        }
        Ok(())
    }

    pub fn recon(&self, query: String) -> Vec<(String, f64)> {
        let mut scores = Vec::new();
        let idf = compute_idf(&self.fi, &query);

        for (doc_name, term_freq_map) in &self.fi {
            let tf = compute_tf(term_freq_map, &query); 
            let tf_idf = compute_tf_idf(tf, idf); 
            if tf_idf > 0.0 {
                scores.push((doc_name.clone(), tf_idf));
            }
        }

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores
    }

    pub fn load(&mut self, file_path: std::path::PathBuf) -> std::io::Result<()>{
        let content = std::fs::read_to_string(file_path)?;
        let deserialized: DocumentMap<'static> = serde_json::from_str(&content)
            .expect("failed to deserialize the inverted index");
        self.fi = deserialized;
        Ok(())
    }

    pub fn save(&self, file_path: std::path::PathBuf) -> std::io::Result<()>{
        let serialized = serde_json::to_string(&self.fi).expect("failed to serialize the inverted index");
        std::fs::write(file_path, serialized)?;
        Ok(())
    }
}