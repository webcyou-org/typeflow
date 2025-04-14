use std::{str::Chars, thread, time::Duration};

pub struct CharFlow<'a> {
    chars: Chars<'a>,
    delay: Duration,
}

impl<'a> CharFlow<'a> {
    pub fn new(text: &'a str, delay_millis: u64) -> Self {
        CharFlow {
            chars: text.chars(),
            delay: Duration::from_millis(delay_millis),
        }
    }
}

impl<'a> Iterator for CharFlow<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.chars.next();
        if next_char.is_some() {
            thread::sleep(self.delay);
        }
        next_char
    }
}

