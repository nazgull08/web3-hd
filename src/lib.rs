mod wallet;

mod tests {
    use ethkey::EthAccount;
    use log::info;

    #[test]
    fn test_wallet() {
        let a = EthAccount::load_or_generate("test", "test").unwrap();
        let addr = a.address();
        let pubkey = a.public();
        println!("==================");
        println!("addr {:?}",addr);
        println!("pubkye {:?}",pubkey);
        println!("acc {:?}",a);
        println!("==================");
        assert_eq!("test2","test2");
        assert_eq!("test1","test1");
    }
}
