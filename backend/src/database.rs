use std::collections::HashMap;

use model::{PostShopItem, ShoppingListItem};

pub struct ShoppingList {
    list: HashMap<String, ShoppingItem>,
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self {
            list: [
                (
                    "6855cfc9-78fd-4b66-8671-f3c90ac2abd8".to_string(),
                    ShoppingItem {
                        title: "Coffee".to_string(),
                        creator: "Roland".to_string(),
                    },
                ),
                (
                    "3d778d1c-5a4e-400f-885d-10212027382d".to_string(),
                    ShoppingItem {
                        title: "Tomato Seeds".to_string(),
                        creator: "Tania".to_string(),
                    },
                ),
            ]
            .into(),
        }
    }
}

#[derive(Clone)]
pub struct ShoppingItem {
    title: String,
    creator: String,
}

impl From<PostShopItem> for ShoppingItem {
    fn from(value: PostShopItem) -> Self {
        Self {
            title: value.title,
            creator: value.posted_by,
        }
    }
}

pub struct InMemoryDatabase {
    db: HashMap<String, ShoppingList>,
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        let mut db = HashMap::new();
        db.insert(
            "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string(),
            ShoppingList::default(),
        );
        InMemoryDatabase { db }
    }
}

impl InMemoryDatabase {
    pub fn insert_item(
        &mut self,
        list_uuid: String,
        item_uuid: String,
        shopping_item: ShoppingItem,
    ) {
        self.db
            .get_mut(&list_uuid)
            .and_then(|list| list.list.insert(item_uuid, shopping_item));
    }

    pub fn delete_item(&mut self, list_uuid: String, item_uuid: String) {
        self.db
            .get_mut(&list_uuid)
            .and_then(|list| list.list.remove(&item_uuid));
    }

    pub fn create_list(&mut self, list_uuid: String) {
        self.db.insert(list_uuid, ShoppingList::default());
    }

    fn get_list(&self, list_uuid: String) -> Option<&ShoppingList> {
        self.db.get(&list_uuid)
    }

    pub fn as_vec(&self, list_uuid: String) -> Vec<ShoppingListItem> {
        let list = self.get_list(list_uuid);
        match list {
            Some(list) => list
                .list
                .iter()
                .map(|(key, item)| ShoppingListItem {
                    title: item.title.clone(),
                    posted_by: item.creator.clone(),
                    uuid: key.clone(),
                })
                .collect(),
            None => Vec::default(),
        }
    }
}
