# Word game
![Static Badge](https://img.shields.io/badge/Rust-000?style=for-the-badge&logo=rust)
![Crates.io MSRV](https://img.shields.io/crates/msrv/iced?style=for-the-badge&logo=iced&label=iced&link=https%3A%2F%2Fdocs.iced.rs%2Ficed%2Findex.html)
***

## About
This a simple, based on guessing <b>word game</b>.  
It uses this free [dictionary api](https://api.msmc.cc/api/dictionary/random) for random words and shuffle letters in them.
If you guess the word right, next one will be given, but if you truly cannot, you can always use help, it will show you the original word.

Enjoy!

---

## Usage
>In debug it is not fetching words! Only word foobarbaz is given.  
> Enabled with argument `fetch`:
> ```bash
> cargo run -- fetch
> ```
> Or run release
> ```bash
> cargo run --release
> ```




