use combinatory::Combinatory;

const SIDES: &[&'static str] = &["Soup", "Bread and butter"];
const MAIN: &[&'static str] = &["Chicken", "Burger with fries"];
const DESSERTS: &[&'static str] = &["Vanilla ice cream", "Souffle"];

#[test]
fn test_all_combinations() {
    let possible_menus = Combinatory::new([SIDES, MAIN, DESSERTS])
        .ref_combinations()
        .enumerate()
        .map(|(menu_index, possible_menu)| {
            format!("{} - {}", menu_index + 1, possible_menu
                .map(|string| string.to_string()).collect::<Vec<_>>().join(", "))
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{possible_menus}");

    assert_eq!(possible_menus, "1 - Soup, Chicken, Vanilla ice cream
2 - Soup, Chicken, Souffle
3 - Soup, Burger with fries, Vanilla ice cream
4 - Soup, Burger with fries, Souffle
5 - Bread and butter, Chicken, Vanilla ice cream
6 - Bread and butter, Chicken, Souffle
7 - Bread and butter, Burger with fries, Vanilla ice cream
8 - Bread and butter, Burger with fries, Souffle");
}

const NUMS: &[&'static str] = &["1", "2", "3", "4", "5"];
const LETTERS: &[&'static str] = &["A", "B", "C", "D"];
const COLORS: &[&'static str] = &["Red", "Green", "Blue"];

#[test]
fn test_one_combination() {
    let combinatory = Combinatory::new([NUMS, LETTERS, COLORS]);
    let combination = combinatory.combination(31).unwrap();
    let combination_as_string = format!("{}", combination.map(|string| string.to_string())
        .collect::<Vec<_>>().join(", "));
    assert_eq!(combination_as_string, "3, C, Green");
}