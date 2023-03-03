pub(crate) mod abi;
mod erc20;

use std::{collections::HashMap, sync::Arc};

use ethers::{
    contract::{ContractError, EthEvent, LogMeta},
    providers::{Http, JsonRpcClient, Provider},
    types::{Address, Filter, FilterBlockOption, H256},
};
use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    TryFutureExt,
};
use itertools::Itertools;
use url::Url;

use crate::{abi::ierc20::ApprovalFilter, erc20::CachedERC20};

pub struct App<P: JsonRpcClient> {
    client: Arc<Provider<P>>,
}

impl App<Http> {
    pub fn new(node: impl Into<Url>) -> Self {
        Self {
            client: Provider::new(Http::new(node)).into(),
        }
    }
}

impl<P: JsonRpcClient + 'static> App<P> {
    async fn get_approvals_from(
        &self,
        owner: Address,
        block_filter: FilterBlockOption,
    ) -> Result<Vec<(ApprovalFilter, LogMeta)>, ContractError<Provider<P>>> {
        ApprovalFilter::new(
            Filter::new().select(block_filter).topic1(H256::from(owner)),
            &*self.client,
        )
        .query_with_meta()
        .await
    }

    async fn get_tokens(
        &self,
        tokens: impl IntoIterator<Item = Address>,
    ) -> Result<HashMap<Address, CachedERC20>, ContractError<Provider<P>>> {
        tokens
            .into_iter()
            .unique()
            .map(|address| {
                CachedERC20::new(address, self.client.clone()).map_ok(move |t| (address, t))
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect()
            .await
    }

    pub async fn get_approvals_and_tokens(
        &self,
        owner: Address,
        block_filter: FilterBlockOption,
    ) -> Result<
        (
            Vec<(ApprovalFilter, LogMeta)>,
            HashMap<Address, CachedERC20>,
        ),
        ContractError<Provider<P>>,
    > {
        let approvals = self.get_approvals_from(owner, block_filter).await?;

        let tokens = self
            .get_tokens(approvals.iter().map(|(_, m)| m.address))
            .await?;

        Ok((approvals, tokens))
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use std::str::FromStr;

    use ethers::types::U256;
    use serde::Serialize;
    use wasm_bindgen::prelude::*;

    /// Non-generic wrapper for App<P> to use with #[wasm_bindgen].
    /// It is needed since #[wasm_bindgen] does not support generics,
    /// and to implement conversions between Rust and JS types.
    #[wasm_bindgen]
    pub struct HTTPApp(App<Http>);

    #[wasm_bindgen]
    impl HTTPApp {
        pub fn new(node: &str) -> Result<HTTPApp, JsError> {
            Ok(Self(App::new(Url::parse(node)?)))
        }

        pub async fn get_approvals_and_tokens(
            &self,
            owner: &str,
            from_block: Option<u64>,
            to_block: Option<u64>,
        ) -> Result<JsValue, JsError> {
            #[derive(Serialize)]
            #[wasm_bindgen]
            struct Approval {
                owner: Address,
                spender: Address,
                value: U256,
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

            struct ApprovalWithMeta {
                approval: ApprovalFilter,
                meta: LogMeta,
            }

            let (approvals, tokens) = self
                .0
                .get_approvals_and_tokens(
                    Address::from_str(owner)?,
                    FilterBlockOption::Range {
                        from_block: from_block.map(Into::into),
                        to_block: to_block.map(Into::into),
                    },
                )
                .await?;

            serde_wasm_bindgen::to_value(&(
                approvals
                    .into_iter()
                    .map(|(a, m)| (Approval::from(a), m))
                    .collect::<Vec<_>>(),
                tokens,
            ))
            .map_err(Into::into)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
