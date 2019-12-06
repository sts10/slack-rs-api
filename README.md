# slack-rs-api

[Slack][slack] Web API interface.

[![Build Status][ci-img]][ci-url] [![Crates.io][crates-img]][crates-url] [![License][license-img]][license-url]

This fork of [a fork](https://github.com/saethlin/slack-rs-api) is an attempt to fix [an issue in the existing crate](https://github.com/slack-rs/slack-rs-api/issues/78) having to do with the timestamps of certain messages being returned from the API as a string and not an `f32` type (or a string that cannot be parsed into an `f32`, I'm honestly not sure). 

I didn't write the code that attempts to fix it. 

The error, in part, looks something like this:

```text
Err(MalformedResponse(Error("invalid type: string \"1575650231.403900\", expected f32", line: 1, column: 8089)))
```

There is [a pull request](https://github.com/slack-rs/slack-rs-api/issues/78), but it hasn't been merged. 

There's also [another fork](https://github.com/clux/slack-rs-api/commit/673dc064405bcc74929cae10ce571c6254a798a8) that attempts to fix this issue (or a similar one), albeit with what looks like a blunter approach. 

[Documentation][docs] (may be outdated)

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
slack_api = { git = "https://github.com/sts10/slack-rs-api" }
```

and this to your crate root:

```rust
extern crate slack_api;
```

## License
`slack-api` is distributed under the [Apache-2.0 License](./LICENSE).

[docs]: https://docs.rs/slack_api
[ci-img]: https://travis-ci.org/slack-rs/slack-rs-api.svg?branch=master
[ci-url]: https://travis-ci.org/slack-rs/slack-rs-api
[crates-img]: https://img.shields.io/crates/v/slack_api.svg
[crates-url]: https://crates.io/crates/slack_api
[license-img]: https://img.shields.io/github/license/mthjones/slack-rs-api.svg
[license-url]: https://raw.githubusercontent.com/mthjones/slack-rs-api/master/LICENSE
[slack]: https://api.slack.com/
