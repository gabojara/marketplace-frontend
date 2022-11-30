use std::fmt::Display;

pub trait Entity {
	type Id: PartialEq + Display;
}

use anyhow::Result;
pub trait Repository<E: Entity>: Send + Sync {
	fn find_by_id(&self, id: &E::Id) -> Result<E>;
	fn insert(&self, entity: &E) -> Result<()>;
	fn update(&self, id: &E::Id, entity: &E) -> Result<()>;
	fn upsert(&self, entity: &E) -> Result<()>;
	fn delete(&self, id: &E::Id) -> Result<()>;
	fn clear(&self) -> Result<()>;
}
