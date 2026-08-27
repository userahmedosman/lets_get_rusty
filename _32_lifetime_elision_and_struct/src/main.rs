struct Tweet<'a> {
    content: & 'a str
}

impl <'a> Tweet<'a> {
    
    
    fn replace(&mut self, new_content: & 'a str) -> &str {
        let old = self.content;
        self.content = new_content;

        old
    }
}

fn main() {

    let mut tweet = Tweet{
        content:"first tweet"
    };

    let old = tweet.replace("new mamasita tweet");

    print!(" old tweet: {}", old);
    println!("\nnew tweet: {}", tweet.content);
}


