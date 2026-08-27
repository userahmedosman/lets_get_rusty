trait UIComponent {
    fn render(&self){
        println!("Rendering component..");
    }
}

struct Button {
    text:String
}

struct Container {
    name:String,
    t_id:Box<Container>
}

impl UIComponent for Button {}
fn main() {
    let button_c = Button{text:"Button".to_owned()};
    let button_b = Box::new(Button{text:"Box Button".to_owned()});

    println!("{}", button_c.text);
    print!("{}", button_b.text);

    let button_collection:Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_b
    ];

    for btn in button_collection{
        btn.render();
    }
}
