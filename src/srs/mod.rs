use serde::{Deserialize, Serialize};

pub mod utils;
pub mod scope;
use crate::srs::utils::*;

#[derive(Serialize, Deserialize)]
pub struct SRSStep {
    origin: String,
    steps_from_origin: usize,
    current_state: String,
}

impl SRSStep {
    pub fn next_step(&self) -> Result<Self, String> {
        let mut ret_string = String::new();
        let srs = &self.current_state;
        for (i, c) in srs.char_indices() {
            match c {
                'O' => ret_string.push_str(trim_scope(find_scope_of_index(srs, i, 1)?)?),
                'E' => ret_string.push_str(trim_scope(find_scope_of_index(srs, i, 0)?)?),
                'I' => ret_string.push_str(find_scope_of_index(srs, i, 0)?),
                _ => ret_string.push(c),
            }
        }

        Ok(SRSStep {
            origin: self.origin.clone(),
            steps_from_origin: self.steps_from_origin + 1,
            current_state: ret_string,
        })
    }
}
