struct Product {
    name: String,
    price: f32,
    is_in_stock: bool
}


fn main() {
    let product = Product {
        name: String::from("Dell G15 Gaming Laptop"),
        price: 1200.0,
        is_in_stock: true
    };
    if product.is_in_stock{ 
    let tax = calculate_sale_tax(&product);

    println!("{1} tax is: {}$", tax, product.name);
    }
}

fn calculate_sale_tax(product: &Product) -> f32{
    return product.price * 0.1;
}
