use clap::{Parser, ValueHint};
use ethers::types::{Address, BlockNumber};
use tokio::main;
use url::Url;

use my_approvals::App;

#[derive(Parser)]
struct Args {
    /// HTTP ethereum node url
    #[arg(
        short, long,
        value_hint = ValueHint::Url,
        value_name = "URL",
    )]
    node: Url,

    /// Starting block number to query from [default: earliest]
    #[arg(
        short, long,
        value_name = "BLOCK_NUMBER",
    )]
    from_block: Option<u64>,

    /// Ending block number to query from [default: latest]
    #[arg(
        short, long,
        value_name = "BLOCK_NUMBER",
    )]
    to_block: Option<u64>,

    /// Owner of tokens
    #[arg()]
    owner: Address,
}

#[main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app = App::new(args.node);

    app.run(
        args.owner,
        ethers::types::FilterBlockOption::Range {
            from_block: Some(
                args.from_block
                    .map(Into::into)
                    .unwrap_or(BlockNumber::Earliest),
            ),
            to_block: Some(args.to_block.map(Into::into).unwrap_or(BlockNumber::Latest)),
        },
    )
    .await
}
