struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    is_in_stock: bool
}

enum ProductCategory {
    Cloths,
    Food,
    Electronics,
}

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
        String::from("Json serialized")
    }
}

fn main() {
    let category = ProductCategory::Electronics;
    let product = Product {
        name: String::from("Dell G15 Gaming"),
        category,
        price: 1420.0,
        is_in_stock: true
    };
    println!("{}", product.name);

    let cmd = Command::Undo;
    let cmd = Command::Redo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(25, 6);
    let cmd = Command::Replace {
        from: String::from("a"),
        to: String::from("b")
    };

    let serializer = cmd.serialize();
    println!("{}", serializer);
}
