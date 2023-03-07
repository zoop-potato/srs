struct SRS {
    string: String,
    list: Vec<SRS>,
}

impl SRS {
    fn to_string(&self) -> String {
        let mut ret_string = String::new();
        let mut list_iter = self.list.iter();
        for c in self.string.chars() {
            if c.eq(&'#') {
                ret_string.push_str(
                    &list_iter
                        .next()
                        .expect("No matching SRS found in list")
                        .to_string(),
                );
            } else {
                ret_string.push(c);
            }
        }
        ret_string
    }
}