// Imports
use crate::item::ItemRequirement;

// Structs
#[derive(Default, Clone)]
pub struct QueryResult {
    pub items_is_crafted: Vec<ItemRequirement>,
    pub items_is_gathered: Vec<ItemRequirement>,
    pub items_is_bought: Vec<ItemRequirement>,
    pub items_is_ventured: Vec<ItemRequirement>,
}

// Impl
#[allow(dead_code)]
impl QueryResult {
    pub fn new(
        items_is_crafted: Vec<ItemRequirement>,
        items_is_gathered: Vec<ItemRequirement>,
        items_is_bought: Vec<ItemRequirement>,
        items_is_ventured: Vec<ItemRequirement>,
    ) -> Self {
        Self {
            items_is_crafted,
            items_is_gathered,
            items_is_bought,
            items_is_ventured,
        }
    }

    pub fn combine(&mut self, other: &Self) -> &Self {
        self.items_is_crafted.extend(other.items_is_crafted.clone());
        self.items_is_gathered
            .extend(other.items_is_gathered.clone());
        self.items_is_bought.extend(other.items_is_bought.clone());
        self.items_is_ventured
            .extend(other.items_is_ventured.clone());

        self
    }

    pub fn combine_move(mut self, other: Self) -> Self {
        self.items_is_crafted.extend(other.items_is_crafted);
        self.items_is_gathered.extend(other.items_is_gathered);
        self.items_is_bought.extend(other.items_is_bought);
        self.items_is_ventured.extend(other.items_is_ventured);

        self
    }
}

// Display
impl std::fmt::Display for QueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "=== Venture Items ===\n")?;
        self.items_is_ventured.iter().for_each(|item| {
            write!(f, "{}\n", item).unwrap();
        });
        write!(f, "\n")?;

        write!(f, "=== Gathered Items ===\n")?;
        self.items_is_gathered.iter().for_each(|item| {
            write!(f, "{}\n", item).unwrap();
        });
        write!(f, "\n")?;

        write!(f, "=== Crafted Items ===\n")?;
        self.items_is_crafted.iter().for_each(|item| {
            write!(f, "{}\n", item).unwrap();
        });
        write!(f, "\n")
    }
}
