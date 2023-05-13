use std::str::FromStr;

use log::info;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::base58;
use bitcoin::bip32::ExtendedPubKey;
use bitcoin::{
    network::constants::Network,
    bip32::{DerivationPath, ExtendedPrivKey},
    PublicKey,
};
use eth_keystore::encrypt_key;
use secp256k1::{Secp256k1};
use serde::{Deserialize, Serialize};
use ::sha256::digest;

use crate::hash::keccak_hash;

pub enum HDWallet {
    Ethereum(HDSeed),
    Tron(HDSeed),
}

pub struct HDSeed {
    pub mnemonic: Mnemonic,
}
//NTD add funcion for save key to file
impl HDWallet {
    pub fn address(&self,index: i32) -> String{ 
        match self{
            HDWallet::Ethereum(seed) => eth_address_by_index(seed,index),
            HDWallet::Tron(seed) => tron_address_by_index(seed,index),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthAddr(String);

impl EthAddr {
    pub fn new(addr: &str) -> Self {
        let mut proper_addr = addr.to_owned();
        //check for 0x prefix
        if !addr.starts_with("0x") {
            proper_addr = format!("0x{}", addr);
        }
        //check that passed str is a hex string
        hex::decode(&proper_addr[2..])
            .map_err(|e| {
                println!("String passed into EthAddr is not hex.");
                e
            })
            .unwrap();
        //check length
        if proper_addr.len() != 42 {
            panic!(
                "String passed into EthAddr is {} hex chars long instead of 42.",
                proper_addr.len()
            );
        }
        //checksum and return
        let checksummed_addr = eth_checksum::checksum(&proper_addr);
        println!("New eth addr: {}", checksummed_addr);
        Self(checksummed_addr)
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}

fn eth_address_by_index(seed: &HDSeed, index : i32) -> String{
    let hd_path_str = format!("m/44'/60'/0'/0/{index}");
    let seed_m = Seed::new(&seed.mnemonic,"");
    let seed_bytes = seed_m.as_bytes();
    let (_pk, pubk) = get_extended_keypair(&seed_bytes.clone(), &DerivationPath::from_str(&hd_path_str).unwrap());
    let eth_addr = extended_pubk_to_addr(&pubk);
    format!("{:?}",eth_addr)
}

fn tron_address_by_index(seed: &HDSeed, index : i32) -> String{
    let hd_path_str = format!("m/44'/195'/0'/0/{index}");
    let seed_m = Seed::new(&seed.mnemonic,"");
    let seed_bytes = seed_m.as_bytes();
    let (_pk, pubk) = get_extended_keypair(&seed_bytes, &DerivationPath::from_str(&hd_path_str).unwrap());
    let tron_addr= extended_pubk_to_addr_tron(&pubk);
    tron_addr 
}



fn get_extended_keypair(
    seed: &[u8],
    hd_path: &DerivationPath
) -> (ExtendedPrivKey, ExtendedPubKey) {
    let secp = Secp256k1::new();
    let pk = ExtendedPrivKey::new_master(Network::Bitcoin, seed)
        // we convert HD Path to bitcoin lib format (DerivationPath)
        .and_then(|k| k.derive_priv(&secp, hd_path))
        .unwrap();
    let pubk = ExtendedPubKey::from_priv(&secp, &pk);

    let a = pk;
    let b:[u8; 32] = a.private_key.secret_bytes();
    let pub_b = pubk.public_key.serialize_uncompressed();

    (pk, pubk)
}

fn extended_pubk_to_addr(pubk: &ExtendedPubKey) -> EthAddr {
    //massage into the right format
    let pubk_str = pubk.public_key.to_string();
    let pubk_secp = secp256k1::PublicKey::from_str(&pubk_str).unwrap();
    //format as uncompressed key, remove "04" in the beginning
    let pubk_uncomp = &PublicKey::new_uncompressed(pubk_secp).to_string()[2..];
    //decode from hex and pass to keccak for hashing
    let pubk_bytes = hex::decode(pubk_uncomp).unwrap();
    let addr = &keccak_hash(&pubk_bytes);
    //keep last 20 bytes of the result
    let addr = &addr[(addr.len() - 40)..];
    //massage into domain unit
    EthAddr::new(addr)
}

fn extended_pubk_to_addr_tron(pubk: &ExtendedPubKey) -> String {
    //massage into the right format
    let pubk_str = pubk.public_key.to_string();
    let pubk_secp = secp256k1::PublicKey::from_str(&pubk_str).unwrap();
    //format as uncompressed key, remove "04" in the beginning
    let pubk_uncomp = &PublicKey::new_uncompressed(pubk_secp).to_string()[2..];
    //decode from hex and pass to keccak for hashing
    let pubk_bytes = hex::decode(pubk_uncomp).unwrap();
    let k_addr = &keccak_hash(&pubk_bytes);
    //keep last 20 bytes of the result
    let experimental_addr = "41".to_owned()+&k_addr[24..];
    let hex_exp_addr = hex::decode(&experimental_addr).unwrap(); 
    let s_hex_exp_addr = hex_exp_addr.as_slice();
    //let exp_k_addr_b = hex::de(&experimental_addr).unwrap();
    let val0 = digest(s_hex_exp_addr);
    let hex_val0 = hex::decode(&val0).unwrap(); 
    let s_hex_val0 = hex_val0.as_slice();
    let val1 = digest(s_hex_val0); 
    let check_sum_val1 = &val1[0..8];
    let final_addr = experimental_addr + check_sum_val1;
    let final_addr_bytes = hex::decode(final_addr).unwrap();

    let b58_res = base58::encode(&final_addr_bytes);
    //massage into domain unit
    b58_res
}


pub fn tron_pub_key_from_priv_key_bytes(priv_key_bytes : [u8;32]) -> [u8;32]{
   priv_key_bytes 
}



