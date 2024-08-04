use crate::class::{ClassFlags, ClassHeader, FieldInfo, MethodInfo, PoolEntry, PoolIndex};

use super::Reference;




pub struct ClassObjectBody {
    parent_ref: Reference,
    this_info: PoolIndex,
    parent_info: PoolIndex,
    class_flags: ClassFlags,
    constant_pool: Vec<PoolEntry>,
    static_fields: Vec<FieldInfo>,
    instance_fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    strings: Vec<PoolIndex>,
}

impl ClassObjectBody {
    pub fn new(this_info: PoolIndex, parent_info: PoolIndex, class_flags: ClassFlags, constant_pool: Vec<PoolEntry>, static_fields: Vec<FieldInfo>, instance_fields: Vec<FieldInfo>, methods: Vec<MethodInfo>, strings: Vec<PoolIndex>) -> ClassObjectBody {
        ClassObjectBody {
            parent_ref: 0,
            this_info,
            parent_info,
            class_flags,
            constant_pool,
            static_fields,
            instance_fields,
            methods,
            strings,
        }
    }

    pub fn get_parent_ref(&self) -> Reference {
        self.parent_ref
    }
    pub fn set_parent_ref(&mut self, parent_ref: Reference) {
        self.parent_ref = parent_ref;
    }

    pub fn get_this_info(&self) -> PoolIndex {
        self.this_info
    }

    pub fn get_parent_info(&self) -> PoolIndex {
        self.parent_info
    }

    pub fn get_class_flags(&self) -> ClassFlags {
        self.class_flags
    }

    pub fn get_constant_pool(&self) -> &Vec<PoolEntry> {
        &self.constant_pool
    }

    pub fn get_static_fields(&self) -> &Vec<FieldInfo> {
        &self.static_fields
    }

    pub fn get_instance_fields(&self) -> &Vec<FieldInfo> {
        &self.instance_fields
    }

    pub fn get_methods(&self) -> &Vec<MethodInfo> {
        &self.methods
    }

    pub fn get_strings(&self) -> &Vec<PoolIndex> {
        &self.strings
    }
}


impl From<ClassHeader> for ClassObjectBody {
    fn from(header: ClassHeader) -> Self {
        let mut static_fields = Vec::new();
        let mut instance_fields = Vec::new();

        for field in header.fields.into_iter() {
            if field.is_static() {
                static_fields.push(field.clone());
            } else {
                instance_fields.push(field.clone());
            }
        }
        
        ClassObjectBody {
            parent_ref: 0,
            this_info: header.this_info,
            parent_info: header.parent_info,
            class_flags: header.class_flags,
            constant_pool: header.constant_pool,
            static_fields,
            instance_fields,
            methods: header.methods,
            strings: header.strings,
        }
    }
}
