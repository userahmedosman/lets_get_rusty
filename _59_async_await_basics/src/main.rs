
#[tokio::main]
async fn main() {
    my_fun().await;
    println!("Hello, world!");
}

async fn my_fun(){
    print!("Mean while this is the first line");
    let a1 = auth("12400").await;   
    print!("{a1}");
    let a2 = auth("123456").await;
    print!("{a2}");
}

async fn get_data_from_db() -> String{
    String::from("db_name: mysql, port:1447")
}

async fn auth(password:&str) -> String{
     if password == "123456"{
        print!("Authentication successful");
     let db =get_data_from_db().await;
       return db
     }
     print!("Authentication failed");
     String::from("0")

}
