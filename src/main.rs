
use std::convert::TryInto;
use std::path::Path;
use std::str::FromStr;

mod hash;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::base58;
use bitcoin::bip32::ExtendedPubKey;
use bitcoin::{
    network::constants::Network,
    bip32::{DerivationPath, ExtendedPrivKey},
    PublicKey,
};
use eth_keystore::encrypt_key;
use hash::keccak_hash;
use secp256k1::hashes::sha256;
use secp256k1::{Secp256k1};
use serde::{Deserialize, Serialize};
use ::sha256::digest;
use sha3::{Digest, Keccak256, Sha3_256};

/// How it works:
/// 1. generate a new mnemonic (tiny-bip39)
/// 2. mnemonic -> entropy -> keystore file (eth-keystore)
/// 3. mnemonic -> seed -> xpub -> pk, pubk -> eth addr (hdpath, bitcoin, secp25k1, sha3, eth_checksum)

pub const ETH_COINTYPE : i32 = 60;
pub const TRX_COINTYPE : i32 = 195;

fn main() {
    // ----------------------------------------------------------------------------- 1 mnemonic
    let mnemonic_eth= Mnemonic::from_phrase("candy defy gospel leisure kit vague arrive filter toast robot female document", Language::English).unwrap();
    let mnemonic = Mnemonic::from_phrase("debris resemble coil soul shrimp slender deal aunt twenty gown fee test", Language::English).unwrap();
    let mnemonic_new = Mnemonic::new(MnemonicType::Words12, Language::English);
    println!("Mnemonic: {}", mnemonic);

    // ----------------------------------------------------------------------------- 2 keystore
    // save it as a keystore file
    // my understanding is that you save the entropy, not the seed into keystore - https://support.mycrypto.com/general-knowledge/ethereum-blockchain/difference-between-wallet-types
    let entropy = mnemonic.entropy();
    println!("Entropy: {:?}", entropy); //128 bits for 12 words, 256 bits for 24 words

    let mut rng = rand::thread_rng();
    let dir = Path::new("./keys");
    let uuid = encrypt_key(&dir, &mut rng, entropy, "password_to_keystore").unwrap();
    println!("File uuid: {}", uuid);

    // ----------------------------------------------------------------------------- 3 derived addr
    // get the HD wallet seed
