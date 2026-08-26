trait Vehicle: Paint { // supertrait
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String){
        print!("painting object: {}", color);
    }
}



struct Motor  {
    make: String
}

impl Paint for Motor{
    fn paint(&self, color: String) {
        print!("\npainting motor:{} with {color}", self.make);
    }
}

impl Vehicle for Motor {
    fn park(&self) {
        print!("\nparking motor");
    }

    
}
struct Car {
    make: String,
    model: String,
    year: u16
}

impl Vehicle for Car {
    fn park(&self) {
        print!("\nparking car.");
    }
}

impl Paint for Car {
   fn paint(&self, color: String) {
       print!("\npainting car:{1} {} {color}", self.make, self.model);
   }
}

fn main() {
    let car = Car {
        make: "Toyota".to_owned(),
        model: "Canari".to_owned(),
        year: 2009
    };

    let motor = Motor {
        make: "Honda".to_owned()
    };
    vehicle_action(&motor);
    vehicle_action(&car);
}


fn vehicle_action<T> (obj: &T) where T: Vehicle {
    obj.park();
    obj.paint("blue".to_owned());
}