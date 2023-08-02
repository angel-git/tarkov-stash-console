use std::collections::HashSet;
use std::fmt::Formatter;
use std::io::Error;

use serde_json::Value;

use crate::stash::stash_utils::{get_items, set_fir};

#[derive(Clone)]
pub struct ItemOptions {
    items: Vec<ItemOption>,
    profile_path: String,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ItemOption {
    id: String,
    name: String,
}

impl std::fmt::Display for ItemOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ItemOptions {
    pub fn new(profile_path: String) -> Self {
        let bytes = include_bytes!("../../data/items-3.5.7.json");
        let items_content = String::from_utf8_lossy(bytes);
        let items_root: Value = serde_json::from_str(items_content.as_ref()).unwrap();

        let optional_items = get_items(profile_path.as_str()).unwrap();

        if let Some(items) = optional_items {
            let profile_items = items
                .into_iter()
                .map(|i| String::from(i.get("_tpl").unwrap().as_str().unwrap()));
            let profile_items_names: Vec<ItemOption> = profile_items
                .into_iter()
                .map(|i| {
                    let name = items_root
                        .get(i.as_str())
                        .unwrap()
                        .get("_name")
                        .unwrap()
                        .as_str()
                        .unwrap();

                    let short_name = items_root
                        .get(i.as_str())
                        .unwrap()
                        .get("_props")
                        .unwrap()
                        .get("ShortName")
                        .unwrap()
                        .as_str()
                        .unwrap();

                    let item_name = if name != short_name {
                        format!("{name} ({short_name})")
                    } else {
                        name.to_string()
                    };

                    ItemOption {
                        id: i.clone(),
                        name: item_name,
                    }
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<ItemOption>>();

            ItemOptions {
                items: profile_items_names,
                profile_path,
            }
        } else {
            ItemOptions {
                items: vec![],
                profile_path,
            }
        }
    }

    pub fn get_items(&self) -> Vec<ItemOption> {
        self.items.clone()
    }

    pub fn update_fir_item(&self, item: ItemOption) -> Result<(), Error> {
        set_fir(self.profile_path.as_str(), item.id.as_str())
    }
}
