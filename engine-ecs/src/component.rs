use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::entity::Entity;

pub trait Component: Any + Send + Sync + 'static {}

impl<T: Any + Send + Sync + 'static> Component for T {}

pub struct ComponentStorage {
    storages: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn register<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.storages.contains_key(&type_id) {
            self.storages.insert(type_id, Box::new(Vec::<Option<T>>::new()));
        }
    }

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let storage = self
            .storages
            .get_mut(&type_id)
            .expect("Component type not registered");

        let vec = storage
            .downcast_mut::<Vec<Option<T>>>()
            .expect("Type mismatch in component storage");

        let id = entity.id() as usize;
        while id >= vec.len() {
            vec.push(None);
        }

        vec[id] = Some(component);
    }

    pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get_mut(&type_id)?;

        let vec = storage
            .downcast_mut::<Vec<Option<T>>>()
            .expect("Type mismatch in component storage");

        let id = entity.id() as usize;
        if id >= vec.len() {
            return None;
        }

        vec[id].take()
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get(&type_id)?;

        let vec = storage
            .downcast_ref::<Vec<Option<T>>>()
            .expect("Type mismatch in component storage");

        let id = entity.id() as usize;
        if id >= vec.len() {
            return None;
        }

        vec[id].as_ref()
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get_mut(&type_id)?;

        let vec = storage
            .downcast_mut::<Vec<Option<T>>>()
            .expect("Type mismatch in component storage");

        let id = entity.id() as usize;
        if id >= vec.len() {
            return None;
        }

        vec[id].as_mut()
    }

    pub fn iter<T: Component>(&self) -> Box<dyn Iterator<Item = (Entity, &T)> + '_> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get(&type_id);

        if let Some(vec) = storage {
            let vec = vec
                .downcast_ref::<Vec<Option<T>>>()
                .expect("Type mismatch in component storage");

            Box::new(vec.iter()
                .enumerate()
                .filter_map(|(id, comp)| comp.as_ref().map(|c| (Entity::new(id as u32, 1), c))))
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn iter_mut<T: Component>(&mut self) -> Box<dyn Iterator<Item = (Entity, &mut T)> + '_> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get_mut(&type_id);

        if let Some(vec) = storage {
            let vec = vec
                .downcast_mut::<Vec<Option<T>>>()
                .expect("Type mismatch in component storage");

            Box::new(vec.iter_mut()
                .enumerate()
                .filter_map(|(id, comp)| comp.as_mut().map(|c| (Entity::new(id as u32, 1), c))))
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl Default for ComponentStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[test]
    fn test_component_storage_insert_get() {
        let mut storage = ComponentStorage::new();
        storage.register::<Position>();

        let entity = Entity::new(0, 1);
        storage.insert(entity, Position { x: 10.0, y: 20.0 });

        let pos = storage.get::<Position>(entity);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap().x, 10.0);
    }

    #[test]
    fn test_component_storage_remove() {
        let mut storage = ComponentStorage::new();
        storage.register::<Position>();

        let entity = Entity::new(0, 1);
        storage.insert(entity, Position { x: 10.0, y: 20.0 });

        let removed = storage.remove::<Position>(entity);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().x, 10.0);

        assert!(storage.get::<Position>(entity).is_none());
    }

    #[test]
    fn test_component_storage_multiple_types() {
        let mut storage = ComponentStorage::new();
        storage.register::<Position>();
        storage.register::<Velocity>();

        let entity = Entity::new(0, 1);
        storage.insert(entity, Position { x: 10.0, y: 20.0 });
        storage.insert(entity, Velocity { dx: 5.0, dy: 3.0 });

        let pos = storage.get::<Position>(entity);
        let vel = storage.get::<Velocity>(entity);

        assert!(pos.is_some());
        assert!(vel.is_some());
        assert_eq!(pos.unwrap().x, 10.0);
        assert_eq!(vel.unwrap().dx, 5.0);
    }

    #[test]
    fn test_component_storage_iter() {
        let mut storage = ComponentStorage::new();
        storage.register::<Position>();

        let e1 = Entity::new(0, 1);
        let e2 = Entity::new(1, 1);
        let e3 = Entity::new(2, 1);

        storage.insert(e1, Position { x: 1.0, y: 2.0 });
        storage.insert(e2, Position { x: 3.0, y: 4.0 });
        storage.insert(e3, Position { x: 5.0, y: 6.0 });

        let count = storage.iter::<Position>().count();
        assert_eq!(count, 3);
    }
}
