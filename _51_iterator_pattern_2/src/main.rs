struct Employee {
    fullname: String,
    country: String,
    occupation:String
}

struct EmployeeIterator {
    values:Vec<String>
}

impl Iterator for EmployeeIterator{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty(){
            None
        }else{
            Some(self.values.remove(0))
        }
    }
}

impl IntoIterator for Employee{
    type Item = String;

    type IntoIter = EmployeeIterator;

    fn into_iter(self) -> Self::IntoIter {
            EmployeeIterator{
            values:vec![
                self.fullname,
                self.country,
                self.occupation]
            }
    }
}



fn main() {
    let employee = Employee {
        fullname: "Ahmed Osman".to_owned(),
        country: "Eritrea".to_owned(),
        occupation: "Software Engineer".to_owned()
    };
    
    
    for i in employee{
    println!("{i}");
    }
}
