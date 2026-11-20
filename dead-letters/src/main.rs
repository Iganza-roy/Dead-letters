mod letter;

use letter::Letter;

fn main() {
    let first_letter = Letter {
        id: 1,
        title: String::from("Words I Kept"),
        recipient: Some(String::from("An old friend")),
        category: String::from("Never Said"),
        date_created: String::from("2026-11-20"),
        message: String::from("There are things I wish I had said."),
    };

    println!("Letter #{}", first_letter.id);
    println!("Title: {}", first_letter.title);

    match &first_letter.recipient {
        Some(recipient) => println!("Recipient: {}", recipient),
        None => println!("Recipient: None"),
    }

    println!("Category: {}", first_letter.category);
    println!("Date: {}", first_letter.date_created);
    println!("Message: {}", first_letter.message);
}
