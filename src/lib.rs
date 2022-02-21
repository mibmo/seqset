const DEFAULT_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyzæøå";

#[derive(Debug, Clone, Copy)]
struct Sequence<'a> {
    charset: &'a str,
    output_length: usize,
    charset_size: usize,
    n: usize,
    // the last valid value for `n` in the sequence (before the pattern repeats)
    last_valid_n: usize,
}

impl<'a> Sequence<'a> {
    fn new(output_length: usize) -> Self {
        Self::with_charset(output_length, DEFAULT_CHARSET)
    }

    fn with_charset(output_length: usize, charset: &'a str) -> Self {
        let charset_size = charset.chars().count();
        let last_valid_n = if output_length == 0 {
            0
        } else {
            (charset_size as f64).powi(output_length as i32) as usize
        };

        Self {
            output_length,
            charset,
            charset_size,
            n: 0,
            last_valid_n,
        }
    }
}

impl<'a> Iterator for Sequence<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.last_valid_n {
            return None;
        }

        let mut pattern = String::with_capacity(self.output_length);
        for position in (0..self.output_length).rev() {
            let i = (self.n / (self.charset_size as f64).powi(position as i32) as usize)
                % self.charset_size;
            let c = self.charset.chars().nth(i).expect("index is out of bounds");
            pattern.push(c);
        }

        self.n += 1;
        Some(pattern)
    }
}
