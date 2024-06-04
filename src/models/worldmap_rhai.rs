use rhai::{
	CustomType,
	TypeBuilder,
};

use super::worldmap::WorldMap;


// Поля, выставляемые в объект скрипта
impl CustomType for WorldMap {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("WorldMap")
			.with_fn("class", WorldMap::show_class)
			.with_fn("object", WorldMap::show_object)
			.on_print(WorldMap::on_print)
		;
	}
}