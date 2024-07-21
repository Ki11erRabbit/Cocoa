use once_cell::sync::Lazy;
use definitions::{class::{ClassHeader, ClassInfo, PoolEntry}, object::{Object, ObjectTable, Reference}, RustNativeMethod};

use super::machine;

static OBJECT_TABLE: Lazy<ObjectTable> = Lazy::new(|| {
    ObjectTable::new()
});

pub struct ObjectTableSingleton {}

impl ObjectTableSingleton {
    pub fn get_singleton() -> Self {
        Self {} 
    }

    fn get_object_table<'a>(&'a self) -> &'a ObjectTable {
        &OBJECT_TABLE
    }

}

impl machine::ObjectTable for ObjectTableSingleton {
    fn create_object(&self, class_ref: Reference) -> Reference {
        let class = self.get_object_table()
            .get_object(class_ref)
            .expect("Invalid Reference")
            .get_class_ptr();

        let parent_info_index = class.get_parent_info();
        let parent_info = class.get_constant_pool_entry(parent_info_index);
        let parent_reference = match parent_info {
            PoolEntry::ClassInfo(ClassInfo {class_ref: Some(class_ref), ..}) => {
                self.create_object(*class_ref)
            }
            PoolEntry::ClassInfo(ClassInfo {class_ref: None, ..}) => {
                0
            }
            _ => panic!("Entry was not a class info"),
        };

        // TODO: make this not include static members
        let field_count = class.fields_count();

        let object = Object::new(parent_reference, class_ref, field_count);

        self.get_object_table().add_object(object)
    }

    fn add_class(&self, class: ClassHeader) -> Reference {
        self.get_object_table().add_class(class)
    }

    fn get_object(&self, reference: Reference) -> Object {
        self.get_object_table()
            .get_object(reference)
            .expect("Invalid Reference")
            .get_object_ptr()
    }

    fn get_class(&self, reference: Reference) -> ClassHeader {
        self.get_object_table()
            .get_object(reference)
            .expect("Invalid Reference")
            .get_class_ptr()
    }

}
