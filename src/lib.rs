pub fn search<'a>(query: &str , contents: &'a str ) -> Vec<&'a str> {
    let mut data = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            data.push(line)
        }
    }
    data
}



pub fn search_insensitive<'a>(query: &str , contents: &'a str ) -> Vec<&'a str> {
    let mut data = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.contains(&query) {
            data.push(line)
        }
    }
    data
}