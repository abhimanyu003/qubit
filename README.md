[![qubit](media/banner.png)](https://abhimanyu003.github.io/qubit/)

# qubit

<div align="center">

 **[💥 Visit Website To Use Calculator](https://abhimanyu003.github.io/qubit/)**

</div>

[![qubit](media/screenshot.png)](https://abhimanyu003.github.io/qubit/)

## Example

<div align="center">

 **[💥 Visit Website To Use Calculator](https://abhimanyu003.github.io/qubit/)**

</div>

```
2 + 2

sin( 90 ) + cos ( 120 )
sqrt(144) + 12
ceil ( 12.12 ) + 22
floor( 12.12) + 22

25 % of 100
25 % on 100

// Conversions
1024 kb to mb
22 kg to g
```

## Operations

* add 
* subtract
* multiply
* divide
* power
* modulus
* rightShift
* leftShift
* percentOf
* percentOn 
* sin
* cos
* tan
* sqrt
* cbrt
* round
* ceil
* floor

## Supported Conversions

* Angle
* Area
* Digital Information
* Length
* Mass
* Speed
* Time
* Temperature

# Development

### Stack qubit is using

* [Rust](https://www.rust-lang.org/) as programing language
* [Pest](https://pest.rs/) for parser + grammar
* [Yew](https://yew.rs/) for webassembly
* [Tailwind](https://tailwindcss.com/) for CSS styles

### Local Development + Enhancement

* Clone the repo
* Pest Grammar is defined in `src/grammar.pest` file.
* Conversion chart is `src/convert_chart.rs`

To start the project locally on `:8080` *run*

```
trunk serve
```

Before creating pull request you can run sanity checks.

```
cargo fmt
cargo check
cargo test
```

Final build ( Optional )

```
trunk build --release --public-url=qubit
```


# Contribution

This project welcomes your PR and issues.
For example, refactoring, adding features, correcting English, etc.
If you need any help, you can contact me on [Twitter](https://twitter.com/abhimanyu003).

Thanks to all the people who already contributed!

<a href="https://github.com/abhimanyu003/sttr/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=abhimanyu003/qubit" />
</a>

# License

[MIT](./LICENSE)
