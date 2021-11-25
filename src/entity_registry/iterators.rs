use super::EntityEntry;
use crate::Entity;

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
        if self.current >= self.entities.len() {
            return None;
        }
        let entry = &self.entities[self.current];
        return if entry.is_valid() {
            self.current += 1;
            Some(unsafe { Entity::new_unchecked(self.current as u32, entry.version()) })
        } else {
            let (i, entry) = self.entities[self.current + 1..]
                .iter()
                .enumerate()
                .find(|(i, e)| e.is_valid())?;
            self.current = i;
            Some(unsafe { Entity::new_unchecked(self.current as u32, entry.version()) })
        };
    }
}
