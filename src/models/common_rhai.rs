use rhai::{
	CustomType,
	TypeBuilder,
};

use super::common::LevelPosition;


// Поля, выставляемые в объект скрипта
impl CustomType for LevelPosition {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("Position")
		;
	}
}