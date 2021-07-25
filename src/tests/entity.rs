#[cfg(test)]
use crate::*;

#[test]
fn test_entity_handles() {
    let mut entity = unsafe { Entity::new(8_000_000, 255) };
    assert_eq!(entity.index(), 8_000_000);
    assert_eq!(entity.version(), 255);

    entity.set_version(20);
    assert_eq!(entity.version(), 20);
    assert_eq!(entity.index(), 8_000_000);

    unsafe {
        entity.set_index(30);
    }
    assert_eq!(entity.index(), 30);
    assert_eq!(entity.version(), 20);

    assert_eq!(Entity::invalid().raw(), u32::MAX);
}
