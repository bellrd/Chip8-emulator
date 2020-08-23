use std::collections::HashMap;

pub struct KeyBoard {
    pub pressed_key: HashMap<u8, bool>,
    pub last_pressed_key: Option<u8>,
    pub should_wait_for_key: bool,
}

impl KeyBoard {
    pub fn new() -> Self {
        let mut pressed_key: HashMap<u8, bool> = HashMap::with_capacity(15);
        for i in 0..15 {
            pressed_key.insert(i, false);
        }
        Self {
            pressed_key,
            last_pressed_key: None,
            should_wait_for_key: false,
        }
    }

    pub fn on_key_down(&mut self, key: u8) {
        self.pressed_key.insert(key, true);
        self.last_pressed_key = Some(key);
        if self.should_wait_for_key == true {
            self.should_wait_for_key = false
        }
        //if let Some(handler) = self.handler {
        //    handler();
        //    self.handler = None;
        //}
    }

    pub fn on_key_up(&mut self, key: u8) {
        self.pressed_key.insert(key, false);
    }
}
