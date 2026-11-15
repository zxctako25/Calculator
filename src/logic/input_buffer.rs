pub const MAX_DIGITS: usize = 32;

pub struct InputBuffer {
    text: String,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self { 
            text: "0".to_string() 
        }
    }
}

impl InputBuffer {
    pub fn reset(&mut self) {
        self.text = "0".to_string();
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn value(&self) -> f64 {
        self.text.parse().unwrap_or(0.0)
    }

    pub fn push_digit(&mut self, d: char) {
        if self.text == "0" {
            if d == '0' {
                return;
            }
            self.text.clear();
        }
        
        if self.text.chars().filter(|c| c.is_ascii_digit()).count() >= MAX_DIGITS {
            return;
        }
        self.text.push(d);
    }

    pub fn push_decimal(&mut self) {
        if !self.text.contains('.') {
            self.text.push('.');
        }
    }

    pub fn backspace(&mut self) {
        self.text.pop();
        if self.text.is_empty() || self.text == "-" || self.text == "-0" {
            self.text = "0".to_string();
        }
    }

    pub fn toggle_sign(&mut self) {
        if self.value() == 0.0 {
            return;
        }
        
        if self.text.starts_with('-') {
            self.text.remove(0);
        } else {
            self.text.insert(0, '-');
        }
    }
}
