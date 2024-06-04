use rhai::{
	CustomType,
	TypeBuilder,
};

use super::worldobject::WorldObject;


// Поля, выставляемые в объект скрипта
impl CustomType for WorldObject {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("WorldObject")
			.with_get("id", WorldObject::get_id)
			.with_get("name", WorldObject::get_name)
			.with_get("title", WorldObject::get_title)
			.with_fn("attribute", WorldObject::get_attribute)
			.on_print(WorldObject::on_print)
		;
	}
}