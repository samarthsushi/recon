use crate::arena::Arena;
use std::collections::HashMap;
use std::io::Read;

type DocId = usize;
type DocFreqMap<'a> = HashMap<DocId, usize>;
type InvertedIndexMap<'a> = HashMap<std::borrow::Cow<'a, str>, DocFreqMap<'a>>;

#[derive(Debug)]
pub struct InvertedIndex<'a> {
    pub ii: InvertedIndexMap<'a>,
    doc_names: Vec<String>,
    doc_lengths: Vec<usize>
}

impl<'a> InvertedIndex<'a> {
    pub fn new() -> Self {
        Self { ii: InvertedIndexMap::new(), doc_names: Vec::new() , doc_lengths: Vec::new() }
    }

    pub fn build(&mut self, current_dir: std::path::PathBuf, arena: &mut Arena) -> std::io::Result<()> {
        for entry in std::fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();
    
            if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                let doc_name = path.file_name().unwrap().to_string_lossy().to_string();
                let doc_id = self.doc_names.len();
                self.doc_names.push(doc_name);
    
                let mut file_content = String::new();
                match ext {
                    "txt" => {
                        let mut file = std::fs::File::open(&path)?;
                        file.read_to_string(&mut file_content)?;
                    }
                    "pdf" => { file_content = crate::utils::pdf2string(&path).unwrap(); }
                    "html" => { file_content = crate::utils::html2string(&path).unwrap(); }
                    _ => continue,
                }

                let file_content: &'static str = unsafe {
                    let content = file_content.to_lowercase();
                    let reference = arena.alloc(content);
                    std::mem::transmute::<&str, &'static str>(reference)
                };

                let mut lexer = crate::crawler::Crawler::new(file_content);
                self.doc_lengths.push(lexer.len());
                while let Some(word) = lexer.next_token() {
                    self.ii
                        .entry(word)
                        .or_insert_with(HashMap::new)
                        .entry(doc_id)
                        .and_modify(|freq| *freq += 1)
                        .or_insert(1);
                }
            }
        }
        Ok(())
    }

    pub fn recon(&self, queries: Vec<String>) -> Vec<(String, f64)> {
        let mut scores: HashMap<usize, f64> = HashMap::new();

        for query in queries {
            if let Some(doc_freq_map) = self.ii.get(query.as_str()) {
                let idf = Self::compute_idf(self.doc_lengths.len(), doc_freq_map.len());
                for (&doc_id, &term_freq) in doc_freq_map {
                    let tf = Self::compute_tf(term_freq, self.doc_lengths[doc_id]);
                    let tf_idf = Self::compute_tf_idf(tf, idf);
                    println!("{doc_id}::= tf({term_freq},{})={tf}, idf({},{})={idf}", self.doc_lengths[doc_id], self.doc_lengths.len(), doc_freq_map.len());
                    scores
                        .entry(doc_id)
                        .and_modify(|score| *score += tf_idf)
                        .or_insert(tf_idf);
                }
            }
        }
        let mut sorted_scores: Vec<_> = scores.into_iter().collect();
        sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted_scores
            .into_iter()
            .map(|(doc_id, score)| (self.doc_names[doc_id].clone(), score))
            .collect()
    }

    pub fn load(&mut self, file_path: std::path::PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(file_path)?;
        let deserialized: (InvertedIndexMap<'static>, Vec<String>, Vec<usize>) =
            serde_json::from_str(&content).expect("failed to deserialize the inverted index");
        self.ii = deserialized.0;
        self.doc_names = deserialized.1;
        self.doc_lengths = deserialized.2;
        Ok(())
    }

    pub fn save(&self, file_path: std::path::PathBuf) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&(&self.ii, &self.doc_names, &self.doc_lengths))
            .expect("failed to serialize the inverted index");
        std::fs::write(file_path, serialized)?;
        Ok(())
    }

    pub fn prune(&mut self, threshold: f32) {
        let threshold_num = (self.doc_names.len() as f32 * threshold).round() as usize;
        self.ii.retain(|_, doc_freq_map| doc_freq_map.len() < threshold_num);
    }

    #[inline]
    pub fn compute_tf(query_count: usize, total_words: usize) -> f64 {
        query_count as f64 / total_words as f64 
    }
    
    #[inline]
    pub fn compute_idf(doc_count: usize, containing_docs: usize) -> f64 {
        if containing_docs == 0 {
            0.0 
        } else {
            (doc_count as f64 / containing_docs as f64).ln() 
        }
    }
    
    #[inline]
    pub fn compute_tf_idf(tf: f64, idf: f64) -> f64 {
        tf * idf
    }
}