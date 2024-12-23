use ergo_lib::{ergotree_ir::chain::address::{AddressEncoder, NetworkPrefix}, wallet::{mnemonic_generator::{Language, MnemonicGenerator}, Wallet}};

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn get_balance(address: String) -> Balance {
    let address_enc = AddressEncoder::new(NetworkPrefix::Mainnet);
    let address_value = address_enc.parse_address_from_str(address.as_str()).expect("failed ergo_lib");
    let script = address_value.script().expect("failed ergo_lib");

    Balance {
        balance_in_nano: address.len() as u64,
        address: script.debug_tree()
    }
}

#[derive(uniffi::Record)]
pub struct Balance {
    pub balance_in_nano: u64,
    pub address: String
}

#[uniffi::export]
pub fn get_wallet() -> WalletWrapper {
    let generator = MnemonicGenerator::new(Language::English, 128);
    let mnemonic_phrase = generator.generate().expect("failed generate mnemonic");
    let _ = Wallet::from_mnemonic(&mnemonic_phrase, "").expect("failed load wallet");

    WalletWrapper { 
        mnemonic_phrase
    }
}

#[derive(uniffi::Record)]
pub struct WalletWrapper {
    mnemonic_phrase: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let balance = get_balance("9grWzUzkzQaqizwa2Jr2gkLq9H6NxHJ6THzoKfu1L1sKs4tvYyn".to_string());
        let result = balance.balance_in_nano;
        assert_eq!(result, 51);
        println!("{}", balance.address);
    }
}
