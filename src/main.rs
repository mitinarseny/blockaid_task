use clap::{Parser, ValueHint};
use ethers::types::{Address, BlockNumber, FilterBlockOption};
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
    #[arg(short, long, value_name = "BLOCK_NUMBER")]
    from_block: Option<u64>,

    /// Ending block number to query from [default: latest]
    #[arg(short, long, value_name = "BLOCK_NUMBER")]
    to_block: Option<u64>,

    /// Owner of tokens
    #[arg()]
    owner: Address,
}

#[main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app = App::new(args.node);

    eprintln!("getting approvals from {:#x}", &args.owner);
    let (approvals, tokens) = app
        .get_approvals_and_tokens(
            args.owner,
            FilterBlockOption::Range {
                from_block: Some(
                    args.from_block
                        .map(Into::into)
                        .unwrap_or(BlockNumber::Earliest),
                ),
                to_block: Some(args.to_block.map(Into::into).unwrap_or(BlockNumber::Latest)),
            },
        )
        .await?;

    eprintln!(
        "got {} approvals from {} distinct tokens",
        approvals.len(),
        tokens.len()
    );

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
