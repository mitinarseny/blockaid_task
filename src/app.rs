use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use ethers::{
    contract::EthEvent,
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
    pub async fn run(&self, owner: Address, block_filter: FilterBlockOption) -> anyhow::Result<()> {
        eprintln!("getting approvals from {:#x}", owner);
        let approvals = ApprovalFilter::new(
            Filter::new().select(block_filter).topic1(H256::from(owner)),
            &self.client,
        )
        .query_with_meta()
        .await?;

        let token_addresses: HashSet<Address> =
            approvals.iter().map(|(_, m)| m.address).unique().collect();

        eprintln!(
            "got {} approvals from {} distinct tokens, resolving tokens...",
            approvals.len(),
            token_addresses.len()
        );

        let tokens: HashMap<Address, CachedERC20> = token_addresses
            .into_iter()
            .map(|address| {
                CachedERC20::new(address, self.client.clone()).map_ok(move |t| (address, t))
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect()
            .await?;

        println!("TOKEN\tAMOUNT\tSPENDER\t(tx TX_HASH)");
        for (a, m) in approvals.into_iter().rev() {
            let token = tokens.get(&m.address).unwrap(); // we already collected all tokens

            let (int_part, fract_part) = token.as_decimals(a.value);

            println!(
                "{}\t{}.{}\t{:#x}\t(tx {:#x})",
                token.symbol(),
                int_part,
                fract_part,
                a.spender,
                m.transaction_hash,
            );
        }
        Ok(())
    }
}
