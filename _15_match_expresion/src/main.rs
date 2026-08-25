enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command {
    fn serialize(&self) -> String {
        let json = match self {
            Command::Undo => String::from(
                "{
                \"cmd\": \"undo\"
                }"
            ),
            Command::Redo => String::from(
                "{
                \"cmd\": \"redo\"
                }"
            ),
            Command::AddText(s) => {
                format!("
                {{
                \
                \"cmd\": \"addText\", \
                \"text\": \"{s}\" 
                }}")
            },

            Command::MoveCursor(line, column) => {
                format!("
            {{
                \
                \"cmd\": \"moveCursor\", \
                \"from\": \"{line}\",\
                \"to\": \"{column}\"
                
            }}")
            },

            Command::Replace {from, to} => {
                format!("
            {{
                \
                \"cmd\": \"replace\", \
                \"from\": \"{from}\", \
                \"to\": \"{to}\"
                
            }}")
            }
        };

        json
    }
}


fn main() {
    let age = 27;

    match age { // propably is like a switch statement in other languages
        1..=10 => println!("you are a toddler"),
        11..=15 => println!("You are a teenager"),
        16..=25 => println!("You are on fire age"),
        x => println!("You are a {} years old Men", x),
    }
    let cmd1 = Command::Undo;
    let cmd2 = Command::Redo;
    let cmd3 = Command::AddText(String::from("hi ma nigga"));
    let cmd4 = Command::MoveCursor(17, 35);
    let cmd5 = Command::Replace{
        from: String::from("x"),
        to: String::from("z")
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    println!("{}", cmd5.serialize());
}
