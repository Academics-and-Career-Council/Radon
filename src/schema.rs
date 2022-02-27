use crate::{
    doc,
    model::{Category, Database, NewObject, Object, Resources, ResourcesFrame, Wings},
    Document, ObjectId, MONGO_DATABASE,
};
use chrono::{offset::Local, DateTime};
use juniper::FieldResult;
use mongodb::error::Error;
use mongodb::results::DeleteResult;
use std::time::SystemTime;

struct Log {
    service_name: String,
    status_code: i32,
    severity: String,
    msg_name: String,
    msg: String,
    invoked_by: String,
    result: String,
    timestamp: String,
}

impl Log {
    fn new(
        // service_name: String,
        status_code: i32,
        severity: String,
        msg_name: String,
        msg: String,
        invoked_by: String,
        result: String,
        timestamp: String,
    ) -> Self {
        Self {
            service_name: "radon".to_string(),
            status_code,
            severity,
            msg_name,
            msg,
            invoked_by,
            result,
            timestamp,
        }
    }
}

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
            return Ok(Object {
                id: inserted_id,
                name: new_object.name,
                category: new_object.category,
                link: new_object.link,
            });
        } else {
            let resource_frame = doc! {"wing": wing, "order": order.parse::<i32>().unwrap() , "title":heading.clone(), "category" : "document", "object_ids": vec![inserted_id] };
            resources_db
                .insert_one(resource_frame, None)
                .expect(&format!("adding {} failed", heading));
            return Ok(Object {
                id: "inserted_id".to_string(),
                name: new_object.name,
                category: new_object.category,
                link: new_object.link,
            });
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
        // for logging
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        let timestamp: String = datetime.format("D%d/%m/%YT%T").to_string();

        let resources_db = MONGO_DATABASE.collection::<ResourcesFrame>("resources");
        let objects_db = MONGO_DATABASE.collection::<Object>("objects");
        let found_resource = resources_db
            .find_one(
                doc! {"_id": ObjectId::parse_str(heading_id.clone()).unwrap()},
                None,
            )
            .expect(&format!("find failed for {}", heading_id))
            .unwrap();

        if found_resource.object_ids.len() == 1 {
            let delete_result: Result<DeleteResult, Error> = resources_db.delete_one(
                doc! {"_id": ObjectId::parse_str(heading_id.clone()).unwrap()},
                None,
            );
            // .expect(&format!("find failed for {}", heading_id));
            match delete_result {
                Ok(_) => {
                    // creating a logging struct
                    Log::new(
                        200,
                        "crit".to_string(),
                        format!("Resource Block id {{resource_id}}, deleted by {{user_id/email}}"),
                        format!(
                            "Resource Block id {{resource_id}} deleted by {{user_id/email}} at {}",
                            datetime.format("%d/%m/%y %T").to_string()
                        ),
                        format!("{{user_id/email}}"),
                        "success".to_string(),
                        timestamp.clone(),
                    );
                }
                Err(err) => {
                    // creating a logging struct
                    Log::new(500,
                        "crit".to_string(),
                        format!("Deletion of resource block id {{resource_id}} failed"), 
                        format!(
                        "Deletion of resource block id {{resource_id}} by {{user_id/email}} failed at {}. Reson: {}",
                        datetime.format("%d/%m/%y %T").to_string(), err.to_string()),
                        format!("{{user_id/email}}"), "failure".to_string(),
                        timestamp.clone());
                }
            }
        } else {
            let resource_update_result = resources_db.update_one(
                doc! {"_id": ObjectId::parse_str(heading_id.clone()).unwrap()},
                doc! { "$pullAll": { "object_ids": [ object_id.clone() ] } },
                None,
            );
            // .expect(&format!("delete failed for {}", heading_id));
            match resource_update_result {
                Ok(_) => {
                    // creating a logging struct
                    Log::new(
                        200,
                        "crit".to_string(),
                        format!("Resource Block id {{resource_id}}, updated by {{user_id/email}}"),
                        format!(
                            "Resource Block id {{resource_id}} updated by {{user_id/email}} at {}",
                            datetime.format("%d/%m/%y %T").to_string()
                        ),
                        format!("{{user_id/email}}"),
                        "success".to_string(),
                        timestamp.clone(),
                    );
                }
                Err(err) => {
                    // creating a logging struct
                    Log::new(500,
                            "crit".to_string(),
                            format!("Edit resource block id {{resource_id}} failed"), 
                            format!(
                            "Edit resource block id {{resource_id}} by {{user_id/email}} failed at {}. Reson: {}",
                            datetime.format("%d/%m/%y %T").to_string(), err.to_string()),
                            format!("{{user_id/email}}"), "failure".to_string(),
                            timestamp.clone());
                }
            }
        }
        let object_delete_result = objects_db.delete_one(
            doc! {"_id": ObjectId::parse_str(object_id.clone()).unwrap()},
            None,
        );
        // .expect(&format!("delete failed for {}", object_id));

        match object_delete_result {
            Ok(_) => {
                // creating a logging struct
                Log::new(
                    200,
                    "crit".to_string(),
                    format!("Object id {{resource_id}}, deleted by {{user_id/email}}"),
                    format!(
                        "Object id {{resource_id}} deleted by {{user_id/email}} at {}",
                        datetime.format("%d/%m/%y %T").to_string()
                    ),
                    format!("{{user_id/email}}"),
                    "success".to_string(),
                    timestamp.clone(),
                );
            }
            Err(err) => {
                // creating a logging struct
                Log::new(500,
                        "crit".to_string(),
                        format!("Deletion of Object id {{resource_id}} failed"), 
                        format!(
                        "Deletion of resource block id {{resource_id}} by {{user_id/email}} failed at {}. Reson: {}",
                        datetime.format("%d/%m/%y %T").to_string(), err.to_string()),
                        format!("{{user_id/email}}"), "failure".to_string(),
                        timestamp.clone());
            }
        }
        return Ok(true);
    }
}

pub type Schema = juniper::RootNode<
    'static,
    Query,
    // juniper::EmptyMutation<Database>,
    Mutation,
    juniper::EmptySubscription<Database>,
>;
