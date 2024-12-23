use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
    class: String,
}

fn main() {
    println!("Hello, world!");
    let paragraph: Paragraph = Paragraph {
        name: String::from("Sudhir"),
        class: String::from("First"),
    };

    let json = serde_json::to_string(&paragraph).unwrap();
    println! {"This is json {}",json};
}
