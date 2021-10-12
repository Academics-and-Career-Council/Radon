// use crate::model::{Fetch, Database, Resources, Object, Block, Type};
// use juniper::{graphql_interface, Context};

// impl Context for Database {}

// graphql_interface!(<'a> &'a dyn Fetch: Database as "Fetch" |&self| {
//   description: "A Resource in AnC"

//   field  id() -> &i64 as "The id of the Resource" {
//     self.id()
//   }

//   field wing() -> &str as "Wing of the Resource" {
//     self.wing()
//   }

//   field title() -> &str as "Title of the resource under wing" {
//     self.title()
//   }

//   field category() -> &Type as "Type of the Resource" {
//     self.category()
//   }

//   // field objects(&executor) -> Vec<& object> {
//   //   executor.context().populate_objects(self.as_resource())
//   // }

//   instance_resolvers: |&context| {
// 		&dyn Block =>
//   }
// );

// #[juniper::object(
// 	Context = Database,
// 	// Scalar = juniper::DefaultScalarValue,
// 	interfaces = [&dyn Fetch],
// )]
// impl<'a> &'a dyn Block {
// 	fn id(&self) -> &str {
// 		self.id()
// 	}

// 	fn wing(&self) -> &str {
// 		self.wing()
// 	}

// 	fn title(&self) ->&str {
// 		self.title()
// 	}

// 	fn category(&self) -> &Type {
// 		&self.category()
// 	}

// 	// fn objects(&self, ctx:&Database){
// 	//   let b = ctx.populate_objects(self.wing());
// 	// }
// }

pub struct Query;
use crate::model::{Database, Resources};

#[juniper::graphql_object(
	Context = Database,
	// Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    #[graphql(arguments(wing(description = "wing of AnC")))]
    fn get_resources(wing: String) -> juniper::FieldResult<Vec<Resources>> {
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
