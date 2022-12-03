// Imports
use garlandtools::GarlandTools;
use super::ID;

// Struct
#[derive(Clone, Debug)]
pub struct Item {
    pub json: serde_json::Value,
}

// Impl
impl Item {
    pub async fn from_json(json: serde_json::Value) -> Self {
        Self { json }
    }

    pub async fn from_id(garland_tools: &GarlandTools, id: u64) -> Self {
        Self::from_json(garland_tools.item(id).await.unwrap()).await
    }

    pub fn get_name(&self) -> String {
        self.json["item"]["name"].as_str().unwrap().to_string()
    }

    pub fn get_id(&self) -> ID {
        self.json["item"]["id"].as_u64().unwrap()
    }

    pub fn is_crafted(&self) -> bool {
        self.json["item"].get("craft").is_some()
    }

    pub fn is_gathered(&self) -> bool {
        self.json["item"].get("nodes").is_some()
    }

    pub fn is_bought(&self) -> bool {
        self.json["item"].get("vendors").is_some()
    }

    pub fn is_ventured(&self) -> bool {
        self.json["item"].get("venture").is_some()
    }

    pub fn is_tradable(&self) -> bool {
        self.json["item"]["tradeable"].as_u64().unwrap() == 1
    }

    pub fn get_crafting_ingredients(&self) -> Vec<(ID, u64)> {
        let mut ingredients = Vec::new();
        if let Some(craft) = self.json["item"].get("craft") {
            for ingredient in craft.as_array().unwrap()[0]["ingredients"]
                .as_array()
                .unwrap()
            {
                ingredients.push((
                    ingredient["id"].as_u64().unwrap(),
                    ingredient["amount"].as_u64().unwrap(),
                ));
            }
        }
        ingredients
    }
}

// Display
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <#{}> -- Crafted: {}, Gathered: {}, Vendors: {}, Ventured: {}, Market Board: {}",
            self.get_name(),
            self.get_id(),
            self.is_crafted(),
            self.is_gathered(),
            self.is_bought(),
            self.is_ventured(),
            self.is_tradable(),
        )
    }
}
