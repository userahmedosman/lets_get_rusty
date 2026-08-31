use tokio_stream::StreamExt;
#[tokio::main]
async fn main() {
 let mut stream = tokio_stream::iter(["lets", "get", "rusty"])
 .map(| c| c.to_ascii_upper());

 while let Some(e) = stream.next().await{
    print!("{e}");
 }
}