#[derive(Debug, PartialEq, Clone)]
pub struct StringBuilder {
    strings: Vec<String>,
    length: usize,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            length: 0,
        }
    }
    pub fn with_capacity(size: usize) -> Self {
        let mut result = Self::new();
        result.strings.reserve(size);
        return result;
    }
    pub fn push<T>(&mut self, s: T)
        where T: Into<String>
    {
        let t = s.into();
        self.length += t.len();
        self.strings.push(t);
    }
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.length);
        let mut index = 0;
        let mut additional_coefficient = 1;
        for string in self.strings.iter() {
            for character in string.chars() {
                if result.capacity() <= index {
                    additional_coefficient *= 2;
                    result.reserve(self.length * additional_coefficient);
                }
                result.push(character);
                index += 1;
            }
        }
        result.shrink_to_fit();
        return result;
    }
}
