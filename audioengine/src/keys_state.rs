
use std::collections::VecDeque;

use types::KeyAction;

pub struct KeysState {
    state: VecDeque<i32>,
}

impl KeysState {

    pub fn new() -> Self {
        Self {
            state: VecDeque::new()
        }
    }

    pub fn key_down(&mut self, key_action: KeyAction) -> Option<i32> {
        match key_action {
            KeyAction::Press(value) => {
                self.remove_key(value);
                self.state.push_front(value);
                self.state.front().cloned()
            },
            KeyAction::Release(value) => {
                self.remove_key(value);
                self.state.front().cloned()
            }
        }
    }

    fn remove_key(&mut self, key: i32) {
        let index_option = self.state.iter().enumerate().find(|(i, &v)| v == key).map(|(i, _)| i);
        if let Some(index) = index_option {
            self.state.remove(index);
        }
    }
}