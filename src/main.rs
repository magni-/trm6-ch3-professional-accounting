#![feature(plugin)]
#![feature(custom_derive)]

#![plugin(clippy)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io;

#[derive(Debug)]
enum AccountError {
    IO,
    JSON,
    NegativeBalance(i64)
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountError::NegativeBalance(amount) => write!(f, "{}: {}", StdError::description(self), amount),
            _ => write!(f, "{}", StdError::description(self)),
        }
    }
}

impl StdError for AccountError {
    fn description(&self) -> &str {
        match *self {
            AccountError::IO => "IO error",
            AccountError::JSON => "JSON error",
            AccountError::NegativeBalance(_) => "Negative balance",
        }
    }
}

impl From<io::Error> for AccountError {
    fn from(_: io::Error) -> AccountError {
        AccountError::IO
    }
}

impl From<serde_json::Error> for AccountError {
    fn from(_: serde_json::Error) -> AccountError {
        AccountError::JSON
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Account {
    id: String,
    transactions: Vec<Transaction>,
}


impl Account {
    fn load(filename: &str) -> Result<Account, AccountError> {
        let file = try!(File::open(&filename));
        let account: Account = try!(serde_json::from_reader(file));
        Ok(account)
    }

    fn balance(&self) -> Result<i64, AccountError> {
        let balance = self.transactions.iter().fold(0, |sum, ref t| sum + t.amount);
        if balance < 0 {
            return Err(AccountError::NegativeBalance(balance))
        }
        Ok(balance)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Transaction {
    id: String,
    amount: i64,
}

fn main() {
    let account = match Account::load("account.json") {
        Ok(account) => account,
        Err(err) => panic!("Aaand it's all gone: {}", err),
    };
    let balance = account.balance().expect("Impossible balance");
    println!("Balance of account {} is {}", account.id, balance);
}

#[test]
fn should_load_account() {
    Account::load("account.json").unwrap();
}

#[test]
fn should_calculate_balance() {
    let account = Account::load("account.json").unwrap();
    assert_eq!(account.balance(), 4536);
}
