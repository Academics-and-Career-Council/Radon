use crate::{
    doc,
    model::{Category, Database, Object, Resources, ResourcesFrame, Wings, NewObject},
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

// #[graphql_object]
// impl Mutation {
//     fn delete_objects(wing: String, ids:Vec<String>) -> FieldResult<Vec<Resources>> {
//         let database = MONGO_DATABASE;
//         let db = Database::new();
//         Ok(db.get_resources(wing))
//     }
// }

#[juniper::graphql_object(
	Context = Database,
)]
impl Mutation {
    fn add_object(
        heading: String,
        name: String,
        category: Category,
        link: String,
    ) -> FieldResult<Object> {
        let object =
            doc! {"name":name.clone(), "category":category.to_string(), "link":link.clone()};
        let resources_db = MONGO_DATABASE.collection::<ResourcesFrame>("resources");
        let objects_db = MONGO_DATABASE.collection::<Document>("objects");
        let inserted_object = objects_db
            .insert_one(object, None)
            .expect(&format!("write failed for {}", name));
        let inserted_id = inserted_object.inserted_id.to_string();
        let _updated_resource = resources_db
            .update_one(
                doc! {"title":heading.clone()},
                doc! {"$addToSet":{"object_ids":inserted_id.clone()}},
                None,
            )
            .expect(&format!("adding to array failed for {}", heading));
        Ok(Object {
            id: inserted_id,
            name: name,
            category: category,
            link: link,
        })
    }
}

pub type Schema = juniper::RootNode<
    'static,
    Query,
    // juniper::EmptyMutation<Database>,
    Mutation,
    juniper::EmptySubscription<Database>,
>;
