use crate::{
    doc,
    model::{Category, Database, NewObject, Object, Resources, ResourcesFrame, Wings},
    Document, ObjectId, MONGO_DATABASE,
};
use juniper::FieldResult;

pub struct Query;
#[juniper::graphql_object(
	Context = Database,
)]

impl Query {
    #[graphql(arguments(wing(description = "wing of AnC")))]
    fn get_resources_by_wing(wing: String) -> juniper::FieldResult<Vec<Resources>> {
        let db = Database::new();
        Ok(db.get_resources(wing))
    }

    fn get_wings() -> juniper::FieldResult<Vec<Wings>> {
        let db = Database::new();
        Ok(db.get_wings())
    }

    fn get_document(id: String) -> FieldResult<Object> {
        let objects_db = MONGO_DATABASE.collection::<Object>("objects");
        let fetched_document =
            objects_db.find_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, None);
        let err_doc = Object {
            id: "NA".to_string(),
            name: "NA".to_string(),
            category: Category::gdrive,
            link: "NA".to_string(),
        };
        let document = match fetched_document {
            Ok(fetched_document) => match fetched_document {
                None => err_doc,
                Some(document) => document,
            },
            Err(_err) => err_doc,
        };
        return Ok(document);
    }
}

pub struct Mutation;

#[juniper::graphql_object(
	Context = Database,
)]
impl Mutation {
    fn add_object(
        new_object: NewObject,
        heading: String,
        exists: bool,
        order: String,
        wing: String,
    ) -> FieldResult<Object> {
        let object = doc! {"name":new_object.name.clone(), "category":new_object.category.to_string(), "link":new_object.link.clone()};
        let resources_db = MONGO_DATABASE.collection::<Document>("resources");
        let objects_db = MONGO_DATABASE.collection::<Document>("objects");
        let inserted_object = objects_db
            .insert_one(object, None)
            .expect(&format!("write failed for {}", new_object.name));
        let mut inserted_id = inserted_object.inserted_id.to_string();
        inserted_id = inserted_id.split("\"").collect::<Vec<&str>>()[1].to_string();
        if exists {
            let _updated_resource = resources_db
                .update_one(
                    doc! {"title":heading.clone()},
                    doc! {"$addToSet":{"object_ids":inserted_id.clone()}},
                    None,
                )
                .expect(&format!("adding to array failed for {}", heading));
            Ok(Object {
                id: inserted_id,
                name: new_object.name,
                category: new_object.category,
                link: new_object.link,
            })
        } else {
            // wing: String,
            // order: i32,
            // title: String,
            // category: Type,
            // object_ids: Vec<String>,
            let resource_frame = doc! {"wing": wing, "order": order.parse::<i32>().unwrap() , "title":heading.clone(), "category" : "document", "object_ids": vec![inserted_id] };
            resources_db
                .insert_one(resource_frame, None)
                .expect(&format!("adding {} failed", heading));
            Ok(Object {
                id: "inserted_id".to_string(),
                name: new_object.name,
                category: new_object.category,
                link: new_object.link,
            })
        }
    }

    fn edit_object(data: NewObject, id: String) -> FieldResult<Object> {
        let updated_doc = doc! {"$set" :{"name": data.name.clone(), "category": data.category.to_string(), "link" :data.link.clone() }};
        let objects_db = MONGO_DATABASE.collection::<Document>("objects");
        objects_db
            .update_one(
                doc! {"_id": ObjectId::parse_str(id.clone()).unwrap()},
                updated_doc,
                None,
            )
            .expect(&format!("edit failed for id {}", id));
        return Ok(Object {
            id,
            name: data.name,
            category: data.category,
            link: data.link,
        });
    }

    fn delete_objects(heading_id: String, object_id: String) -> FieldResult<bool> {
        // println!("{:#?}", data);
        let resources_db = MONGO_DATABASE.collection::<ResourcesFrame>("resources");
        let objects_db = MONGO_DATABASE.collection::<Object>("objects");
        // for heading_id in data.headings {
        //     resources_db.update_one(doc! {"_id": ObjectId::parse_str(heading_id.id.clone()).unwrap()}, doc!{ "$pullAll": { "object_ids": heading_id } }, options)
        // }
        let found_resource = resources_db
            .find_one(
                doc! {"_id": ObjectId::parse_str(heading_id.clone()).unwrap()},
                None,
            )
            .expect(&format!("find failed for {}", heading_id))
            .unwrap();
        if found_resource.object_ids.len() == 1 {
            resources_db
                .delete_one(
                    doc! {"_id": ObjectId::parse_str(heading_id.clone()).unwrap()},
                    None,
                )
                .expect(&format!("find failed for {}", heading_id));
        } else {
            resources_db
                .update_one(
                    doc! {"_id": ObjectId::parse_str(heading_id.clone()).unwrap()},
                    doc! { "$pullAll": { "object_ids": [ object_id.clone() ] } },
                    None,
                )
                .expect(&format!("delete failed for {}", heading_id));
        }
        objects_db
            .delete_one(
                doc! {"_id": ObjectId::parse_str(object_id.clone()).unwrap()},
                None,
            )
            .expect(&format!("delete failed for {}", object_id));
        Ok(true)
    }
}

pub type Schema = juniper::RootNode<
    'static,
    Query,
    // juniper::EmptyMutation<Database>,
    Mutation,
    juniper::EmptySubscription<Database>,
>;
