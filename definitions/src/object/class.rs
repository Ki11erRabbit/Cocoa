use crate::class::{self, ClassFlags, ClassHeader, FieldFlags, FieldInfo, MethodFlags, PoolEntry, PoolIndex};

use super::Reference;

pub struct PartiallyLoadedClass {
    pub this_info: PoolIndex,
    pub parent_info: PoolIndex,
    pub class_flags: ClassFlags,
    pub constant_pool: Vec<PoolEntry>,
    pub fields: Vec<FieldInfo>,
    pub interfaces: Vec<PoolIndex>,
    pub methods: Vec<class::MethodInfo>,
    pub strings: Vec<PoolIndex>,
}

pub struct StaticFieldInfo {
    pub name: PoolIndex,
    pub flags: FieldFlags,
    pub type_info: PoolIndex,
    pub location: Reference,
}

pub struct InstanceFieldInfo {
    pub name: PoolIndex,
    pub flags: FieldFlags,
    pub type_info: PoolIndex,
}

pub struct MethodInfo {
    pub flags: MethodFlags,
    pub name: PoolIndex,
    pub type_info: PoolIndex,
    pub location: Reference,
}

pub struct ClassObjectBody {
    parent_ref: Reference,
    interface_refs: Vec<Reference>,
    this_info: PoolIndex,
    parent_info: PoolIndex,
    class_flags: ClassFlags,
    constant_pool: Vec<PoolEntry>,
    static_fields: Vec<StaticFieldInfo>,
    instance_fields: Vec<InstanceFieldInfo>,
    interfaces: Vec<PoolIndex>,
    methods: Vec<MethodInfo>,
    strings: Vec<PoolIndex>,
}

impl ClassObjectBody {

    pub fn new(
        parent_ref: Reference,
        interface_refs: Vec<Reference>,
        this_info: PoolIndex,
        parent_info: PoolIndex,
        class_flags: ClassFlags,
        constant_pool: Vec<PoolEntry>,
        static_fields: Vec<FieldInfo>,
        instance_fields: Vec<FieldInfo>,
        interfaces: Vec<PoolIndex>,
        methods: Vec<MethodInfo>,
        strings: Vec<PoolIndex>
    ) -> Self {
        ClassObjectBody {
            parent_ref,
            interface_refs,
            this_info,
            parent_info,
            class_flags,
            constant_pool,
            static_fields,
            instance_fields,
            interfaces,
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


