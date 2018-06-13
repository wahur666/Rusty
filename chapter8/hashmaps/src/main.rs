use std::collections::HashMap;

fn main() {

    // Creating a New Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);

    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Accessing Values in a Hash Map

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a Value
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    // Only Inserting a Value If the Key Has No Value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
