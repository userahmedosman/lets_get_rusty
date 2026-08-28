use std::{collections::HashMap, io, num::ParseIntError};

fn main() {
    env_logger::init();
    let credit_cards= HashMap::from([
        ("Julian Blake", "1423561 08 27 477"),
        ("Stephen Muller", "3135269 02 27 341"),
        ("Sara Mike", "4552363 1226 776")

    ]);

    println!("Enter name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");

    let result_card = get_credit_card_info(&credit_cards, name.trim());
    
    match result_card {
        Ok(data) => {print!("Credit Card Info: {data:?}");},
        Err(err) => { 
            match &err{
            CardErrors::InvalidInputError(e) => {
                print!("{e}");
            },
            CardErrors::Other(_, _) => {
                print!("Something went wrong !");
            }

        }
            log::error!("\n{err:?}");
    }
        
    }
}
#[derive(Debug)]
struct CreditCardInfo {
    cardnum: u32,
    expire: Expiration,
    cvv: u32
}
#[derive(Debug)]
struct Expiration {
    month: u32,
    year: u32
}

#[derive(Debug)]
enum CardErrors{
    InvalidInputError(String),
    Other(String, String)
}

fn get_credit_card_info( cd:&HashMap<&str, &str>, name:&str) -> Result<CreditCardInfo, CardErrors>{
    let get_card_string = cd.get(name).ok_or(CardErrors::InvalidInputError(format!{"no card data found with the given name: {name}"}))?;

    parse_card_info(get_card_string).map_err(|e | {CardErrors::Other(e,format!{"{name}'s card failed to be parsed"})})
} 

fn parse_card_info(info: &str) -> Result<CreditCardInfo, String> {
    let mut data = parse_data(info).map_err(|e | {e.to_string()})?;

    let len = data.len();
    let expectd_len = 4;

    if len != expectd_len {
       return Err(format!{"Incorrect number of elements parsed.\n
       Expected {expectd_len} but got {len} elements.\n
       data: {data:?}"})
    }
   
    let _cvv = data.pop().unwrap();
    let _year = data.pop().unwrap();
    let _month = data.pop().unwrap();
    let card_number = data.pop().unwrap();
    Ok(CreditCardInfo { 
        cardnum: card_number, 
        expire: Expiration { 
            month: _month, 
            year: _year 
        }, 
        cvv: _cvv})
}

fn parse_data(info:&str) -> Result<Vec<u32>, ParseIntError> {
   let numbers = info
        .split(" ")
        .map(|s | {
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)

}
