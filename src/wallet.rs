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

pub struct Generate {
    pub arry_private_key: [u8; 64],
    pub base64_private_key: String,
    pub address: String,
}

//generate new wallet
pub fn generate() -> Generate {
    let kp = Keypair::new();

    let arry_private_key: [u8; 64] = kp.to_bytes();
    let base64_private_key = kp.to_base58_string();
    let address = kp.pubkey();

    Generate {
        arry_private_key,
        base64_private_key,
        address: address.to_string(),
    }
}
