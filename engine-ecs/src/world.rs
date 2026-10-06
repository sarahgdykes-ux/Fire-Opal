use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::entity::{Entity, EntityPool};
use crate::component::ComponentStorage;
use crate::system::{System, SystemRegistry};

pub struct World {
    entities: EntityPool,
    components: ComponentStorage,
    systems: SystemRegistry,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntityPool::new(),
            components: ComponentStorage::new(),
            systems: SystemRegistry::new(),
            resources: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entities.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities.deallocate(entity);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    pub fn register_component<T: crate::component::Component>(&mut self) {
        self.components.register::<T>();
    }

    pub fn add_component<T: crate::component::Component>(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }

    pub fn remove_component<T: crate::component::Component>(&mut self, entity: Entity) -> Option<T> {
        self.components.remove(entity)
    }

    pub fn get_component<T: crate::component::Component>(&self, entity: Entity) -> Option<&T> {
        self.components.get(entity)
    }

    pub fn get_component_mut<T: crate::component::Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(entity)
    }

    pub fn iter_components<T: crate::component::Component>(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.components.iter()
    }

    pub fn iter_components_mut<T: crate::component::Component>(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.components.iter_mut()
    }

    pub fn register_system<T: System + 'static>(&mut self, system: T) {
        self.systems.register(system);
    }

    pub fn get_system<T: System + 'static>(&self) -> Option<&T> {
        self.systems.get()
    }

    pub fn get_system_mut<T: System + 'static>(&mut self) -> Option<&mut T> {
        self.systems.get_mut()
    }

    pub fn update(&mut self, delta: f32) {
        self.systems.update_all(delta);
    }

    pub fn insert_resource<T: Any + Send + Sync + 'static>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(resource));
    }

    pub fn get_resource<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let resource = self.resources.get(&type_id)?;
        resource.downcast_ref()
    }

    pub fn get_resource_mut<T: Any + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let resource = self.resources.get_mut(&type_id)?;
        resource.downcast_mut()
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    struct MovementSystem;

    impl System for MovementSystem {
        fn update(&mut self, _delta: f32) {}

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_world_create_entity() {
        let mut world = World::new();
        let entity = world.create_entity();

        assert!(world.is_alive(entity));
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_world_destroy_entity() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.destroy_entity(entity);

        assert!(!world.is_alive(entity));
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn test_world_add_component() {
        let mut world = World::new();
        world.register_component::<Position>();

        let entity = world.create_entity();
        world.add_component(entity, Position { x: 10.0, y: 20.0 });

        let pos = world.get_component::<Position>(entity);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap().x, 10.0);
    }

    #[test]
    fn test_world_remove_component() {
        let mut world = World::new();
        world.register_component::<Position>();

        let entity = world.create_entity();
        world.add_component(entity, Position { x: 10.0, y: 20.0 });

        let removed = world.remove_component::<Position>(entity);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().x, 10.0);

        assert!(world.get_component::<Position>(entity).is_none());
    }

    #[test]
    fn test_world_register_system() {
        let mut world = World::new();
        world.register_system(MovementSystem);

        let system = world.get_system::<MovementSystem>();
        assert!(system.is_some());
    }

    #[test]
    fn test_world_resource() {
        let mut world = World::new();
        world.insert_resource(42u32);

        let resource = world.get_resource::<u32>();
        assert!(resource.is_some());
        assert_eq!(*resource.unwrap(), 42);
    }

    #[test]
    fn test_world_resource_mut() {
        let mut world = World::new();
        world.insert_resource(42u32);

        let resource = world.get_resource_mut::<u32>();
        assert!(resource.is_some());
        *resource.unwrap() = 100;

        assert_eq!(*world.get_resource::<u32>().unwrap(), 100);
    }
}
