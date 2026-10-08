use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
    generation: NonZeroU32,
}

impl Entity {
    pub const INVALID: Self = Self {
        id: u32::MAX,
        generation: unsafe { NonZeroU32::new_unchecked(1) },
    };

    pub fn new(id: u32, generation: u32) -> Self {
        Self {
            id,
            generation: NonZeroU32::new(generation).unwrap_or_else(|| unsafe {
                NonZeroU32::new_unchecked(1)
            }),
        }
    }

    pub fn id(self) -> u32 {
        self.id
    }

    pub fn generation(self) -> u32 {
        self.generation.get()
    }

    pub fn is_valid(self) -> bool {
        self.id != u32::MAX
    }
}

#[derive(Debug, Clone)]
pub struct EntityPool {
    free_list: Vec<u32>,
    generations: Vec<u32>,
    next_id: u32,
}

impl EntityPool {
    pub fn new() -> Self {
        Self {
            free_list: Vec::new(),
            generations: Vec::new(),
            next_id: 0,
        }
    }

    pub fn allocate(&mut self) -> Entity {
        let id = if let Some(free_id) = self.free_list.pop() {
            free_id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.generations.push(1);
            id
        };

        let generation = self.generations[id as usize];
        Entity::new(id, generation)
    }

    pub fn deallocate(&mut self, entity: Entity) {
        if !entity.is_valid() {
            return;
        }

        let id = entity.id();
        if id >= self.generations.len() as u32 {
            return;
        }

        self.generations[id as usize] = self.generations[id as usize].wrapping_add(1);
        self.free_list.push(id);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        if !entity.is_valid() {
            return false;
        }

        let id = entity.id();
        if id >= self.generations.len() as u32 {
            return false;
        }

        self.generations[id as usize] == entity.generation()
    }

    /// Returns the current live `Entity` for a given id (i.e. with its correct,
    /// up-to-date generation), or `None` if the id has never been allocated.
    pub fn entity_for_id(&self, id: u32) -> Option<Entity> {
        let generation = *self.generations.get(id as usize)?;
        Some(Entity::new(id, generation))
    }

    pub fn len(&self) -> usize {
        self.next_id as usize - self.free_list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for EntityPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(0, 1);
        assert_eq!(entity.id(), 0);
        assert_eq!(entity.generation(), 1);
        assert!(entity.is_valid());
    }

    #[test]
    fn test_entity_invalid() {
        assert!(!Entity::INVALID.is_valid());
    }

    #[test]
    fn test_pool_allocate() {
        let mut pool = EntityPool::new();
        let e1 = pool.allocate();
        let e2 = pool.allocate();

        assert_eq!(e1.id(), 0);
        assert_eq!(e2.id(), 1);
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_pool_deallocate() {
        let mut pool = EntityPool::new();
        let e1 = pool.allocate();
        pool.deallocate(e1);

        assert_eq!(pool.len(), 0);
        assert!(!pool.is_alive(e1));
    }

    #[test]
    fn test_pool_reuse() {
        let mut pool = EntityPool::new();
        let e1 = pool.allocate();
        pool.deallocate(e1);
        let e2 = pool.allocate();

        assert_eq!(e2.id(), 0);
        assert_ne!(e2.generation(), e1.generation());
    }

    #[test]
    fn test_pool_is_alive() {
        let mut pool = EntityPool::new();
        let e1 = pool.allocate();

        assert!(pool.is_alive(e1));

        pool.deallocate(e1);
        assert!(!pool.is_alive(e1));
    }
}
