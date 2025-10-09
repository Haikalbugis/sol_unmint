use solana_sdk::{
    signature::{Keypair, keypair_from_seed},
    signer::Signer,
};

pub enum WalletKey {
    StringKey(String),
    ArryKey(Vec<u8>),
}
pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn new(private_key: WalletKey) -> Self {
        match private_key {
            WalletKey::StringKey(key) => Self {
                keypair: Keypair::from_base58_string(&key),
            },
            WalletKey::ArryKey(items) => Self {
                keypair: keypair_from_seed(&items).unwrap(),
            },
        }
    }

    pub fn address(&self) -> String {
        self.keypair.pubkey().to_string()
    }
}
