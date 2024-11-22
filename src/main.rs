use std::{
    fs,
    io::{self, Read}
};

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

        match self.chars.next()? {
            c if c.is_alphanumeric() => {
                let mut word = String::new();
                word.push(c); 
                word.push_str(&self.extract_word());
                Some(word)
            }
            _ => None, 
        }
    }
}

fn main() -> io::Result<()> {
    let current_dir = r"data\blonde_plaintext";

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "txt") {
            let mut file_content = String::new();
            let mut file = fs::File::open(&path)?;
            file.read_to_string(&mut file_content)?;

            let file_content = file_content.to_lowercase();

            let mut freq = std::collections::HashMap::new();
            let mut lexer = Lexer::new(&file_content);

            while let Some(word) = lexer.next_token() {
                *freq.entry(word).or_insert(0) += 1;
            }
            println!("{:?}:", path.file_name().unwrap());
            for (word, count) in &freq {
                println!("\t{}: {}", word, count);
            }
        }
    }

    Ok(())
}
