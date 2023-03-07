#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::fs::*;

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

/// s: The String slice to search in
/// index: The index into s that you are to find the scope of
/// depth: How many scopes beyond the first to look for as the return value
fn find_scope_of_index(s: &str, index: usize, depth: u8) -> Result<&str, String> {
    let mut scope_starts: Vec<usize> = vec![];
    let mut containing_scopes: Vec<&str> = vec![];

    for (i, c) in s.char_indices() {
        match c {
            '(' => scope_starts.push(i),
            ')' => {
                let start = scope_starts
                    .pop()
                    .ok_or::<String>("Invalid Parans".into())?;
                if start <= index && index <= i {
                    containing_scopes.push(s.get(start..=i).unwrap());
                }
            }
            _ => {}
        }
    }
    if depth as usize >= containing_scopes.len() {
        return Err("Depth is out of bounds".into());
    }
    Ok(containing_scopes[depth as usize])
}

fn find_all_scopes(s: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut scopes: Vec<(usize, usize)> = vec![];
    let mut scope_starts: Vec<usize> = vec![];

    for (i, c) in s.char_indices() {
        match c {
            '(' => scope_starts.push(i),
            ')' => {
                let start = scope_starts
                    .pop()
                    .ok_or::<String>("Invalid Parans".into())?;
                scopes.push((start, i));
            }
            _ => {}
        }
    }
    scopes.sort_by(|(a, _), (b, _)| a.cmp(b).reverse());
    Ok(scopes)
}

fn trim_scope(s: &str) -> Result<&str, String> {
    s.get(1..s.len() - 1).ok_or::<String>("Shrug".into())
}

fn oe() -> String {
    "((OE))".to_string()
}

//
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
