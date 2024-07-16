use alloy_chains::{Chain, ChainKind, NamedChain};
use alloy_primitives::Address;

pub mod mainnet;

pub enum AddressBook {
    Weth,
    Wbtc,
    Dai,
    Usdt,
    Usdc,
    Frax,
    Usdd,
    Susd,
    Lusd,
    Mai,
    Metis,
    Sprm,
    UniswapV2Router,
    UniswapV2Factory,
    WethUsdcUniV2Pool,
    WethUsdtUniV2Pool,
    UniswapV3Router,
    UniswapV3Factory,
}

impl AddressBook {
    pub fn on(&self, chain: impl Into<Chain>) -> Address {
        match chain.into().into_kind() {
            ChainKind::Named(named) => match named {
                NamedChain::Mainnet => self.mainnet(),
                _ => {
                    panic!("Unsupported chain")
                }
            },
            ChainKind::Id(id) => match id {
                1 => self.mainnet(),
                _ => {
                    panic!("Unsupported chain")
                }
            }
        }
    }

    fn mainnet(&self) -> Address {
        use AddressBook::*;
        match self {
            Weth => mainnet::WETH,
            Wbtc => mainnet::WBTC,
            Dai => mainnet::DAI,
            Usdt => mainnet::USDT,
            Usdc => mainnet::USDC,
            Frax => mainnet::FRAX,
            Usdd => mainnet::USDD,
            Susd => mainnet::SUSD,
            Lusd => mainnet::LUSD,
            Mai => mainnet::MAI,
            Metis => mainnet::METIS,
            Sprm => mainnet::SPRM,
            UniswapV2Router => mainnet::UNISWAP_V2_ROUTER,
            UniswapV2Factory => mainnet::UNISWAP_V2_FACTORY,
            WethUsdcUniV2Pool => mainnet::WETH_USDC_UNI_V2,
            WethUsdtUniV2Pool => mainnet::WETH_USDT_UNI_V2,
            UniswapV3Router => mainnet::UNISWAP_V3_ROUTER,
            UniswapV3Factory => mainnet::UNISWAP_V3_FACTORY,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloy_chains::NamedChain;

    #[test]
    fn test_address_book() {
        let address_book = AddressBook::Weth.on(NamedChain::Mainnet);
        assert_eq!(address_book, mainnet::WETH);
    }
}