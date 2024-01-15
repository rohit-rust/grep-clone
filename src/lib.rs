pub fn search<'a>(query: &str , contents: &'a str ) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect() 
}



pub fn search_insensitive<'a>(query: &str , contents: &'a str ) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line|{
            let query_lower = query.to_lowercase();
            line.contains(&query_lower)
        })
        .collect()
}