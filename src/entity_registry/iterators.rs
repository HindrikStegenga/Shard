use super::EntityEntry;
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
        let (_i, entry) = self.entities[self.current..]
            .iter()
            .enumerate()
            .find(|(_i, e)| e.is_valid())?;
        self.current = _i + 1;
        Some(unsafe { Entity::new_unchecked(_i as u32, entry.version()) })
    }
}

impl<'a> FusedIterator for EntityIter<'a> {}
