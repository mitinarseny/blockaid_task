# test assignment

Task: https://wobbly-nutmeg-8a5.notion.site/Approval-Detection-rust-1a61b33c073b4293a46138c01754689e

## `transfer` vs `transferFrom`
Both calls `transfer` and `transferFrom` are part of IERC20 interface:
* `transfer(address to,uint256 amount)` transfers `amount` of tokens from `msg.sender` to `to`
* `transferFrom(address owner,address to,uint256 amount)` transfers `amount` of tokens
  from `owner` to `to`, while `msg.sender` has allowance to transfer at least `amount`
  from `msg.sender`. To set an allowance, `owner` should previously called
  `approve(spender, amount)`, where `spender` is `msg.sender` of the transaction, which
  calls `transferFrom` in turn.

## Features and other notes

* Automatic Rust bindings generation from ERC20 JSON ABI (see [./build.rs](./build.rs))
* Metadata about tokens (symbols and decimals) is collected from the node and cached at runtime.
  So that every token metadata is requested only once through the whole lifetime of `App` object.
  This is especially useful when refresing list of approvals in the web-UI.
  As a downside, this WASM module, for instance, would consume more and more memory
  every time new tokens are resolved. This can be fixed by introducing an upper limit
  for cache size.
* The whole process took me ~14 hours:
  * ~3 hours on CLI app
  * ~3 hours on learning about WASM and conditional compilation
    for `x86_64` and `wasm32` architectures
  * ~3 hours on Rust <-> JS interaction, types conversion and etc.
  * ~3 hours on HTML + JS to make web UI.  
    I'm inexperienced with JS, so the whole web-UI part barely works
    and errors are only logged to console, but still...
  * ~2 hours on implementing token metadata caching and refactoring
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
got 4 approvals
tx 0xf3d8…a69b: approval on USDT (0xdac1…1ec7) for 0x68b3…fc45 on amount of 115792089237316195423570985008687907853269984665640564039457584007913129.639935
tx 0x6289…9c15: approval on SKL (0x00c8…a7a7) for 0x68b3…fc45 on amount of 115792089237316195423570985008687907853269984665640563830357.584007913129639935
tx 0x5ca3…54fd: approval on SKL (0x00c8…a7a7) for 0x68b3…fc45 on amount of 115792089237316195423570985008687907853269984665640564019457.584007913129639935
tx 0x07a8…bae8: approval on SKL (0x00c8…a7a7) for 0x68b3…fc45 on amount of 115792089237316195423570985008687907853269984665640564039457.584007913129639935
```

## WASM

```sh
$ wasm-pack build
$ cd ./www
$ npm start
```

Then go to [localhost:8080](http://localhost:8080)
![image](https://user-images.githubusercontent.com/10659110/222786309-225fd802-6345-4c21-bf9c-998277afd64f.png)

