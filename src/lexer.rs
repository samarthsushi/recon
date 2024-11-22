pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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

    pub fn next_token(&mut self) -> Option<String> {
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