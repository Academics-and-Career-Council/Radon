use crate::model::{Database, Resources};

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
}

pub type Schema = juniper::RootNode<
    'static,
    Query,
    juniper::EmptyMutation<Database>,
    juniper::EmptySubscription<Database>,
>;
