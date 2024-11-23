use std::collections::HashMap;
use std::borrow::Cow;

type TermFreqMap<'a> = HashMap<Cow<'a, str>, usize>;
type DocumentMap<'a> = HashMap<String, TermFreqMap<'a>>;

pub fn compute_tf(doc: &TermFreqMap, query: &str) -> f64 {
    let total_words: usize = doc.values().sum();
    let query_count = *doc.get(query).unwrap_or(&0); 
    query_count as f64 / total_words as f64 
}

pub fn compute_idf(documents: &DocumentMap, query: &str) -> f64 {
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
pub fn compute_tf_idf(tf: f64, idf: f64) -> f64 {
    tf * idf
}