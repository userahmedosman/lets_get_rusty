pub struct SavingAccount {
    balance: i32
}

impl SavingAccount {
    pub fn new() -> SavingAccount {
        SavingAccount {
            balance:0
        }
    }

    pub fn get(&self) -> i32 {self.balance}

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Negative deposit is not possible");
        }    
        self.balance+= amount;
    }

    pub fn transfer(&self, account_num: i32, amount: i32) -> Result<String, String> {
        Ok(format!("Transfered {amount} to {account_num}"))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_get_the_correct_initial_balance(){
        let account = SavingAccount::new();
        assert_eq!(account.get(), 0);
    }

    #[test]
    fn shoud_get_correct_balance_after_deposit(){
        let mut account = SavingAccount::new();
        account.deposit(1000);
        assert_eq!(account.get(), 1000, "deposited 1000 to initial balance but balance is not equal to 1000 ");
        assert_ne!(account.get(), 0);
        assert!(account.get() == 1000);

    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_negative_balance(){
        let mut account  = SavingAccount::new();
        account.deposit(-5);
        
    }

    #[test]

    fn should_transfer_balance() -> Result<(), String> {
        let mut account = SavingAccount::new();

        account.deposit(100);
        account.transfer(11211, 50)?;
        Ok(())
    }
}