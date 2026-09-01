
#[macro_export]
macro_rules! hashmap {
    ($key:ty, $value:ty) => {
        {
        let hm:HashMap<$key, $value> = HashMap::new();
        hm
        }
    };

    ($($key:expr => $value:expr),*) => {
        {
            let mut hm: HashMap<String, i32> = HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    }
}