use std::{collections::HashMap};
use crate::{MONGO_DATABASE, LOCAL_DB};
use crate::{Deserialize, Serialize, doc, Document,ObjectId};

#[allow(non_camel_case_types)]
#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Category {
	// #[graphql(name="pdf")]
	pdf,
	// #[graphql(name="gdrive")]
	gdrive,
	// #[graphql(name="youtube")]
	youtube,
	// #[graphql(name="zoom")]
	zoom
}

#[allow(non_camel_case_types)]
#[derive(GraphQLEnum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Type {
	// #[graphql(name="document")]
	document,
	// #[graphql(name="video")]
	video
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Object {
	#[serde(rename="_id")]
	id: ObjectId,
	name: String,
	category: Category,
	link: String
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct ResourcesFrame {
	#[serde(rename="_id")]
	id: ObjectId,
	wing: String,
	title: String,
	category: Type,
	object_ids: Vec<ObjectId>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
	frame: ResourcesFrame,
	objects: Vec<Object>
}

pub trait Fetch {
	fn id(&self) -> &ObjectId;
	fn wing(&self) -> &str;
	fn title(&self) -> &str;
	fn category(&self) -> &Type;
	fn objects_ids(&self) -> &Vec<ObjectId>;
	fn as_resource(&self) -> & dyn Fetch;
}

pub trait Content {
	fn id(&self) -> &str;
	fn name(&self) -> &str;
	fn category(&self) -> &Category;
	fn link(&self) -> &str;
}

pub trait ResourceList:Fetch {
	fn objects(&self) -> Vec<&Object>;
}

impl Fetch for ResourcesFrame {
	fn id(&self) -> &ObjectId {
		&self.id
	}
	fn wing(&self) -> &str {
		&self.wing
	}
	fn title(&self) -> &str {
		&self.title
	}
	fn category(&self) ->&Type {
		&self.category
	}
	fn objects_ids(&self) -> &Vec<ObjectId> {
		&self.object_ids
	}
	fn as_resource(&self) -> &dyn Fetch {
		self
	}
}

impl Fetch for Resources {
	fn id(&self) -> &ObjectId {
		&self.frame.id
	}
	fn wing(&self) -> &str {
		&self.frame.wing
	}
	fn title(&self) -> &str {
		&self.frame.title
	}
	fn category(&self) ->&Type {
		&self.frame.category
	}
	fn objects_ids(&self) -> &Vec<ObjectId> {
		&self.frame.object_ids
	}
	fn as_resource(&self) -> &dyn Fetch {
		self
	}
}

// impl ResourceList for Resources {
//   fn objects(&self) -> Vec<&Object> {

//       let db = &LOCAL_DB;
// 			// let obj = thread::spawn(move || {
// 			// 	let mut ids = self.frame.object_ids.clone();
// 			// 	let objects = ids.into_iter().map(|id| db.objects.get(& id).expect("Not found").clone()).collect::<Vec<_>>();
// 			// 	objects
// 			// }).unwrap();
// 			let ids = self.frame.object_ids.clone();
// 			let objects = ids.into_iter().map(|id| db.objects.get(& id).expect("Not found").clone()).collect::<Vec<_>>();
// 			objects
//   }
// }

// impl ResourcesFrame {
//   pub fn new (id: &ObjectId, wing: &str, title: &str,category: &Type, object_ids: &[&str]) -> ResourcesFrame{
//     ResourcesFrame {
//       id: id.to_owned(),
//       wing: wing.to_owned(),
//       title: title.to_owned(),
//       category: category.to_owned(),
//       object_ids: object_ids.to_owned().into_iter().map(|obj| obj.to_owned()).collect()
//     }
//   }
// }

impl Object {
	pub fn new(id: &ObjectId, name: &str, category: &Category, link: &str) -> Object {
		Object {
			id: id.to_owned(),
			name: name.to_owned(),
			category: category.to_owned(),
			link: link.to_owned()
		}
	}
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
	resources_frame: HashMap<String, ResourcesFrame>,
	objects: HashMap<ObjectId, Object>,
	resources: HashMap<String, Vec<Resources>>
}

impl Database {
	pub fn new() -> Database {
		let mut resources_frame:HashMap<String, ResourcesFrame> = HashMap::new();
		let mut objects:HashMap<ObjectId, Object> = HashMap::new();
		let mut resources: HashMap<String, Vec<Resources>> = HashMap::new();

		let objects_db = MONGO_DATABASE.collection::<Object>("objects");
		let objects_cursor = objects_db.find(None, None).unwrap();
		
		let resources_db = MONGO_DATABASE.collection::<ResourcesFrame>("resources");
		let resources_cursor = resources_db.find(None, None).unwrap();
		
		for obj in objects_cursor{
			let item = obj.unwrap();
			// println!("from objects{:#?}", item);
			objects.insert(item.id.clone(), item);
		}

		for obj in resources_cursor{
			let item = obj.unwrap();
			// println!("from resources{:#?}", item);
			resources_frame.insert(item.wing.clone(), item);
		}
		
		for (key, val) in resources_frame.clone() {
			let mut items_vec:Vec<Object> = Vec::new();
			for id in val.object_ids.clone() {
				let found = objects.get(&id).unwrap().clone();
				items_vec.push(found);
			}
			let res:Resources = Resources {
				frame: val,
				objects: items_vec
			};
		}

		return Database {
				resources_frame: resources_frame,
				objects: objects,
				resources: resources
		};
	}
	

	// pub fn get_object(&self, id: &str) -> Option<& Object> {
	//   if let Some(h) = self.objects.get(id) {
	//     Some(h)
	//   }
	//   else {
	//     None
	//   }
	// }

	// pub fn populate_objects(&self, ctx: &dyn Fetch) -> Vec<& Object> {
	//   ctx.objects().iter().flat_map(|id| self.get_object(id)).collect::<Vec<_>>()
	// }

	// pub fn get_resource(&self, wing:&str) -> Option<&dyn Fetch> {
	//   self.resources.get(wing).map(|resource| resource as &dyn Fetch)
	// }


}