use std::{io::{stdin, BufRead}, fmt::Display};

struct PersonInformation {
    name: String,
    age: i32,
}

trait Printable {
    fn format_name_age(&self) -> String;
}

impl Printable for PersonInformation {
    fn format_name_age(&self) -> String {
        format!("{} ({})", self.name, self.age)
    }
}

impl Display for PersonInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Hello, {}!, You are {} years old", self.name.trim_end(), self.age))
    }
}

impl PersonInformation {
    fn format_message(&self) -> String {
        format!(
            "Hello, {}!, You are {} years old",
            self.name.trim_end(),
            self.age
        )
    }
}

fn main() {
    let mut buffer = String::new();
    let mut handle = stdin().lock();
    let result = handle.read_line(&mut buffer);

    match result {
        Ok(size) => println!("Read {} bytes", size),
        Err(_) => println!("Something went wrong"),
    }

    let person = PersonInformation {
        name: buffer,
        age: 21
    };

    println!(
        "Hello, {}!",
        person
    );
}

#[cfg(test)]
mod tests {
    use crate::{PersonInformation, Printable};

    #[test]
    fn format_message_display_trait_implementation() {
        let person = PersonInformation { name: "Richard".to_owned(), age: 21 };
        assert_eq!("Hello, Richard!, You are 21 years old", format!("{}", person)); 
    }

    #[test]
    fn format_message_method() {
        let person = PersonInformation { name: "Richard".to_owned(), age: 21 };
        assert_eq!("Hello, Richard!, You are 21 years old", person.format_message()); 
    }

    #[test]
    fn format_message_custom_trait() {
        let person = PersonInformation { name: "Richard".to_owned(), age: 21 };
        assert_eq!("Richard (21)", person.format_name_age()); 
    }
}
