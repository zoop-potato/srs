pub fn trim_scope(s: &str) -> Result<&str, String> {
    s.get(1..s.len() - 1).ok_or::<String>("Shrug".into())
}

/// s: The String slice to search in
/// index: The index into s that you are to find the scope of
/// depth: How many scopes beyond the first to look for as the return value
pub fn find_scope_of_index(s: &str, index: usize, depth: u8) -> Result<&str, String> {
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

pub fn find_scopes_containing_index(s: &str, index: usize) -> Result<Vec<&str>, String> {
    todo!()
}

pub fn find_all_scopes(s: &str) -> Result<Vec<(usize, usize)>, String> {
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

pub fn oe() -> String {
    "((OE))".to_string()
}
