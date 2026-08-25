struct Product {
    name: String,
    price: f32,
    is_in_stock: bool
}

impl Product {

    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            is_in_stock: true
        }
    }

    fn get_default_tax() -> f32 {
        0.1
    }

    fn calculate_sale_tax(&self) -> f32{
    return self.price * Product::get_default_tax();
    }

    fn set_discount(&mut self, price: f32){
        self.price =  price;
    }

    fn buy_product(self) -> i32 {
        let name: String = self.name;
        println!("{} has been bought !", name);

        123
    }
}

fn main() {
    let mut product = Product::new(
        String::from("Dell G15 Gaming Laptop"),
        1200.0,
    );
    if product.is_in_stock{ 
    product.set_discount(1015.0);
    let tax = product.calculate_sale_tax();
    println!("{1} tax is: {}$", tax, product.name);
    let id = product.buy_product();
    println!("product package id: {}", id);
    
    }
}



