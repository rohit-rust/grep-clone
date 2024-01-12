
fn search<'a>(query: &str , contents: &'a str ) -> Vec<&'a str> {
    let mut data = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            data.push(line)
        }
    }
    data
}

fn search_insensitive<'a>(query: &str , contents: &'a str ) -> Vec<&'a str> {
    let mut data = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.contains(&query) {
            data.push(line)
        }
    }
    data
}



#[test]
fn search_test(){
    let query = "rust";
    
    let contents = "rust is awesome";
    assert_eq!(vec!["rust is awesome"],search(query,contents))
}

#[test]
fn search_case_insensitive(){
    let query = "RuSt";
    let contents = "rust is awesome";
    assert_eq!(vec!["rust is awesome"],search_insensitive(query,contents))
}
