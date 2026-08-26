trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String){
        print!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}
struct Car {
    info: VehicleInfo
}

struct House {

}

impl Paint for House {
    fn paint(&self, color: String){
        print!("printint house... {}", color);
    }
}

impl Park for Car{
    fn park(&self) {
        print!("parking car..");
    }
}

impl Paint for Car {
   fn paint(&self, color: String) {
      
      
       print!("\npainting car:{1} {} {color}", self.info.make, self.info.model);
   }
}

struct Truck {
   info: VehicleInfo
}

impl Park for Truck{
    fn park(&self) {
        print!("parking truck..");
    }
}

impl Paint for Truck  {
    fn paint(&self, color: String) {
        print!("painting truck: {}", color);
    }
}

impl Truck {
    fn unload(&self){
        print!("unloading truck");
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 2014
        }
    };
    let truck = Truck {
        info:VehicleInfo { make: "Mercedes".to_owned(), model: "Avici".to_owned(), year: 2011 }
    };
    let house = House {};

    let anyobj = any_paint_bound_object();
    paint_red_then_park(&car);
    paint_red_then_park(&truck);
    
   
   
}

fn paint_red_then_park<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
    object.park();
}

fn park<T: Park>(object: &T){
    object.park();
}

fn any_paint_bound_object() -> impl Paint {
    Car { info: VehicleInfo { make: "Toyota".to_owned(), model: "Corola".to_owned(), year: 2010 }}
}
