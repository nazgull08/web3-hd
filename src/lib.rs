pub mod wallet;

#[cfg(test)]
mod tests {
    use crate::wallet::HDSeed;
    use crate::wallet::HDWallet;

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

        let tron_privs = vec![
            "a5c84bafe7d65767b679da87d01353390b3f3f7bfd4eb043052e4a64360e68c9",
            "cb64fa4e279260bb4e0c9a9f1cca1aeb52f76b44e62fbcb98ed6589c500cf980",
            "8d7fcdb80cf9a48856d2162b01ddec578a99ed8d158445d887abe29386082e14",
        ];

        let eth_privs = vec![
            "3b1c95e0545b8d31718ca5f616f263eefca71bb644372be5c279b5d80ca7b76b",
            "125b8a21b7b1a603be86dbf459d840415ae64cc8c3271697417bb6bd1436380f",
            "6de12bbe8e09b7ef5d1e34becd5f554f2e11e8ebae4a48ebf2a2c63ccc074f30",
        ];

        let phrase = "debris resemble coil soul shrimp slender deal aunt twenty gown fee test";
        let hdw_eth = HDWallet::Ethereum(HDSeed::new(phrase));
        let hdw_tron = HDWallet::Tron(HDSeed::new(phrase));
        for i in 0..3 {
            let eth_i = hdw_eth.address(i as i32);
            let tron_i = hdw_tron.address(i as i32);
            let eth_priv = hdw_eth.private(i as i32);
            let tron_priv = hdw_tron.private(i as i32);
            let eth_pub = hdw_eth.public(i as i32);
            let tron_pub = hdw_tron.public(i as i32);
            println!("=======================");
            println!("ETH");
            println!("addr: {:?}", eth_i);
            println!("priv: {:?}", eth_priv);
            println!("pub: {:?}", eth_pub);
            println!("TRON");
            println!("addr: {:?}", tron_i);
            println!("priv: {:?}", tron_priv);
            println!("pub: {:?}", tron_pub);
            assert_eq!(eth_addrs[i], eth_i);
            assert_eq!(tron_addrs[i], tron_i);
            assert_eq!(eth_privs[i], eth_priv);
            assert_eq!(tron_privs[i], tron_priv);
            println!("=======================");
        }

    }
}
