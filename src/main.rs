// only words are reverse - Rusty version
fn main() {
    let sentence = "ylno sdrow era esrever".to_owned();

    let mut result: Vec<String> = vec![];
    for word in sentence.split(" ") {
        result.push(word.chars().rev().collect::<String>());
    }
    println!("{:#?}", result.join(" "));
    capacity_check();

    //    smart_reverse();
    // chars_counter();
    split_collect();
}

fn split_collect() {
    for elem in 0..=10 {
        println!("\n")
    }
    let message = "hello cruel world";
    let result = for word in message.split(" ") {
        return word.chars().rev();
    }
    println!("{:#?}", result);
}

fn chars_counter() {
    let word = "testing";
    let count = word.chars().count();
    println!("{}", count);
    for each in word.chars() {
        println!("{}", each);
    }
}

fn capacity_check() {
    let mut message = String::with_capacity(75);
    message.push_str("this is some text");
    println!("length: {}", message.len());
    println!("{}", message.capacity());
    message.shrink_to_fit();
    println!("{}", message);
    println!("{}", message.capacity());
    message.remove(14);
    println!("{}", message);
    message.retain(|ch| ch != 't');
    println!("{}", message);
}

// fn smart_reverse() {
//     let mut sentence = "only words are reverse".to_owned();
//     let sentence_copy = sentence.as_mut_vec();
//     sentence_copy.reverse();
//     println!("{}",);
// }
