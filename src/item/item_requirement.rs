use crate::Item;
use garlandtools::GarlandTools;

// Struct
#[derive(Clone, Debug)]
pub struct ItemRequirement {
    pub item: Item,
    pub quantity: u64,
}

// Impl
impl ItemRequirement {
    pub async fn from_id(garland_tools: &GarlandTools, id: u64, quantity: u64) -> ItemRequirement {
        ItemRequirement {
            item: Item::from_id(garland_tools, id).await,
            quantity,
        }
    }

    pub async fn from_item(
        garland_tools: &GarlandTools,
        item: Item,
        quantity: u64,
    ) -> ItemRequirement {
        ItemRequirement { item, quantity }
    }
}

// Display
impl std::fmt::Display for ItemRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {}", self.quantity, self.item)
    }
}
