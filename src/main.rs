#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::fs::*;
use ron::to_string;

mod srs;
use crate::srs::{utils::*, *};


fn main() {}

fn expand_srs(s: &str) -> Result<String, String> {
    let mut ret_string = String::new();
    for (i, c) in s.char_indices() {
        match c {
            'O' => ret_string.push_str(trim_scope(find_scope_of_index(s, i, 1)?)?),
            'E' => ret_string.push_str(trim_scope(find_scope_of_index(s, i, 0)?)?),
            'I' => ret_string.push_str(find_scope_of_index(s, i, 0)?),
            _ => ret_string.push(c),
        }
    }
    Ok(ret_string)
}

fn print_loop(s: String) {
    let mut the_big_list = vec![s];
    println!("1: {}", the_big_list.last().unwrap());
    for i in 2..8 {
        the_big_list.push(expand_srs(&the_big_list.last().unwrap()).unwrap());
        println!("{i}, Len {}: ", the_big_list.last().unwrap().len());
    }
}

fn list_scopes_of_every_index(s: String) {
    let mut list = vec![s];
    for _ in 0..4 {
        list.push(expand_srs(list.last().unwrap()).unwrap());
    }
    let test = list.last().unwrap();
    let all_scopes = find_all_scopes(&test).unwrap();

    for i in 0..test.len() {
        let scopes_of_index = all_scopes
            .iter()
            .filter(|(x, y)| *x <= i && *y >= i)
            .map(|(x, y)| (*x, *y))
            .collect::<Vec<(usize, usize)>>();
        println!("{:?}", scopes_of_index);
    }
}