use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("world"), 1);
    map.insert(String::from("rust"), 2);

    map.remove("world");

    let cnt: Option<&i32> = map.get("rust");

    for(text, number) in &map {
        println!("{} -> {}", text, number);
    }

    hash_map_words_stats_poem();
}

fn words_stats(words: &Vec<String>) -> HashMap<String, i32> {

    let mut map = HashMap::new();

    for (_, word) in words.iter().enumerate() {
        if map.contains_key(word) {
            *map.get_mut(word).unwrap() += 1;
        } else{
            map.insert(word.clone(), 1);
        }
    }

    map
}

fn hash_map_words_stats_poem() {
    let response = reqwest::blocking::get("https://wolnelektury.pl/media/book/txt/pan-tadeusz.txt").expect("Cannot get poem from a given URL");
    let poem = response.text().unwrap();

    let stats = words_stats(&split_to_words(&poem));
    let sorted_stats = sort_stats(&stats);
    println!("{:?}", &sorted_stats);
}


fn split_to_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    for word in s.split_whitespace() {
        let unified_word = word.trim_matches(|c| char::is_ascii_punctuation(&c)).to_lowercase();
        words.push(unified_word);
    }
    words
}

fn sort_stats(stats : &HashMap<String, i32>) -> Vec<(&str, i32)> {
    let mut sorted_stats : Vec<(&str, i32)> = Vec::new();
    for (word, count) in stats.iter() {
        sorted_stats.push((word, *count));
    }

    sorted_stats.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).expect("FAILED"));

    sorted_stats
}