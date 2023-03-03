use std::sync::Arc;

use ethers::{
    contract::ContractError,
    providers::Middleware,
    types::{Address, U256},
};
use futures::future::try_join;

use crate::abi::ierc20::IERC20;

#[derive(Debug)]
pub struct CachedERC20 {
    symbol: String,
    decimals: u8,
}

impl CachedERC20 {
    pub async fn new<M: Middleware>(
        address: impl Into<Address>,
        client: Arc<M>,
    ) -> Result<Self, ContractError<M>> {
        let token = IERC20::new(address, client);
        let (symbol, decimals) = try_join(token.symbol().call(), token.decimals().call()).await?;

        Ok(Self { symbol, decimals })
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    /// dummy float representation
    /// returns (integer_part, fractional_part)
    pub fn as_decimals(&self, x: U256) -> (U256, U256) {
        x.div_mod(10u128.pow(self.decimals() as u32).into())
    }
}
