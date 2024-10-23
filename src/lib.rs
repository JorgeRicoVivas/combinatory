#![cfg_attr(not(test), no_std)]

//! # Combinatory
//!
//! [![crates.io](https://img.shields.io/crates/v/combinatory.svg)](https://crates.io/crates/combinatory)
//! [![docs.rs](https://img.shields.io/docsrs/combinatory)](https://docs.rs/combinatory/latest/combinatory/)
//! [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JorgeRicoVivas/combinatory/rust.yml)](https://github.com/JorgeRicoVivas/combinatory/actions)
//! [![GitHub last commit](https://img.shields.io/github/last-commit/JorgeRicoVivas/combinatory)](https://github.com/JorgeRicoVivas/combinatory)
//! [![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/combinatory)](https://github.com/JorgeRicoVivas/combinatory?tab=CC0-1.0-1-ov-file)
//!
//! > *You are reading the documentation for combinatory version 1.0.0*
//!
//! Allows to combinate values of two dimensional iterators / collections.
//!
//! For example, the combinations of each value `[1, 2]` with the values `[a, b]` are:
//!
//! - (1,a).
//! - (1,b).
//! - (2,a).
//! - (2,b).
//!
//! The following code shows every combination of a menu-deal composed of a main, a side, and a
//! dessert, having two options for each:
//!
//! ```rust
//! use combinatory::Combinatory;
//!
//! const SIDES: &[&'static str] = &["Soup", "Bread and butter"];
//! const MAIN: &[&'static str] = &["Chicken", "Burger with fries"];
//! const DESSERTS: &[&'static str] = &["Vanilla ice cream", "Souffle"];
//!
//! let possible_menus = Combinatory::new([SIDES, MAIN, DESSERTS])
//!     .ref_combinations()
//!     .enumerate()
//!     .map(|(menu_index, possible_menu)| {
//!         format!("{} - {}", menu_index + 1, possible_menu
//!             .map(|string|string.to_string()).collect::<Vec<_>>().join(", "))
//!     })
//!     .collect::<Vec<_>>()
//!     .join("\n");
//!
//! println!("{possible_menus}");
//!
//! assert_eq!(possible_menus,
//! "1 - Soup, Chicken, Vanilla ice cream
//! 2 - Soup, Chicken, Souffle
//! 3 - Soup, Burger with fries, Vanilla ice cream
//! 4 - Soup, Burger with fries, Souffle
//! 5 - Bread and butter, Chicken, Vanilla ice cream
//! 6 - Bread and butter, Chicken, Souffle
//! 7 - Bread and butter, Burger with fries, Vanilla ice cream
//! 8 - Bread and butter, Burger with fries, Souffle");
//! ```
//!
//! This code will print:
//! ```html
//! 1 - Soup, Chicken, Vanilla ice cream
//! 2 - Bread and butter, Chicken, Vanilla ice cream
//! 3 - Soup, Fillet with fries, Vanilla ice cream
//! 4 - Bread and butter, Fillet with fries, Vanilla ice cream
//! 5 - Soup, Chicken, Souffle
//! 6 - Bread and butter, Chicken, Souffle
//! 7 - Soup, Fillet with fries, Souffle
//! 8 - Bread and butter, Fillet with fries, Souffle
//! ```

extern crate alloc;

use alloc::vec::{IntoIter, Vec};


/// Allows to combinate values of two dimensional iterators / collections.
///
/// For example, the combinations of each value `[1, 2]` with the values `[a, b]` are:
///
/// - (1,a).
/// - (1,b).
/// - (2,a).
/// - (2,b).
///
/// The following code shows every combination of a menu-deal composed of a main, a side, and a
/// dessert, having two options for each:
///
/// ```rust
/// use combinatory::Combinatory;
///
/// const SIDES: &[&'static str] = &["Soup", "Bread and butter"];
/// const MAIN: &[&'static str] = &["Chicken", "Burger with fries"];
/// const DESSERTS: &[&'static str] = &["Vanilla ice cream", "Souffle"];
///
/// let possible_menus = Combinatory::new([SIDES, MAIN, DESSERTS])
///     .ref_combinations()
///     .enumerate()
///     .map(|(menu_index, possible_menu)| {
///         format!("{} - {}", menu_index + 1, possible_menu
///             .map(|string|string.to_string()).collect::<Vec<_>>().join(", "))
///     })
///     .collect::<Vec<_>>()
///     .join("\n");
///
/// println!("{possible_menus}");
///
/// assert_eq!(possible_menus,
/// "1 - Soup, Chicken, Vanilla ice cream
/// 2 - Soup, Chicken, Souffle
/// 3 - Soup, Burger with fries, Vanilla ice cream
/// 4 - Soup, Burger with fries, Souffle
/// 5 - Bread and butter, Chicken, Vanilla ice cream
/// 6 - Bread and butter, Chicken, Souffle
/// 7 - Bread and butter, Burger with fries, Vanilla ice cream
/// 8 - Bread and butter, Burger with fries, Souffle");
/// ```
///
/// This code will print:
/// ```html
/// 1 - Soup, Chicken, Vanilla ice cream
/// 2 - Bread and butter, Chicken, Vanilla ice cream
/// 3 - Soup, Fillet with fries, Vanilla ice cream
/// 4 - Bread and butter, Fillet with fries, Vanilla ice cream
/// 5 - Soup, Chicken, Souffle
/// 6 - Bread and butter, Chicken, Souffle
/// 7 - Soup, Fillet with fries, Souffle
/// 8 - Bread and butter, Fillet with fries, Souffle
/// ```
#[derive(Debug)]
pub struct Combinatory<T> {
    values: Vec<Vec<T>>,
    current_combination_forwards: usize,
    current_combination_backwards: usize,
    forward_modules: Vec<usize>,
    backward_modules: Vec<usize>,
    direction_is_forward: bool,
    combinations_len: usize,
}


impl<T> Combinatory<T> {
    /// Creates a new Combinatory out of this two-dimensional iterator, where the first dimension
    /// contains collections of the elements to combine.
    ///
    /// For example, the combinations of each value `[1, 2]` with the values `[a, b]` are:
    ///
    /// - (1,a).
    /// - (1,b).
    /// - (2,a).
    /// - (2,b).
    ///
    /// The following code shows every combination of a menu-deal composed of a main, a side, and a
    /// dessert, having two options for each:
    ///
    /// ```rust
    /// use combinatory::Combinatory;
    ///
    /// const SIDES: &[&'static str] = &["Soup", "Bread and butter"];
    /// const MAIN: &[&'static str] = &["Chicken", "Burger with fries"];
    /// const DESSERTS: &[&'static str] = &["Vanilla ice cream", "Souffle"];
    ///
    /// let possible_menus = Combinatory::new([SIDES, MAIN, DESSERTS])
    ///     .ref_combinations()
    ///     .enumerate()
    ///     .map(|(menu_index, possible_menu)| {
    ///         format!("{} - {}", menu_index + 1, possible_menu
    ///             .map(|string|string.to_string()).collect::<Vec<_>>().join(", "))
    ///     })
    ///     .collect::<Vec<_>>()
    ///     .join("\n");
    ///
    /// println!("{possible_menus}");
    ///
    /// assert_eq!(possible_menus,
    /// "1 - Soup, Chicken, Vanilla ice cream
    /// 2 - Soup, Chicken, Souffle
    /// 3 - Soup, Burger with fries, Vanilla ice cream
    /// 4 - Soup, Burger with fries, Souffle
    /// 5 - Bread and butter, Chicken, Vanilla ice cream
    /// 6 - Bread and butter, Chicken, Souffle
    /// 7 - Bread and butter, Burger with fries, Vanilla ice cream
    /// 8 - Bread and butter, Burger with fries, Souffle");
    /// ```
    ///
    /// This code will print:
    /// ```html
    /// 1 - Soup, Chicken, Vanilla ice cream
    /// 2 - Bread and butter, Chicken, Vanilla ice cream
    /// 3 - Soup, Fillet with fries, Vanilla ice cream
    /// 4 - Bread and butter, Fillet with fries, Vanilla ice cream
    /// 5 - Soup, Chicken, Souffle
    /// 6 - Bread and butter, Chicken, Souffle
    /// 7 - Soup, Fillet with fries, Souffle
    /// 8 - Bread and butter, Fillet with fries, Souffle
    /// ```
    pub fn new<FirstLevelIter: IntoIterator<Item=SecondLevelIter>, SecondLevelIter: IntoIterator<Item=T>>(sentences: FirstLevelIter) -> Self {
        let sentences = sentences.into_iter().map(|second_level| second_level.into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();

        let combinations_len =
            sentences.iter()
                .map(|sentences| sentences.into_iter().count()).reduce(|len1, len2| len1 * len2)
                .unwrap();

        let mut accumaleted_module = 1;

        let mut forward_modules = Vec::new();
        let mut iter_for_modules = sentences.iter().rev();
        (0..sentences.iter().count()).for_each(|_| {
            forward_modules.push(accumaleted_module);
            accumaleted_module *= iter_for_modules.next().unwrap().len();
        });
        forward_modules.reverse();

        let mut backward_modules = Vec::new();
        let mut iter_for_modules = sentences.iter().rev();
        (0..sentences.iter().count()).for_each(|_| {
            backward_modules.push(accumaleted_module);
            accumaleted_module *= iter_for_modules.next().unwrap().len();
        });
        backward_modules.reverse();

        Self {
            values: sentences,
            current_combination_forwards: 0,
            current_combination_backwards: combinations_len.checked_sub(1).unwrap_or(0),
            forward_modules,
            backward_modules,
            direction_is_forward: true,
            combinations_len: combinations_len,
        }
    }
}

impl<T> Combinatory<T> {
    /// Makes combinations to grow from right to left, meaning the combinations of \[1,2\] and
    /// \[a,b\] follow the order:
    ///
    /// - (1,a).
    /// - (1,b).
    /// - (2,a).
    /// - (2,b).
    ///
    /// The opposite direction is backwards, and it is indicated with [Self::backward_direction],
    /// and it looks like:
    ///
    /// - (1,a).
    /// - (2,a).
    /// - (1,b).
    /// - (2,b).
    pub fn forward_direction(mut self) -> Self {
        self.direction_is_forward = true;
        self
    }

    /// Makes combinations to grow from left to right, meaning the combinations of \[1,2\] and
    /// \[a,b\] follow the order:
    ///
    /// - (1,a).
    /// - (2,a).
    /// - (1,b).
    /// - (2,b).
    ///
    /// The opposite direction is forwards, and it is indicated with [Self::forward_direction], and
    /// it looks like:
    ///
    /// - (1,a).
    /// - (1,b).
    /// - (2,a).
    /// - (2,b).
    pub fn backward_direction(mut self) -> Self {
        self.direction_is_forward = true;
        self
    }

    /// Gets a combination given its index, for example, given a 2D array like: <br>
    /// [<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp; ["1", "2", "3", "4", "5"],<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp; ["a", "b", "c", "d"],<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp; ["red", "green", "blue"]<br>
    /// ]<br>
    ///
    /// The combination number 32 is ("3", "C", "Green").
    ///
    /// The following code shows that on console:
    ///
    /// ```rust
    /// use combinatory::Combinatory;
    ///
    /// const NUMS: &[&'static str] = &["1","2","3","4","5"];
    /// const LETTERS: &[&'static str] = &["A","B","C","D"];
    /// const COLORS: &[&'static str] = &["Red", "Green", "Blue"];
    ///
    /// let combinatory = Combinatory::new([NUMS, LETTERS, COLORS]);
    /// let combination = combinatory.combination(31).unwrap();
    /// let combination_as_string = format!("{}", combination.map(|string| string.to_string())
    ///     .collect::<Vec<_>>().join(", "));
    /// assert_eq!(combination_as_string, "3, C, Green");
    /// ```
    pub fn combination(&self, combination_index: usize) -> Option<impl DoubleEndedIterator<Item=&T>> {
        if combination_index >= self.combinations_len {
            return None;
        }
        let mut combinations = Vec::with_capacity(self.values.len());
        for index in 0..self.values.len() {
            let modules = if self.direction_is_forward { &self.forward_modules } else { &self.backward_modules };
            combinations.push((combination_index / modules[index]) % self.values[index].len());
        }
        Some(combinations.into_iter().enumerate()
            .map(|(level_1, level_2)| &self.values[level_1][level_2]))
    }

    /// Gives every combination, but as a reference rather than cloning it as [Iterator::next] and
    /// [DoubleEndedIterator::next_back] does.
    pub fn ref_combinations(&self) -> impl DoubleEndedIterator<Item=impl DoubleEndedIterator<Item=&T>> {
        (0..self.combinations_len)
            .map(|combination| self.combination(combination).unwrap())
    }
}

impl<T: Clone> DoubleEndedIterator for Combinatory<T> {
    /// Gives the next combination as a clone, looking from the end.
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_combination_backwards < self.current_combination_forwards {
            return None;
        }
        let res = self.combination(self.current_combination_backwards)?
            .map(|element| element.clone())
            .collect::<Vec<_>>()
            .into_iter();
        self.current_combination_backwards -= 1;
        Some(res)
    }
}

impl<T: Clone> Iterator for Combinatory<T> {
    type Item = IntoIter<T>;

    /// Gives the next combination as a clone, looking from the start.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_combination_backwards < self.current_combination_forwards {
            return None;
        }
        let res = self.combination(self.current_combination_forwards)?
            .map(|element| element.clone())
            .collect::<Vec<_>>()
            .into_iter();
        self.current_combination_forwards += 1;
        Some(res)
    }
}