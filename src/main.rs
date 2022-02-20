// only words are reverse - Rusty version
fn main() {
    let sentence = "ylno sdrow era esrever".to_owned();

    let mut result: Vec<String> = vec![];
    for word in sentence.split(" ") {
        result.push(word.chars().rev().collect::<String>());
    }
    println!("{:#?}", result.join(" "));
}
