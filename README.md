# Combinatory

[![crates.io](https://img.shields.io/crates/v/combinatory.svg)](https://crates.io/crates/combinatory)
[![docs.rs](https://img.shields.io/docsrs/combinatory)](https://docs.rs/combinatory/latest/combinatory/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JorgeRicoVivas/combinatory/rust.yml)](https://github.com/JorgeRicoVivas/combinatory/actions)
[![GitHub last commit](https://img.shields.io/github/last-commit/JorgeRicoVivas/combinatory)](https://github.com/JorgeRicoVivas/combinatory)
[![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/combinatory)](https://github.com/JorgeRicoVivas/combinatory?tab=CC0-1.0-1-ov-file)

> *You are reading the documentation for combinatory version 1.0.0*

Allows to combinate values of two dimensional iterators / collections.

For example, the combinations of each value `[1, 2]` with the values `[a, b]` are:

- (1,a).
- (1,b).
- (2,a).
- (2,b).

The following code shows every combination of a menu-deal composed of a main, a side, and a
dessert, having two options for each:

```rust
use combinatory::Combinatory;

const SIDES: &[&'static str] = &["Soup", "Bread and butter"];
const MAIN: &[&'static str] = &["Chicken", "Burger with fries"];
const DESSERTS: &[&'static str] = &["Vanilla ice cream", "Souffle"];

let possible_menus = Combinatory::new([SIDES, MAIN, DESSERTS])
    .ref_combinations()
    .enumerate()
    .map(|(menu_index, possible_menu)| {
        format!("{} - {}", menu_index + 1, possible_menu
            .map(|string|string.to_string()).collect::<Vec<_>>().join(", "))
    })
    .collect::<Vec<_>>()
    .join("\n");

println!("{possible_menus}");

assert_eq!(possible_menus,
"1 - Soup, Chicken, Vanilla ice cream
2 - Soup, Chicken, Souffle
3 - Soup, Burger with fries, Vanilla ice cream
4 - Soup, Burger with fries, Souffle
5 - Bread and butter, Chicken, Vanilla ice cream
6 - Bread and butter, Chicken, Souffle
7 - Bread and butter, Burger with fries, Vanilla ice cream
8 - Bread and butter, Burger with fries, Souffle");
```

This code will print:
```html
1 - Soup, Chicken, Vanilla ice cream
2 - Bread and butter, Chicken, Vanilla ice cream
3 - Soup, Fillet with fries, Vanilla ice cream
4 - Bread and butter, Fillet with fries, Vanilla ice cream
5 - Soup, Chicken, Souffle
6 - Bread and butter, Chicken, Souffle
7 - Soup, Fillet with fries, Souffle
8 - Bread and butter, Fillet with fries, Souffle
```