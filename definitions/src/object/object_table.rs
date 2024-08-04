use crate::class::Method;

use super::{class::ClassObjectBody, ArrayObjectBody, NormalObjectBody, Object, Reference, VTable, Deallocate};

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
}

impl ObjectTable {
    pub fn new(method_table: Vec<Method>) -> ObjectTable {
        ObjectTable {
            objects: Vec::new(),
            object_data: Vec::new(),
            classes: Vec::new(),
            vtables: Vec::new(),
            method_table,
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

    pub fn generate_layout_for_object(&self, class_ref: Reference) -> std::alloc::Layout {
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
