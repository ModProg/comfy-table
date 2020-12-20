use pretty_assertions::assert_eq;

use comfy_table::presets::*;
use comfy_table::*;

fn get_preset_table() -> Table {
    let mut table = Table::new();
    table.set_header(&vec!["Header1", "Header2", "Header3"]);
    table.add_row(&vec!["One One", "One Two", "One Three"]);
    table.add_row(&vec!["One One", "One Two", "One Three"]);

    table
}

#[test]
fn ascii_no_borders() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_NO_BORDERS);
    let expected = "
 Header1 | Header2 | Header3
===============================
 One One | One Two | One Three
---------+---------+-----------
 One One | One Two | One Three";
    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn ascii_borders_only() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_BORDERS_ONLY);
    let expected = "
+-------------------------------+
| Header1   Header2   Header3   |
+===============================+
| One One   One Two   One Three |
|                               |
| One One   One Two   One Three |
+-------------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn ascii_horizontal_borders_only() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_HORIZONTAL_BORDERS_ONLY);
    let expected = "
-------------------------------
 Header1   Header2   Header3
===============================
 One One   One Two   One Three
-------------------------------
 One One   One Two   One Three
-------------------------------";
    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn ascii_markdown() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_MARKDOWN);
    let expected = "
| Header1 | Header2 | Header3   |
|---------|---------|-----------|
| One One | One Two | One Three |
| One One | One Two | One Three |";

    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn utf8_full() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_FULL);
    let expected = "
┌─────────┬─────────┬───────────┐
│ Header1 ┆ Header2 ┆ Header3   │
╞═════════╪═════════╪═══════════╡
│ One One ┆ One Two ┆ One Three │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ One One ┆ One Two ┆ One Three │
└─────────┴─────────┴───────────┘";

    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn utf8_no_borders() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_NO_BORDERS);
    let expected = "
 Header1 ┆ Header2 ┆ Header3
═════════╪═════════╪═══════════
 One One ┆ One Two ┆ One Three
╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌
 One One ┆ One Two ┆ One Three";
    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn utf8_borders_only() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_BORDERS_ONLY);
    let expected = "
┌───────────────────────────────┐
│ Header1   Header2   Header3   │
╞═══════════════════════════════╡
│ One One   One Two   One Three │
│ One One   One Two   One Three │
└───────────────────────────────┘";

    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn utf8_horizontal_borders_only() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_HORIZONTAL_BORDERS_ONLY);
    let expected = "
───────────────────────────────
 Header1   Header2   Header3
═══════════════════════════════
 One One   One Two   One Three
───────────────────────────────
 One One   One Two   One Three
───────────────────────────────";
    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}
