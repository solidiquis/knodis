use std::marker::PhantomData;

pub struct EntityCollection<E, DB> {
    entities: Vec<E>,
    database: PhantomData<DB>
}
