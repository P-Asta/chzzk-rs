# chzzk
Unofficial SDK for the Korean video streaming platform [Chzzk](https://chzzk.naver.com/)

## Caution
- This SDK is currently in development. Do not consider anything in this crate stable.
- Every part of this crate relies on non-public APIs that are not documented in [Chzzk Developers](https://developers.chzzk.naver.com/). Many features depend on empirical observation. Some features remain unknown and may not be provided in this crate. Please let us know if you discover any information about currently unknown features.
  - Due to the lack of official documentation, this crate may cause runtime errors, especially when deserializing certain fields in responses from Chzzk. This usually happens when a user discovers that a field can be `null`, which we failed to observe before. You can fix this problem by changing the type of the struct field to [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html) or another appropriate type.
- The majority of this crate was written before the publication of the official API documentation, Chzzk Developers. We are currently working on supporting the official API as of version 0.3.
