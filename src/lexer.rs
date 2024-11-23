use std::borrow::Cow;

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,  
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.cursor += c.len_utf8();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.cursor..].chars().next()
    }

    pub fn next_token(&mut self) -> Option<Cow<'a, str>> {
        self.skip_whitespace();

        let start = self.cursor;
        let mut requires_cleanup = false;

        while let Some(c) = self.peek_char() {
            if c.is_whitespace() { break };
            if c.is_alphanumeric() {
                self.cursor += c.len_utf8();
            } else {
                self.cursor += c.len_utf8();
                requires_cleanup = true;
            }
        }

        if start != self.cursor {
            let slice = &self.input[start..self.cursor];

            if requires_cleanup {
                let cleaned: String = slice.chars().filter(|c| c.is_alphanumeric() || *c == '\'').collect();
                Some(Cow::Owned(cleaned))
            } else {
                Some(Cow::Borrowed(slice))
            }
        } else {
            None
        }
    }
}
