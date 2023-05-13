
mod hash;
mod wallet;

use bip39::{Language, Mnemonic};
use wallet::HDWallet;

use crate::wallet::HDSeed;


fn main(){
    let mnemonic = Mnemonic::from_phrase("", Language::English).unwrap();
    let hdw_eth = HDWallet::Ethereum(HDSeed { mnemonic:mnemonic.clone() });
    let hdw_tron = HDWallet::Tron(HDSeed { mnemonic });
    for i in 0..3{
        println!("=======================");
        println!("ETH");
        println!("addr: {:?}",hdw_eth.address(i));
        println!("TRON");
        println!("addr: {:?}",hdw_tron.address(i));
        println!("=======================");
    }
}

