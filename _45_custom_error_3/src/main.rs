use std::{collections::HashMap, error::Error, fmt::{Display, format}, io};


struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String
}

impl std::fmt::Debug for ParsePaymentInfoError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}\n\t{}", self.msg)?;

        if let Some(e) = self.source.as_ref(){
            write!(f, "\n\nCaused by: \n\t{e:?}")?;
        }

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let credit_cards= HashMap::from([
        ("Julian Blake", "1423561 Aug 27 477"),
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


enum CardErrors{
    InvalidInputError(String),
    Other(Box<dyn Error>, String)
}

impl std::fmt::Debug for CardErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInputError(msg) => write!(f, "{self}\n{msg}"),
            Self::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by\n\t{e:?}"),
        }
    }
}

impl Display for CardErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Credit card error: could not retrive credit card.")
    }
}

impl Error for CardErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CardErrors::InvalidInputError(_) => {
                None
            },
            CardErrors::Other(e,_ ) => {
                Some(e.as_ref())
            }
        }
    }
}

fn get_credit_card_info( cd:&HashMap<&str, &str>, name:&str) -> Result<CreditCardInfo, CardErrors>{
    let get_card_string = cd.get(name).ok_or(CardErrors::InvalidInputError(format!{"no card data found with the given name: {name}"}))?;

    parse_card_info(get_card_string)
    .map_err(|e | {
        CardErrors::Other(Box::new(e),format!{"{name}'s card failed to be parsed"})})
} 

fn parse_card_info(info: &str) -> Result<CreditCardInfo, ParsePaymentInfoError> {
    let mut data = parse_data(info)?;

    let len = data.len();
    let expectd_len = 4;

    if len != expectd_len {
       return Err(ParsePaymentInfoError { 
        source: None, 
        msg: format!("Incorrect number of elements parsed.\n
       Expected {expectd_len} but got {len} elements.\n
       data: {data:?}")})
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

impl Display for ParsePaymentInfoError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.write_str("Parsing payment error: invalid payment info")
    }
}

impl Error for ParsePaymentInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}


fn parse_data(info:&str) -> Result<Vec<u32>, ParsePaymentInfoError> {
   let numbers = info
        .split(" ")
        .map(|s | {
            s.parse().map_err(| _e | {
                ParsePaymentInfoError { 
                    source: None, 
                    msg: format!("{s:?} could not be parsed as u32") }
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e | {
            ParsePaymentInfoError { 
                source: Some(Box::new(e)), 
                msg: format!("faild to parse input as numbers {info}") }
        })?;

    Ok(numbers)

}
