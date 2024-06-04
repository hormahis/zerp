use rhai::{
	CustomType,
	ImmutableString,
	TypeBuilder,
};
use serde::Deserialize;


pub type ObjectReference = String;

#[derive(Debug, Clone, Deserialize)]
pub struct Object
{
	/// Unique object identifier
	pub id: ObjectReference,
	/// Object class name
	pub name: String,
	/// Overriding object metainformation
	pub meta: ObjectMeta,
	/// Objects within the object
	pub map: Vec<ObjectReference>,
	/// Object position in the world and additional info
	pub location: ObjectLocation,

	#[serde(default)]
	pub scripts: ObjectScripts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObjectMeta {}


#[derive(Debug, Clone, Deserialize)]
pub struct ObjectLocation {
	/// Imrpovised X axis
	pub lon: i32,
	/// Improvised Y axis
	pub lat: i32,
	/// Those who "discovered" the object
	#[serde(default)]
	pub nearbies: Vec<crate::models::person::PersonReference>,
	/// Those who "entered" the object
	#[serde(default)]
	pub entered: Vec<crate::models::person::PersonReference>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ObjectScripts {
	/// Object was created
	pub on_init: Option<String>,

	pub on_event: Option<String>,
	pub on_daytime: Option<String>,

	/// Someone was spotted nearby
	pub on_discover: Option<String>,
	/// ... and moved away
	pub on_bypass: Option<String>,

	/// Someone entered the object
	pub on_enter: Option<String>,
	/// ... and left it
	pub on_leave: Option<String>,
}



impl Object {
	pub fn get_id(&mut self) -> ImmutableString { self.id.clone().into() }
}



impl CustomType for Object {

	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("Object")
			.with_get("id", Self::get_id)
		;

	}

}