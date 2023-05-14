pub mod wallet;

#[cfg(test)]
mod tests {
    use crate::wallet::HDSeed;
    use crate::wallet::HDWallet;
    use bip39::{Language, Mnemonic};

    #[test]
    fn test_wallet() {
        let eth_addrs = vec![
            "0xa03eF5A8A00b938886b5e54b228759Ce8cBb6bF5",
            "0x94A902854842c4A5931AB49E558690b1dfd16394",
            "0x7c132f9840602B086b51aEAa6367518b210C69D7",
        ];
        let tron_addrs = vec![
            "TDLSuRq683BHiBuV9oSBFTxxJs1U1YuT1n",
            "THh8xUn3R8B51U6UD8YAp9mhUkcRM8fJEL",
            "TDe2FTTE6B16LzDUKLoWQvvF4iEEdYHCnM",
        ];

        let mnemonic = Mnemonic::from_phrase(
            "debris resemble coil soul shrimp slender deal aunt twenty gown fee test",
            Language::English,
        )
        .unwrap();
        let hdw_eth = HDWallet::Ethereum(HDSeed {
            mnemonic: mnemonic.clone(),
        });
        let hdw_tron = HDWallet::Tron(HDSeed { mnemonic });
        for i in 0..3 {
            let eth_i = hdw_eth.address(i as i32);
            let tron_i = hdw_tron.address(i as i32);
            println!("=======================");
            println!("ETH");
            println!("addr: {:?}", eth_i);
            println!("TRON");
            println!("addr: {:?}", tron_i);
            assert_eq!(eth_addrs[i], eth_i);
            assert_eq!(tron_addrs[i], tron_i);
            println!("=======================");
        }
    }
}
