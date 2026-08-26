use bank::SavingAccount;
mod utils;

#[test]

pub fn saving_account_should_set_initial_account_balance_to_0(){
    utils::common_setup();
    let account = SavingAccount::new();

    assert_eq!(account.get(), 0, "balance should be zero");
}