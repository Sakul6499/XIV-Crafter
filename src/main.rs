// Macro Crates
#[macro_use]
extern crate lazy_static;

// Modules
mod globals;
mod item;
mod query;

// Imports
use item::{Item, ItemRequirement};
use query::Query;

#[tokio::main]
async fn main() {
    let query_result = Query::query_from_item(24451, 1).await;

    handle_ventures(&query_result.items_is_ventured);
    handle_vendors(&query_result.items_is_bought);
    handle_gathered(&query_result.items_is_gathered);
    handle_crafted(&query_result.items_is_crafted);
}

fn handle_ventures(venture_items: &Vec<ItemRequirement>) {
    println!();
    println!("=== Venture Items ===");
    venture_items.iter().for_each(|item| println!("{}", item));
}

fn handle_vendors(vendor_items: &Vec<ItemRequirement>) {
    println!();
    println!("=== Vendor Items ===");
    vendor_items.iter().for_each(|item| println!("{}", item));
}

fn handle_gathered(gathered_items: &Vec<ItemRequirement>) {
    println!();
    println!("=== Gathered Items ===");
    gathered_items.iter().for_each(|item| println!("{}", item));
}

fn handle_crafted(crafted_items: &Vec<ItemRequirement>) {
    println!();
    println!("=== Crafted Items ===");
    crafted_items.iter().for_each(|item| println!("{}", item));
}
