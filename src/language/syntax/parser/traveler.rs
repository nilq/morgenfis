use lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Traveler {
    pub tokens: Vec<Token>,
    top: usize,
}

#[allow(dead_code)]
impl<'a> Traveler {
    pub fn new(tokens: Vec<Token>) -> Traveler {
        Traveler {
            tokens: tokens,
            top: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.top < self.tokens.len() {
            self.top += 1;
            return true
        }
        false
    }

    pub fn prev(&mut self) -> bool {
        if self.top > 0 {
            self.top -= 1;
            return true
        }
        false
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len() - self.top + 1
    }

    pub fn current(&self) -> &Token {
        if self.top > self.tokens.len() - 1 {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[self.top]
    }

    pub fn get(&self, i: usize) -> &Token {
        assert!(i > 0 && i < self.tokens.len(), "trying to get imaginary token!");
        &self.tokens[i]
    }

    pub fn current_content(&self) -> String {
        self.current().content().clone()
    }

    pub fn expect(&self, token: TokenType) -> Result<&Token, String> {
        if self.current().token_type == token {
            Ok(self.current())
        } else {
            Err(format!("expected '{:?}', found '{:?}'", token, self.current()))
        }
    }

    pub fn expect_content(&self, content: String) -> Result<&Token, String> {
        if self.current_content() == content {
            Ok(self.current())
        } else {
            Err(format!("expected '{}', found '{:#?}'", content, self.current()))
        }
    }

    pub fn expecte_contents(&self, sequence: Vec<String>) -> Result<Vec<&Token>, String> {
        let mut accum: usize = 0;

        let mut res = Vec::new();

        for c in sequence {
            if self.top + accum >= self.tokens.len() {
                return Err(format!("expected '{}', found end of source >:(", c))
            }

            if &c != self.tokens[self.top + accum].content() {
                return Err(format!("expected '{}', found '{}'", c, self.tokens[self.top + accum].content()))
            }

            res.push(self.get(self.top + accum));

            accum += 1
        }

        Ok(res)
    }
}