use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait System: Any + Send + Sync {
    fn update(&mut self, delta: f32);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct SystemRegistry {
    systems: Vec<Box<dyn System>>,
    system_map: HashMap<TypeId, usize>,
}

impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            system_map: HashMap::new(),
        }
    }

    pub fn register<T: System + 'static>(&mut self, system: T) {
        let type_id = TypeId::of::<T>();
        if self.system_map.contains_key(&type_id) {
            return;
        }

        let index = self.systems.len();
        self.system_map.insert(type_id, index);
        self.systems.push(Box::new(system));
    }

    pub fn get<T: System + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let index = *self.system_map.get(&type_id)?;
        let system = self.systems.get(index)?;
        system.as_any().downcast_ref()
    }

    pub fn get_mut<T: System + 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let index = *self.system_map.get(&type_id)?;
        let system = self.systems.get_mut(index)?;
        system.as_any_mut().downcast_mut()
    }

    pub fn update_all(&mut self, delta: f32) {
        for system in &mut self.systems {
            system.update(delta);
        }
    }

    pub fn len(&self) -> usize {
        self.systems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }
}

impl Default for SystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSystem {
        pub update_count: u32,
    }

    impl TestSystem {
        fn new() -> Self {
            Self { update_count: 0 }
        }
    }

    impl System for TestSystem {
        fn update(&mut self, _delta: f32) {
            self.update_count += 1;
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_system_register() {
        let mut registry = SystemRegistry::new();
        registry.register(TestSystem::new());

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_system_get() {
        let mut registry = SystemRegistry::new();
        registry.register(TestSystem::new());

        let system = registry.get::<TestSystem>();
        assert!(system.is_some());
    }

    #[test]
    fn test_system_get_mut() {
        let mut registry = SystemRegistry::new();
        registry.register(TestSystem::new());

        let system = registry.get_mut::<TestSystem>();
        assert!(system.is_some());
    }

    #[test]
    fn test_system_update() {
        let mut registry = SystemRegistry::new();
        registry.register(TestSystem::new());

        registry.update_all(0.016);

        let system = registry.get::<TestSystem>();
        assert_eq!(system.unwrap().update_count, 1);
    }

    #[test]
    fn test_system_multiple_updates() {
        let mut registry = SystemRegistry::new();
        registry.register(TestSystem::new());

        for _ in 0..5 {
            registry.update_all(0.016);
        }

        let system = registry.get::<TestSystem>();
        assert_eq!(system.unwrap().update_count, 5);
    }
}
