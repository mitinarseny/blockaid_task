pub(crate) mod abi;
mod cached;
mod erc20;

use std::sync::Arc;

use ethers::{
    contract::{ContractError, EthEvent, LogMeta},
    providers::{Http, JsonRpcClient, Provider},
    types::{Address, Filter, FilterBlockOption, H256},
};
use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    TryFutureExt,
};
use url::Url;

use self::{
    abi::ierc20::ApprovalFilter,
    erc20::{CachedTokens, TokenApproval},
};

pub struct App<P: JsonRpcClient> {
    client: Arc<Provider<P>>,
    tokens: CachedTokens<Provider<P>>,
}

impl App<Http> {
    pub fn new(node: impl Into<Url>) -> Self {
        let client = Arc::new(Provider::new(Http::new(node)));
        Self {
            tokens: CachedTokens::new(client.clone()),
            client,
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

    pub async fn get_token_approvals(
        &self,
        owner: Address,
        block_filter: FilterBlockOption,
    ) -> Result<Vec<TokenApproval>, ContractError<Provider<P>>> {
        self.get_approvals_from(owner, block_filter)
            .await?
            .into_iter()
            .map(|(approval, meta)| {
                self.tokens
                    .try_get_token(meta.address)
                    .map_ok(move |token| TokenApproval::new(token, approval, meta))
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect()
            .await
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use std::str::FromStr;

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

        pub async fn get_token_approvals(
            &self,
            owner: &str,
            from_block: Option<u64>,
            to_block: Option<u64>,
        ) -> Result<JsValue, JsError> {
            serde_wasm_bindgen::to_value(
                &self
                    .0
                    .get_token_approvals(
                        Address::from_str(owner)?,
                        FilterBlockOption::Range {
                            from_block: from_block.map(Into::into),
                            to_block: to_block.map(Into::into),
                        },
                    )
                    .await?,
            )
            .map_err(Into::into)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
