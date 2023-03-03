# test assignment

Task: https://wobbly-nutmeg-8a5.notion.site/Approval-Detection-rust-1a61b33c073b4293a46138c01754689e

## Features and other notes

* Automatic Rust bindings generation from ERC20 JSON ABI (see ./build.rs)
* Metadata about tokens is collected from the node at runtime:
  token symbols and decimals
* The whole process took me ~12 hours:
  * ~3 hours on CLI app
  * ~3 hours on learning about WASM and conditional compilation
    for `x86_64` and `wasm32` architectures
  * ~3 hours on Rust <-> JS interaction, types conversion and etc.
  * ~3 hours on HTML + JS to make web UI.  
    I'm inexperienced with JS, so the whole web-UI part barely works
    and errors are only logged to console, but still...
* I looked at the bonus 2. It seems like a non-trivial task,
  which I'd be happy to dive in if I have more time
* I've done Aliyah, so I'm officially eligible for work in Israel now!  
  I am very excited about the opportunity and looking forward to hear from you soon.

## Dependencies

* Rust
* [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
* [npm](https://docs.npmjs.com/cli/v9/configuring-npm/install)

## bin

```sh
$ cargo buiild --release

$ ./target/release/my_approvals --help
Usage: my_approvals [OPTIONS] --node <URL> <OWNER>

Arguments:
  <OWNER>  Owner of tokens

Options:
  -n, --node <URL>                 HTTP ethereum node url
  -f, --from-block <BLOCK_NUMBER>  Starting block number to query from [default: earliest]
  -t, --to-block <BLOCK_NUMBER>    Ending block number to query from [default: latest]
  -h, --help                       Print help


$ ./target/release/my_approvals \
    --from-block 15096611 \
    --to-block 15950611 \
    --node https://rpc.flashbots.net \
    0x005e20fcf757b55d6e27dea9ba4f90c0b03ef852
getting approvals from 0x005e20fcf757b55d6e27dea9ba4f90c0b03ef852
got 4 approvals from 2 distinct tokens
TOKEN   AMOUNT  SPENDER (tx TX_HASH)
USDT    115792089237316195423570985008687907853269984665640564039457584007913129.639935 0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45       (tx 0xf3d8d7f12dc73863cd9714340f196156bc92125e61465cdfd4470f241c47a69b)
SKL     115792089237316195423570985008687907853269984665640563830357.584007913129639935 0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45       (tx 0x6289cbd837c03fc46fc5fbab5c68a32eac3b6d7418ae987c48b13144368e9c15)
SKL     115792089237316195423570985008687907853269984665640564019457.584007913129639935 0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45       (tx 0x5ca34b1777ec94fe919267b66492e2730d143a52750470c09642f9b0f3e854fd)
SKL     115792089237316195423570985008687907853269984665640564039457.584007913129639935 0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45       (tx 0x07a8b97798f16b854b7b5538550f0ddde27a0910c710714e16c1f51135e6bae8)
```

## WASM

```sh
$ wasm-pack build
$ cd ./www
$ npm start
```

Then go to [localhost:8080](http://localhost:8080)
