use super::entry::EntityEntry;
use crate::Entity;
use core::iter::FusedIterator;

pub struct EntityIter<'a> {
    entities: &'a [EntityEntry],
    current: usize,
}

impl<'a> EntityIter<'a> {
    pub(super) fn new(entities: &'a [EntityEntry]) -> Self {
        Self {
            entities,
            current: 0,
        }
    }
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.entities.len() {
            self.current += 1;
            let entry = &self.entities[self.current - 1];
            if entry.is_valid() {
                return unsafe {
                    Some(Entity::new_unchecked(
                        (self.current - 1) as u32,
                        entry.version(),
                    ))
                };
            }
        }
        None
    }
}

impl<'a> FusedIterator for EntityIter<'a> {}
