

fn main() {
    let vecques = vec!["a".to_string(), "adarsh".to_string(), "together".to_string()];
    let res = filter_and_double_large_words(vecques);
    println!("{:?}",res);
}
fn filter_and_double_large_words(words: Vec<String>) -> Vec<String>{
    words.into_iter().filter(|x|x.len()>5).map(|x| x.clone()+&x).collect()
}