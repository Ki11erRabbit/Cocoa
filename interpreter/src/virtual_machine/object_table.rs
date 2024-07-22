use once_cell::sync::Lazy;
use definitions::{bytecode::Type, class::{ClassHeader, ClassInfo, PoolEntry}, object::{Array, Object, ObjectHeader, ObjectTable, Reference, StringObject}, RustNativeMethod};

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

    /// TODO: Add array class and create base object
    fn create_array(&self, ty: Type, length: usize) -> Reference {
        let size = match ty {
            Type::U8 | Type::I8 => 1,
            Type::U16 | Type::I16 => 2,
            Type::U32 | Type::I32 | Type::F32 => 4,
            Type::U64 | Type::I64 | Type::F64 => 8,
            Type::Reference => 8,
            _ => panic!("Invalid type for array"),
        };
        
        let array = Array::new(0, 0, size, length);

        self.get_object_table().add_array(array)
    }

    fn get_array(&self, reference: Reference) -> Array {
        self.get_object_table()
            .get_object(reference)
            .expect("Invalid Reference")
            .get_array_ptr()
    }

    // TODO create base object and add string class
    fn create_string(&self, string: String) -> Reference {
        let string = StringObject::new(0, 0,string);

        self.get_object_table().add_string(string)
    }
    fn get_string(&self, reference: Reference) -> StringObject {
        self.get_object_table()
            .get_object(reference)
            .expect("Invalid Reference")
            .get_string_ptr()
    }
    fn is_object(&self, reference: Reference) -> bool {
        self.get_object_table().is_object(reference)
    }
    fn is_array(&self, reference: Reference) -> bool {
        self.get_object_table().is_array(reference)
    }
    fn is_class(&self, reference: Reference) -> bool {
        self.get_object_table().is_class(reference)
    }
    fn is_string(&self, reference: Reference) -> bool {
        self.get_object_table().is_string(reference)
    }
}
