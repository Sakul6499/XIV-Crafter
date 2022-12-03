// Imports
use crate::{
    globals::GARLAND_TOOLS_BASE,
    item::{Item, ItemRequirement, ID},
};

use super::QueryResult;

// Structs
#[derive(Clone)]
pub struct Query {}

// Impl
impl Query {
    pub async fn query_from_multiple_items(v: Vec<(ID, u64)>) -> QueryResult {
        todo!()
    }

    pub async fn query_from_item(id: ID, amount: u64) -> QueryResult {
        let mut query_result = QueryResult::default();
        let mut ids_to_query: Vec<(ID, u64)> = vec![(id, amount)];

        loop {
            if ids_to_query.is_empty() {
                break;
            }

            let mut new_collection_of_ids_to_query: Vec<(ID, u64)> = Vec::new();

            for (id, amount) in ids_to_query {
                let (new_ids_to_query, new_query_result) = Query::query_item(id).await;

                query_result = query_result.combine_move(new_query_result);
                new_collection_of_ids_to_query.extend(new_ids_to_query);
            }

            ids_to_query = new_collection_of_ids_to_query;
        }

        query_result
    }

    async fn query_item_with_ingredients(id: ID, amount: u64) -> (Vec<(ID, u64)>, QueryResult) {
        let item = Item::from_id(&GARLAND_TOOLS_BASE, id).await;

        let mut query_result = QueryResult::default();
        let mut ids_to_query: Vec<(ID, u64)> = Vec::new();

        let mut items = item.get_crafting_ingredients();
        items.push((id, amount));

        for (ingredient_id, ingredient_amount) in items {
            let item_req =
                ItemRequirement::from_id(&GARLAND_TOOLS_BASE, ingredient_id, ingredient_amount)
                    .await;

            if item_req.item.is_crafted() {
                let nested_ingredients = item_req.item.get_crafting_ingredients();
                ids_to_query.extend(nested_ingredients);

                query_result.items_is_crafted.push(item_req);
            } else if item_req.item.is_gathered() {
                query_result.items_is_gathered.push(item_req);
            } else if item_req.item.is_bought() {
                query_result.items_is_bought.push(item_req);
            } else if item_req.item.is_ventured() {
                query_result.items_is_ventured.push(item_req);
            }
        }

        (ids_to_query, query_result)
    }
}
