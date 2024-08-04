use std::collections::HashMap;

use crate::class::{ClassHeader, Method, PoolEntry};

use super::{class::{ClassObjectBody, PartiallyLoadedClass}, ArrayObjectBody, Deallocate, NormalObjectBody, Object, Reference, VTable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GCMark {
    White,
    Gray,
    Black,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ObjectTableHeader {
    Live {
        mark: GCMark,
        object: *mut Object,
    },
    Dead,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ObjectDataHeader {
    Object {
        object: *mut NormalObjectBody,
    },
    Array {
        object: *mut ArrayObjectBody,
    },
    Dead,
}



#[derive(Debug)]
pub struct ObjectTable {
    objects: Vec<ObjectTableHeader>,
    object_data: Vec<ObjectDataHeader>,
    classes: Vec<*mut ClassObjectBody>,
    vtables: Vec<*const VTable>,
    method_table: Vec<Method>,
    symbol_table: Vec<String>,
}

impl ObjectTable {
    pub fn new(method_table: Vec<Method>) -> ObjectTable {
        ObjectTable {
            objects: Vec::new(),
            object_data: Vec::new(),
            classes: Vec::new(),
            vtables: Vec::new(),
            method_table,
            symbol_table: Vec::new(),
        }
    }

    pub fn get_object(&self, reference: Reference) -> Option<*mut Object> {
        match self.objects[reference as usize] {
            ObjectTableHeader::Live { object, .. } => Some(object),
            ObjectTableHeader::Dead => None,
        }
    }

    pub fn get_object_data(&self, reference: Reference) -> Option<ObjectDataHeader> {
        match self.object_data[reference as usize] {
            ObjectDataHeader::Dead => None,
            data => Some(data),
        }
    }

    pub fn get_vtable(&self, class_ref: Reference) -> *const VTable {
        self.vtables[class_ref as usize]
    }

    pub fn get_method(&self, method_ref: Reference) -> Method {
        self.method_table[method_ref as usize].clone()
    }

    pub fn load_class(&mut self, class: ClassHeader, mapper: &mut dyn Mapper) -> Result<Reference, PartiallyLoadedClass> {
        let ClassHeader { this_info, parent_info, class_flags, constant_pool, interfaces, fields, methods, strings } = class;

        let PoolEntry::ClassInfo(parent_class) = &constant_pool[parent_info] else {
            panic!("Invalid parent class reference");
        };
        let PoolEntry::Symbol(parent_name) = &constant_pool[parent_class.name] else {
            panic!("Invalid parent class name");
        };

        if !mapper.contains_symbol(parent_name) {
            return Err(PartiallyLoadedClass {
                this_info,
                parent_info,
                class_flags,
                constant_pool,
                interfaces,
                fields,
                methods,
                strings,
            });
        }

        for interfaces_index in interfaces.iter() {
            let PoolEntry::ClassInfo(parent_class) = &constant_pool[interfaces_index] else {
                panic!("Invalid parent class reference");
            };
            let PoolEntry::Symbol(parent_name) = &constant_pool[parent_class.name] else {
                panic!("Invalid parent class name");
            };

            if !mapper.contains_symbol(parent_name) {
                return Err(PartiallyLoadedClass {
                    this_info,
                    parent_info,
                    class_flags,
                    constant_pool,
                    interfaces,
                    fields,
                    methods,
                    strings,
                });
            }
        }
        
        let mut new_constant_pool = Vec::new();
        for entry in constant_pool {
            if let PoolEntry::Symbol(sym) = entry {
                if !mapper.contains_symbol(&sym) {
                    self.symbol_table.push(sym.clone());
                    let index = self.symbol_table.len() - 1;
                    mapper.insert(sym, index);
                    new_constant_pool.push(PoolEntry::Reference(index));
                } else {
                    new_constant_pool.push(PoolEntry::Reference(mapper.get_symbol(&sym).expect("Symbol not found").clone()));
                }
            } else {
                new_constant_pool.push(entry);
            }
        }

        for method in methods.iter_mut() {
            
        }
        
        
        
        
        Ok(9)
    }

    pub fn load_partially_loaded_class(&mut self, class: ClassHeader) -> Result<Reference, PartiallyLoadedClass> {
        
    }

    #[inline]
    pub fn generate_size_for_object(&self, class_ref: Reference) -> usize {
        let class = self.classes[class_ref as usize];
        let class = unsafe { &*class };
        let mut size = class.get_instance_fields().len();
        let mut parent_ref = class.get_parent_ref();
        while parent_ref != 0 {
            let parent = self.classes[parent_ref as usize];
            let parent = unsafe { &*parent };
            size += parent.get_instance_fields().len();
            parent_ref = parent.get_parent_ref();
        }

        size
    }
    
    pub fn generate_layout_for_object(&self, class_ref: Reference) -> std::alloc::Layout {
        let size = self.generate_size_for_object(class_ref);

        std::alloc::Layout::array::<u64>(size).expect("Layout Overflowed")
    }

    pub fn generate_layout_for_array(&self, array_ref: Reference) -> std::alloc::Layout {
        let array = self.object_data[array_ref];
        if let ObjectDataHeader::Array { object } = array {
            let array = unsafe { &*object };
            let size = array.len() * array.elem_size();
            let layout = std::alloc::Layout::new::<u64>();
            let (layout, _) = layout.extend(std::alloc::Layout::new::<u64>()).expect("Layout Overflowed");
            let (layout, _) = layout.extend(std::alloc::Layout::array::<u8>(size as usize).expect("Layout Overflowed")).expect("Layout Overflowed");
            layout
        } else {
            panic!("Invalid array reference");
        }
    }

    pub fn collect_garbage(&mut self) {
        let mut object_data_to_free = Vec::new();
        for object in self.objects.iter_mut() {
            if let ObjectTableHeader::Live { mark, object: obj} = object {
                if *mark == GCMark::White {
                    let obj = unsafe { &mut **obj };
                    object_data_to_free.push((obj.class_ref, obj.data_ref));
                    
                    *object = ObjectTableHeader::Dead;
                }
            }
        }

        for (class_ref, data_ref) in object_data_to_free {
            if let ObjectDataHeader::Object { object } = self.object_data[data_ref] {
                let object = unsafe { &mut *object };
                object.deallocate(self.generate_layout_for_object(class_ref));
            } else if let ObjectDataHeader::Array { object } = self.object_data[data_ref] {
                let object = unsafe { &mut *object };
                object.deallocate(self.generate_layout_for_array(data_ref));
            }
            
            self.object_data[data_ref] = ObjectDataHeader::Dead;
        }
    }
}
