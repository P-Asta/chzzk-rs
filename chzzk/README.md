# chzzk
Unofficial SDK for the Korean video streaming platform [Chzzk](https://chzzk.naver.com/)

## Features
- `chzzk_debug`\
Made to help debug. Enabling it prints out some additional informations to `stdout`.

## Caution
- This SDK is currently in development. Do not consider anything in this crate stable.
- Every part of this crate relies on non-public APIs that are not documented in [Chzzk Developers](https://developers.chzzk.naver.com/). Many features depend on empirical observation. Some features remain unknown and may not be provided in this crate. Please let us know if you discover any information about currently unknown features.
  - Due to the lack of official documentation, this crate may cause runtime errors, especially when deserializing certain fields in responses from Chzzk. This usually happens when a user discovers that a field can be `null`, which we failed to observe before. You can fix this problem by changing the type of the struct field to [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html) or another appropriate type.
- The majority of this crate was written before the publication of the official API documentation, Chzzk Developers. We are currently working on supporting the official API as of version 0.4.

### Common Error
- `event_handler caught error: wrong bdy (not an attr). Object {"cmd": Number(10000), "ver": String("2")}`\
This is actually not an error. You can ignore it. You won't get this when `chzzk_debug` feature is disabled.
- `ChzzkError("chat.connect: get_user_status error. maybe wrong auth information", ...)`\
You are very likely passing incorrect or outdated authentication information to `ChzzkClient::sign_in()`.

## Running an example
```sh
git clone https://github.com/5-23/chzzk-rs
cd chzzk-rs
cargo r --example get_channel_info
```

Ensure you have an `examples/.env` file. This file is accessed by `examples/auth.rs` to retrieve authentication information for accessing some APIs. The file should look like this:
```
CHZZK_AUT=(64 characters long token)
CHZZK_SES=(520 characters long token)
```
You can find these values in your browser cookies after signing in to Chzzk.
