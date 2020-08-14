pub struct KeyBoard {
    pressed_keys: Vec<char>,
}

impl KeyBoard {
    pub fn new() -> Self {
        Self {
            pressed_keys: vec![],
        }
    }
    pub fn add_key(&mut self, key: char) {
        match key {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | '0' | '1'
            | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.pressed_keys.push(key),
            _ => {}
        }
        println!("{:#?}", self.pressed_keys);
    }

    pub fn get_key(&mut self) -> char {
        // TODO: what if vector is empty
        self.pressed_keys.remove(0)
    }
}
