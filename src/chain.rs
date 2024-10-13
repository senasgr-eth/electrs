use std::str::FromStr;

#[cfg(not(feature = "liquid"))] // use regular Bitcoin data structures
pub use bitcoin::{
    blockdata::{opcodes, script, witness::Witness},
    consensus::deserialize,
    hashes,
    util::address,
    Block, BlockHash, BlockHeader, OutPoint, Script, Transaction, TxIn, TxOut, Txid,
};

#[cfg(feature = "liquid")]
pub use {
    crate::elements::asset,
    elements::{
        address, confidential, encode::deserialize, hashes, opcodes, script, Address, AssetId,
        Block, BlockHash, BlockHeader, OutPoint, Script, Transaction, TxIn, TxInWitness as Witness,
        TxOut, Txid,
    },
};

use bitcoin::blockdata::constants::genesis_block;
pub use bitcoin::network::constants::Network as BNetwork;

#[cfg(not(feature = "liquid"))]
pub type Value = u64;
#[cfg(feature = "liquid")]
pub use confidential::Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    #[cfg(not(feature = "liquid"))]
    Bitcoin,
    #[cfg(not(feature = "liquid"))]
    Testnet,
    #[cfg(not(feature = "liquid"))]
    Testnet4,
    #[cfg(not(feature = "liquid"))]
    Regtest,
    #[cfg(not(feature = "liquid"))]
    Signet,

    Bellcoin,        // Added Bellcoin mainnet
    BellcoinTestnet, // Added Bellcoin testnet

    #[cfg(feature = "liquid")]
    Liquid,
    #[cfg(feature = "liquid")]
    LiquidTestnet,
    #[cfg(feature = "liquid")]
    LiquidRegtest,
}

#[cfg(feature = "liquid")]
pub const LIQUID_TESTNET_PARAMS: address::AddressParams = address::AddressParams {
    p2pkh_prefix: 36,
    p2sh_prefix: 19,
    blinded_prefix: 23,
    bech_hrp: "tex",
    blech_hrp: "tlq",
};

impl Network {
    #[cfg(not(feature = "liquid"))]
    pub fn magic(self) -> u32 {
        match self {
            Network::Bitcoin => BNetwork::Bitcoin.magic(),
            Network::Testnet => BNetwork::Testnet.magic(),
            Network::Testnet4 => BNetwork::Testnet.magic(),
            Network::Regtest => BNetwork::Regtest.magic(),
            Network::Signet => BNetwork::Signet.magic(),
            Network::Bellcoin => 0xD9B4BEF9,        // Magic number for Bellcoin (adjust accordingly)
            Network::BellcoinTestnet => 0x0709110B, // Magic number for Bellcoin testnet (adjust accordingly)
        }
    }

    pub fn is_regtest(self) -> bool {
        match self {
            Network::Regtest | Network::BellcoinTestnet => true, // Bellcoin Testnet treated as regtest-like network
            _ => false,
        }
    }

    pub fn address_params(self) -> &'static address::AddressParams {
        match self {
            Network::Bitcoin => &address::AddressParams::BITCOIN,
            Network::Testnet => &address::AddressParams::TESTNET,
            Network::Bellcoin => &address::AddressParams {
                p2pkh_prefix: 25,        // Bellcoin p2pkh prefix
                p2sh_prefix: 5,          // Bellcoin p2sh prefix
                blinded_prefix: 0,       // No blinded addresses in Bellcoin
                bech_hrp: "bel",         // Bellcoin SegWit HRP
                blech_hrp: "bel1p",      // Bellcoin Taproot HRP
            },
            _ => panic!("Unsupported network"),
        }
    }

    pub fn names() -> Vec<String> {
        vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
            "signet".to_string(),
            "bellcoin".to_string(),
            "bellcointestnet".to_string(),
        ]
    }
}

pub fn genesis_hash(network: Network) -> BlockHash {
    lazy_static! {
        static ref BITCOIN_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Bitcoin).block_hash();
        static ref TESTNET_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Testnet).block_hash();
        static ref BELLCOIN_GENESIS: bitcoin::BlockHash =
            bitcoin::BlockHash::from_str("e5be24df57c43a82d15c2f06bda961296948f8f8eb48501bed1efb929afe0698")
            .unwrap(); // Bellcoin genesis block hash
    }

    match network {
        Network::Bitcoin => *BITCOIN_GENESIS,
        Network::Testnet => *TESTNET_GENESIS,
        Network::Bellcoin => *BELLCOIN_GENESIS,
        _ => panic!("Unsupported network"),
    }
}

impl From<&str> for Network {
    fn from(network_name: &str) -> Self {
        match network_name {
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "regtest" => Network::Regtest,
            "signet" => Network::Signet,
            "bellcoin" => Network::Bellcoin,
            "bellcointestnet" => Network::BellcoinTestnet,
            _ => panic!("unsupported Bitcoin network: {:?}", network_name),
        }
    }
}

#[cfg(not(feature = "liquid"))]
impl From<Network> for BNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BNetwork::Bitcoin,
            Network::Testnet => BNetwork::Testnet,
            Network::Regtest => BNetwork::Regtest,
            Network::Signet => BNetwork::Signet,
            _ => panic!("unsupported Bitcoin network"),
        }
    }
}

#[cfg(not(feature = "liquid"))]
impl From<BNetwork> for Network {
    fn from(network: BNetwork) -> Self {
        match network {
            BNetwork::Bitcoin => Network::Bitcoin,
            BNetwork::Testnet => Network::Testnet,
            BNetwork::Regtest => Network::Regtest,
            BNetwork::Signet => Network::Signet,
        }
    }
}
