fn main() {
    let tweet = String::from("This is a very long long tweet you can read");
    let trimmed_tweet = tweet_trim(&tweet);

    println!("{trimmed_tweet}");

    let a: [i32; 6] = [1, 2 ,3 ,4 ,5 ,6];
    array_trim(&a);
}

fn tweet_trim(st: &str) -> &str {
     &st[..20]
}

fn array_trim(a: &[i32]){
    let slice = &a[..3];
    println!("{:?}", slice);
}