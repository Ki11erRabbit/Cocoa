use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::class::ClassHeader;


struct ObjectBody {
    parent: Reference,
    class_ref: Reference,
    field_count: usize,
}

impl ObjectBody {
    fn new(parent: Reference, class_ref: Reference, fields: usize) -> *mut Self {
        let layout = std::alloc::Layout::new::<Self>();
        let (layout, _) = layout.extend(std::alloc::Layout::array::<Reference>(fields).unwrap()).unwrap();
        let object = unsafe {std::alloc::alloc(layout)};
        let object = object as *mut Self;
        unsafe {
            std::ptr::write(object, Self {
                parent,
                class_ref,
                field_count: fields,
            });
        }
        object
    }

    fn get_parent(&self) -> Reference {
        self.parent
    }

    fn get_class(&self) -> Reference {
        self.class_ref
    }

    fn get_field<T:Copy>(&self, index: usize) -> T {
        let ptr = self as *const Self;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const Reference;
        assert!(index < self.field_count);
        let ptr = unsafe { ptr.add(index) as *const T };
        unsafe {*ptr}
    }

    fn set_field<T:Copy>(&self, index: usize, value: T) {
        let ptr = self as *const Self;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const Reference;
        assert!(index < self.field_count);
        let ptr = unsafe { ptr.add(index) as *mut T };
        unsafe {*ptr = value};
    }
}

#[derive(Debug, Copy, PartialEq, Eq)]
pub struct Object(*mut ObjectBody);

impl Object {
    pub fn new(parent: Reference, class_ref: Reference, fields: usize) -> Self {
        let body = ObjectBody::new(parent, class_ref, fields);

        Object(body)
    }

    pub fn get_parent(&self) -> Reference {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_parent()
    }

    pub fn get_class(&self) -> Reference {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_class()
    }

    pub fn get_size(&self) -> usize {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.field_count
    }

    pub fn get_field<T:Copy>(&self, index: usize) -> T {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_field(index)
    }

    pub fn set_field<T:Copy>(&self, index: usize, value: T) {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.set_field(index, value);
    }

    pub fn deallocate(&mut self) {
        println!("Dropping Object");
        let field_count = unsafe {self.0.as_ref().unwrap().field_count};
        let layout = std::alloc::Layout::new::<Self>();
        let (layout, _) = layout.extend(std::alloc::Layout::array::<Reference>(field_count).unwrap()).unwrap();
        unsafe {
            //std::ptr::drop_in_place(self);
            std::alloc::dealloc(self.0 as *mut u8, layout);
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object(self.0)
    }
}

struct ArrayBody {
    parent: Reference,
    class_ref: Reference,
    elem_size: usize,
    size: usize,
}

impl ArrayBody {
    fn new(parent: Reference, class_ref: Reference, elem_size: usize, size: usize) -> *mut Self {
        let layout = std::alloc::Layout::new::<Self>();
        
        let (layout, _) = match elem_size {
            1 => layout.extend(std::alloc::Layout::array::<u8>(size).unwrap()).unwrap(),
            2 => layout.extend(std::alloc::Layout::array::<u16>(size).unwrap()).unwrap(),
            4 => layout.extend(std::alloc::Layout::array::<u32>(size).unwrap()).unwrap(),
            8 => layout.extend(std::alloc::Layout::array::<u64>(size).unwrap()).unwrap(),
            _ => panic!("Invalid element size"),
        };
        let object = unsafe {std::alloc::alloc(layout)};
        let object = object as *mut Self;
        unsafe {
            std::ptr::write(object, Self {
                parent,
                class_ref,
                elem_size,
                size,
            });
        }
        object
    }

    fn get_parent(&self) -> Reference {
        self.parent
    }

    fn get_class(&self) -> Reference {
        self.class_ref
    }

    fn get_elem_size(&self) -> usize {
        self.elem_size
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_elem<T:Copy>(&self, index: usize) -> T {
        use std::mem::size_of;
        let ptr = self as *const Self;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const T;
        assert!(index < self.size);
        assert!(size_of::<T>() == self.elem_size);
        let ptr = unsafe { ptr.add(index) };
        unsafe {*ptr}
    }

    fn set_elem<T:Copy>(&mut self, index: usize, value: T) {
        use std::mem::size_of;
        let ptr = self as *const Self;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut T;
        assert!(index < self.size);
        assert!(size_of::<T>() == self.elem_size);
        let ptr = unsafe { ptr.add(index) };
        unsafe {*ptr = value};
    }
}

#[derive(Debug, Copy, PartialEq, Eq)]
pub struct Array(*mut ArrayBody);

impl Array {
    pub fn new(parent: Reference, class_ref: Reference, elem_size: usize, size: usize) -> Self {
        let body = ArrayBody::new(parent, class_ref, elem_size, size);

        Array(body)
    }

    pub fn get_parent(&self) -> Reference {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_parent()
    }

    pub fn get_class(&self) -> Reference {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_class()
    }

    pub fn get_elem_size(&self) -> usize {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_elem_size()
    }

    pub fn get_size(&self) -> usize {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_size()
    }

    pub fn get_elem<T:Copy>(&self, index: usize) -> T {
        let body = unsafe {self.0.as_ref().unwrap()};
        body.get_elem(index)
    }

    pub fn set_elem<T:Copy>(&mut self, index: usize, value: T) {
        let body = unsafe {self.0.as_mut().unwrap()};
        body.set_elem(index, value);
    }

    pub fn deallocate(&mut self) {
        println!("Dropping Array");
        let size = unsafe {self.0.as_ref().unwrap().size};
        let elem_size = unsafe {self.0.as_ref().unwrap().elem_size};
        let layout = std::alloc::Layout::new::<Self>();
        let (layout, _) = match elem_size {
            1 => layout.extend(std::alloc::Layout::array::<u8>(size).unwrap()).unwrap(),
            2 => layout.extend(std::alloc::Layout::array::<u16>(size).unwrap()).unwrap(),
            4 => layout.extend(std::alloc::Layout::array::<u32>(size).unwrap()).unwrap(),
            8 => layout.extend(std::alloc::Layout::array::<u64>(size).unwrap()).unwrap(),
            _ => panic!("Invalid element size"),
        };
        unsafe {
            //std::ptr::drop_in_place(self);
            std::alloc::dealloc(self.0 as *mut u8, layout);
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Array(self.0)
    }
}

pub type Reference = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcMark {
    White,
    Gray,
    Black,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HeaderPtr {
    Object(Object),
    Array(Array),
    Class(ClassHeader),
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectHeader {
    mark: GcMark,
    ptr: HeaderPtr,
}

impl ObjectHeader {
    pub fn new_object(ptr: Object) -> Self {
        Self {
            mark: GcMark::White,
            ptr: HeaderPtr::Object(ptr),
        }
    }

    pub fn new_array(ptr: Array) -> Self {
        Self {
            mark: GcMark::White,
            ptr: HeaderPtr::Array(ptr),
        }
    }

    pub fn new_class(ptr: ClassHeader) -> Self {
        Self {
            mark: GcMark::White,
            ptr: HeaderPtr::Class(ptr),
        }
    }
    
    pub fn get_object_ptr(&self) -> Object {
        match &self.ptr {
            HeaderPtr::Object(ptr) => ptr.clone(),
            _ => panic!("Invalid object type"),
        }
    }

    pub fn get_array_ptr(&self) -> Array {
        match &self.ptr {
            HeaderPtr::Array(ptr) => ptr.clone(),
            _ => panic!("Invalid object type"),
        }
    }

    pub fn get_class_ptr(&self) -> ClassHeader {
        match &self.ptr {
            HeaderPtr::Class(ptr) => ptr.clone(),
            _ => panic!("Invalid object type"),
        }
    }

    pub fn get_mark(&self) -> GcMark {
        self.mark
    }

    pub fn set_mark(&mut self, mark: GcMark) {
        self.mark = mark;
    }

    pub fn deallocate(&mut self) {
        match self.ptr {
            HeaderPtr::Array(mut array) => array.deallocate(),
            HeaderPtr::Class(mut class) => class.deallocate(),
            HeaderPtr::Object(mut obj) => obj.deallocate(),
        }
    }
}



pub struct ObjectTable {
    objects: RwLock<Vec<Option<ObjectHeader>>>,
}

impl ObjectTable {
    pub fn new() -> Self {
        Self {
            objects: RwLock::new(Vec::new()),
        }
    }

    pub fn add_object(&self, object: Object) -> Reference {
        let mut objects = self.objects.write().unwrap();
        let ptr = objects.len();
        objects.push(Some(ObjectHeader::new_object(object)));
        ptr
    }

    pub fn add_array(&self, array: Array) -> Reference {
        let mut objects = self.objects.write().unwrap();
        let ptr = objects.len();
        objects.push(Some(ObjectHeader::new_array(array)));
        ptr
    }

    pub fn add_class(&self, class: ClassHeader) -> Reference {
        let mut objects = self.objects.write().unwrap();
        let ptr = objects.len();
        objects.push(Some(ObjectHeader::new_class(class)));
        ptr
    }

    pub fn get_object(&self, reference: Reference) -> Option<ObjectHeader> {
        self.objects.read().unwrap().get(reference).cloned().flatten()
    }

    pub fn delete_object(&self, reference: Reference) {
        let mut table = self.objects.write().unwrap();
        table[reference].as_mut().map(|obj| {
            obj.deallocate();
        });
        table[reference] = None;
    }

    pub fn get_table(&self) -> RwLockReadGuard<Vec<Option<ObjectHeader>>> {
        self.objects.read().unwrap()
    }

    pub fn get_table_mut(&self) -> RwLockWriteGuard<Vec<Option<ObjectHeader>>> {
        self.objects.write().unwrap()
    }
}

unsafe impl Sync for ObjectTable {}
unsafe impl Send for ObjectTable {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_table() {
        let table = ObjectTable::new();
        let object = Object::new(0, 0, 0);
        let reference = table.add_object(object.clone());
        assert_eq!(table.get_object(reference).unwrap().get_object_ptr(), object);
        table.delete_object(reference);
        assert_eq!(table.get_object(reference), None);
    }

    #[test]
    fn test_object() {
        let object = Object::new(0, 0, 0);
        assert_eq!(object.get_parent(), 0);
        assert_eq!(object.get_class(), 0);
    }

    #[test]
    fn test_object_header() {
        let object = Object::new(0, 0, 0);
        let mut header = ObjectHeader::new_object(object.clone());
        assert_eq!(header.get_object_ptr(), object);
        assert_eq!(header.get_mark(), GcMark::White);
        header.set_mark(GcMark::Black);
        assert_eq!(header.get_mark(), GcMark::Black);
    }

    #[test]
    fn test_array() {
        let array = Array::new(0, 0, 4, 4);
        assert_eq!(array.get_parent(), 0);
        assert_eq!(array.get_class(), 0);
        assert_eq!(array.get_elem_size(), 4);
        assert_eq!(array.get_size(), 4);
    }

    #[test]
    fn test_object_body() {
        let object = Object::new(0, 0, 4);
        object.set_field(0, 1);
        object.set_field(1, 2);
        object.set_field(2, 3);
        object.set_field(3, 4);
        assert_eq!(object.get_field::<usize>(0), 1);
        assert_eq!(object.get_field::<usize>(1), 2);
        assert_eq!(object.get_field::<usize>(2), 3);
        assert_eq!(object.get_field::<usize>(3), 4);
    }

    #[test]
    fn test_array_body() {
        let mut array = Array::new(0, 0, 8, 4);
        array.set_elem::<usize>(0, 1);
        array.set_elem::<usize>(1, 2);
        array.set_elem::<usize>(2, 3);
        array.set_elem::<usize>(3, 4);
        assert_eq!(array.get_elem::<usize>(0), 1);
        assert_eq!(array.get_elem::<usize>(1), 2);
        assert_eq!(array.get_elem::<usize>(2), 3);
        assert_eq!(array.get_elem::<usize>(3), 4);
    }

    #[test]
    fn test_object_drop() {
        let object = Object::new(0, 0, 4);
    }
}
