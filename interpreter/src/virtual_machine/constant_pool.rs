use std::sync::RwLock;

use definitions::class::{PoolEntry, PoolIndex};
use once_cell::sync::Lazy;

use super::machine::ConstantPool;


static CONSTANT_POOL: Lazy<RawConstantPool> = Lazy::new(|| {
    RawConstantPool::new()
});

#[derive(Debug)]
struct RawConstantPool {
    pool: RwLock<Vec<PoolEntry>>,
}

impl RawConstantPool {
    pub fn new() -> Self {
        Self {
            pool: RwLock::new(Vec::new()),
        }
    }
    fn add_constant(&self, entry: PoolEntry) -> PoolIndex {
        let mut pool = self.pool.write().expect("Constant pool poisoned");
        let index = pool.len();
        pool.push(entry);
        index
    }

    fn set_constant(&self, index: PoolIndex, entry: PoolEntry) {
        self.pool.write().expect("Constant pool poisoned").insert(index, entry);
    }

    fn get_constant(&self, index: PoolIndex) -> PoolEntry {
        self.pool.read().expect("Constant pool poisoned").get(index).unwrap().clone()
    }
}

pub struct ConstantPoolSingleton {}

impl ConstantPoolSingleton {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConstantPool for ConstantPoolSingleton {
    fn add_constant(&self, entry: PoolEntry) -> PoolIndex {
        CONSTANT_POOL.add_constant(entry)
    }

    fn set_constant(&self, index: PoolIndex, entry: PoolEntry) {
        CONSTANT_POOL.set_constant(index, entry)
    }

    fn get_constant(&self, index: PoolIndex) -> PoolEntry {
        CONSTANT_POOL.get_constant(index)
    }
}
