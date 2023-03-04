use std::{fmt::Display, sync::Arc};

use ethers::{
    contract::{ContractError, LogMeta},
    providers::Middleware,
    types::{Address, U256},
};
use futures::future::try_join;

#[cfg(target_arch = "wasm32")]
use serde::Serialize;

use crate::{
    abi::ierc20::{ApprovalFilter, IERC20},
    cached::CachedMap,
};

#[derive(Debug)]
#[cfg_attr(target_arch = "wasm32", derive(Serialize))]
pub struct CachedERC20 {
    address: Address,
    symbol: String,
    decimals: u8,
}

impl CachedERC20 {
    pub async fn new<M: Middleware>(
        address: impl Into<Address>,
        client: Arc<M>,
    ) -> Result<Self, ContractError<M>> {
        let address = address.into();
        let token = IERC20::new(address, client);
        let (symbol, decimals) = try_join(token.symbol().call(), token.decimals().call()).await?;

        Ok(Self {
            address,
            symbol,
            decimals,
        })
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn address(&self) -> Address {
        self.address
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

impl Display for CachedERC20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.symbol(), self.address())
    }
}

pub struct CachedTokens<M: Middleware> {
    client: Arc<M>,
    cached: CachedMap<Address, Arc<CachedERC20>>,
}

impl<M: Middleware> CachedTokens<M> {
    pub fn new(client: impl Into<Arc<M>>) -> Self {
        Self {
            client: client.into(),
            cached: Default::default(),
        }
    }

    pub async fn try_get_token(
        &self,
        address: Address,
    ) -> Result<Arc<CachedERC20>, ContractError<M>> {
        self.cached
            .get_or_try_insert_with(address, || CachedERC20::new(address, self.client.clone()))
            .await
    }
}

/// Custom Approval, since Serialize and wasm_bindgen are
/// not implemented on ApprovalFilter
#[derive(Debug)]
#[cfg_attr(target_arch = "wasm32", derive(Serialize))]
pub struct Approval {
    pub owner: Address,
    pub spender: Address,
    pub value: U256,
}

impl From<ApprovalFilter> for Approval {
    fn from(value: ApprovalFilter) -> Self {
        let ApprovalFilter {
            owner,
            spender,
            value,
        } = value;
        Self {
            owner,
            spender,
            value,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(target_arch = "wasm32", derive(Serialize))]
pub struct TokenApproval {
    pub token: Arc<CachedERC20>,
    pub approval: Approval,
    pub meta: LogMeta,
}

impl TokenApproval {
    pub fn new(
        token: impl Into<Arc<CachedERC20>>,
        approval: ApprovalFilter,
        meta: LogMeta,
    ) -> Self {
        Self {
            token: token.into(),
            approval: approval.into(),
            meta,
        }
    }
}

impl Display for TokenApproval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (int_part, frac_part) = self.token.as_decimals(self.approval.value);
        write!(
            f,
            "tx {}: approval on {} for {} on amount of {int_part}.{frac_part}",
            self.meta.transaction_hash, self.token, self.approval.spender,
        )
    }
}
