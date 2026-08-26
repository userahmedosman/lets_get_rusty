struct BrowserCommand<T> {
    name: String,
    command: T
}

impl<T> BrowserCommand<T>{
    fn new(name:String, command:T) -> Self {
        BrowserCommand { 
            name, 
            command 
        }
    }

    fn get_payload(&self) -> &T {

        &self.command
    }
}

impl BrowserCommand<String> {
    fn show_payload(&self){
        print!("\nname: {}", self.name);
        print!("\ncommand: {}\n", self.command);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://letsgetrusty.com".to_owned()
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );

    cmd1.show_payload();

    let a = cmd1.get_payload();
    let b = cmd2.get_payload();
    
 
    
}
