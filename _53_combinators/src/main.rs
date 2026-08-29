#[derive(Debug)]
struct Students{
    name:String,
    gpa:f32
}
fn main() {

    let students = vec![
        "Regen 3.5",
        "Rayan 3.7",
        "Lidya 3.2",
        "Matilda 3.8",
        "Thomas 3.8",
        "Ahmed 4.0",
        "Munir 3.6",
        "Arthur 3.0"
    ];
    // using combinators
    let top_students:Vec<Students> = students.iter()
    .filter_map(| s |{
        let mut s = s.split(' ');
        let name = s.next()?.to_owned();
        let gpa = s.next()?.parse::<f32>().ok()?;
        if gpa < 3.5{return None}
        Some(Students{name, gpa})

    })
    .collect();


    // using unclean code
    let mut good_students: Vec<Students> = vec![];
    
    for student in students {
        let mut s = student.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa){
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();
           if let Ok(gpa) = gpa && gpa >= 3.5{
                good_students.push(Students{
                    name,
                    gpa
                });
           }
        }
    }
    
    for good in top_students{


     print!("\n{:?}", good);
        
    }

    print!("\n\n result of none clean code \n");
    for good in good_students{


     print!("\n{:?}", good);
        
    }
}
