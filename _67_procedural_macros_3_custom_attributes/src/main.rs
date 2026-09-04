use _67_procedural_macros_3_custom_attributes::*;


#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let product = Product {
        name: "Dell G15 Gaming".to_string(),
        price: 1150,
    };
    buy_product(product, 10);
}

#[log_call(verbose = true)]
fn buy_product(product: Product, discount: u32) {
    println!("Buying product: {:?}", product);
}