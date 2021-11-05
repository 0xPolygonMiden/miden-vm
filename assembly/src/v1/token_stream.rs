use core::fmt;

// TOKEN STREAM
// ================================================================================================

pub struct TokenStream<'a> {
    tokens: Vec<&'a str>,
    current: Vec<&'a str>,
    pos: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a str) -> Self {
        let tokens = source.split_whitespace().collect::<Vec<_>>();
        let current = tokens[0].split('.').collect::<Vec<_>>();
        Self {
            tokens,
            current,
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn read(&self) -> Option<&[&str]> {
        if !self.current.is_empty() {
            Some(&self.current)
        } else {
            None
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current.clear();
        if self.pos < self.tokens.len() {
            self.tokens[self.pos]
                .split('.')
                .for_each(|value| self.current.push(value));
        }
    }
}

impl<'a> fmt::Display for TokenStream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.tokens[self.pos..])
    }
}
