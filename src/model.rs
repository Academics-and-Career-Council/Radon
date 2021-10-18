use crate::{doc, serde_helpers, Deserialize, ObjectId, Serialize, MONGO_DATABASE};
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Category {
    pdf,
    gdrive,
    youtube,
    zoom,
}

#[allow(non_camel_case_types)]
#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Type {
    document,
    video,
}
#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    #[serde(
        rename = "_id",
        serialize_with = "serde_helpers::serialize_hex_string_as_object_id",
        deserialize_with = "serde_helpers::deserialize_hex_string_from_object_id"
    )]
    id: String,
    name: String,
    category: Category,
    link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesFrame {
    #[serde(rename = "_id")]
    id: ObjectId,
    wing: String,
    order: i32,
    title: String,
    category: Type,
    object_ids: Vec<String>,
}

#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    id: String,
    wing: String,
    order: i32,
    title: String,
    category: Type,
    objects: Vec<Object>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub resources: HashMap<String, Vec<Resources>>,
}

impl Database {
    pub fn new() -> Database {
        let mut objects: HashMap<String, Object> = HashMap::new();
        // let mut resources: Vec<Resources> = Vec::new();
        let mut resources: HashMap<String, Vec<Resources>> = HashMap::new();

        let objects_db = MONGO_DATABASE.collection::<Object>("objects");
        let objects_cursor = objects_db.find(None, None).unwrap();

        let resources_db = MONGO_DATABASE.collection::<ResourcesFrame>("resources");
        let resources_cursor = resources_db.find(None, None).unwrap();

        for obj in objects_cursor {
            // makes map of object_ids : objects
            let item = obj.unwrap();
            objects.insert(item.id.clone(), item);
        }

        for obj in resources_cursor {
            // returns populated db
            let item = obj.unwrap();
            let mut items_vec: Vec<Object> = Vec::new();
            for id in item.object_ids.clone() {
                let found = objects.remove(&id).unwrap();
                items_vec.push(found);
            }
            let res: Resources = Resources {
                id: item.id.clone().to_hex(),
                wing: item.wing.clone(),
                order: item.order.clone(),
                title: item.title.clone(),
                category: item.category.clone(),
                objects: items_vec,
            };
            if resources.contains_key(&res.wing) {
                let key: String = res.wing.clone();
                let mut val: Vec<Resources> = resources.remove(&res.wing).unwrap();
                val.push(res);
                resources.insert(key, val);
            } else {
                let key: String = res.wing.clone();
                let val: Vec<Resources> = vec![res];
                resources.insert(key, val);
            }
        }

        return Database {
            resources: resources,
        };
    }

    pub fn get_resources(&self, wing: String) -> Vec<Resources> {
        return self.resources.get(&wing).unwrap().clone();
    }
}
