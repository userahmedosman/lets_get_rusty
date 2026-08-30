use std::time::Duration;
use tokio::time::sleep;
#[tokio::main(flavor = "current_thread")]
async fn main() {
   let mut handles = vec![];
   for x in 0..2 {
    let handle = tokio::spawn(async move {
        my_fun(x).await;
    });
    handles.push(handle);
   }


   handles.push(tokio::spawn(async {
    let _res = tokio::task::spawn_blocking(||{
        cpu_intersive_task();
    }).await;
}));


   for handle in handles{
    handle.await.unwrap();
   }
}

async fn my_fun(i:i32){
    print!("\n{i}:Mean while this is the first line");
    let a1 = auth("12400").await;   
    print!("\n{i}:{a1}");
    let a2 = auth("123456").await;
    print!("\n{i}:{a2}");
}

async fn get_data_from_db() -> String{
    sleep(Duration::from_millis(50)).await;
    String::from("DB:{data:xxxx}")
}

async fn auth(password:&str) -> String{
     if password == "123456"{
        print!("\nAuthentication successful");
     let db =get_data_from_db().await;
       return db
     }
     print!("\nAuthentication failed");
     String::from("0")

}

fn cpu_intersive_task(){
    let mut a = 0;
    for _ in 0..500_000_000{
        a = a +1;
    }
    print!("\nFinal intensive result is: {a} ");

}