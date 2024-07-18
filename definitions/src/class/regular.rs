use crate::class::{FieldInfo, MethodInfo, PoolEntry};

use super::{PoolIndex, ClassFlags};




struct ClassHeaderPart1 {
    this_info: PoolIndex,
    parent_info: PoolIndex,
    class_flags: ClassFlags,
    contant_pool_size: usize,
}


struct ClassHeaderPart2 {
    interfaces_count: usize,
}

struct ClassHeaderPart3 {
    fields_count: usize,
}

struct ClassHeaderPart4 {
    methods_count: usize,
}

pub struct ClassHeader(*const ());


impl ClassHeader {
    pub fn new(constant_pool_size: usize, interfaces_count: usize, fields_count: usize, methods_count: usize) -> Self {
        use std::alloc::{alloc, Layout};
        let layout = Layout::new::<ClassHeaderPart1>();
        let (layout, _) = layout.extend(Layout::array::<PoolEntry>(constant_pool_size).unwrap()).unwrap();
        let (layout, _) = layout.extend(Layout::new::<ClassHeaderPart2>()).unwrap();
        let (layout, _) = layout.extend(Layout::array::<PoolIndex>(interfaces_count).unwrap()).unwrap();
        let (layout, _) = layout.extend(Layout::new::<ClassHeaderPart3>()).unwrap();
        let (layout, _) = layout.extend(Layout::array::<FieldInfo>(fields_count).unwrap()).unwrap();
        let (layout, _) = layout.extend(Layout::new::<ClassHeaderPart4>()).unwrap();
        let (layout, _) = layout.extend(Layout::array::<MethodInfo>(methods_count).unwrap()).unwrap();

        let ptr = unsafe { alloc(layout) };
        let og_ptr = ptr;
        let ptr = ptr as *mut ClassHeaderPart1;
        unsafe {
            ptr.write(ClassHeaderPart1 {
                this_info: 0,
                parent_info: 0,
                class_flags: ClassFlags::empty(),
                contant_pool_size: constant_pool_size,
            });
        }
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolEntry;
        let ptr = unsafe { ptr.add(constant_pool_size) };
        let ptr = ptr as *mut ClassHeaderPart2;
        unsafe {
            ptr.write(ClassHeaderPart2 {
                interfaces_count,
            });
        }
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolIndex;
        let ptr = unsafe { ptr.add(interfaces_count) };
        let ptr = ptr as *mut ClassHeaderPart3;
        unsafe {
            ptr.write(ClassHeaderPart3 {
                fields_count,
            });
        }
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut FieldInfo;
        let ptr = unsafe { ptr.add(fields_count) };
        let ptr = ptr as *mut ClassHeaderPart4;
        unsafe {
            ptr.write(ClassHeaderPart4 {
                methods_count,
            });
        }

        ClassHeader(og_ptr as *const ())
    }

    pub fn set_this_info(&mut self, this_info: PoolIndex) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        unsafe {
            let old = ptr.read();
            ptr.write(ClassHeaderPart1 {
                this_info,
                ..old
            });
        }
    }

    pub fn get_this_info(&self) -> PoolIndex {
        let ptr = self.0 as *const ClassHeaderPart1;
        unsafe {
            ptr.read().this_info
        }
    }

    pub fn set_parent_info(&mut self, parent_info: PoolIndex) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        unsafe {
            let old = ptr.read();
            ptr.write(ClassHeaderPart1 {
                parent_info,
                ..old
            });
        }
    }

    pub fn get_parent_info(&self) -> PoolIndex {
        let ptr = self.0 as *const ClassHeaderPart1;
        unsafe {
            ptr.read().parent_info
        }
    }

    pub fn set_class_flags(&mut self, class_flags: ClassFlags) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        unsafe {
            let old = ptr.read();
            ptr.write(ClassHeaderPart1 {
                class_flags,
                ..old
            });
        }
    }

    pub fn get_class_flags(&self) -> ClassFlags {
        let ptr = self.0 as *const ClassHeaderPart1;
        unsafe {
            ptr.read().class_flags
        }
    }

    pub fn constant_pool_len(&self) -> usize {
        let ptr = self.0 as *const ClassHeaderPart1;
        unsafe {
            ptr.read().contant_pool_size
        }
    }

    pub fn interfaces_count(&self) -> usize {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *const ClassHeaderPart2;
        unsafe {
            ptr.read().interfaces_count
        }
    }

    pub fn methods_count(&self) -> usize {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *const ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolIndex;
        let ptr = unsafe { ptr.add(self.interfaces_count()) };
        let ptr = ptr as *const ClassHeaderPart3;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const FieldInfo;
        let ptr = unsafe { ptr.add(self.fields_count()) };
        let ptr = ptr as *const ClassHeaderPart4;
        unsafe {
            ptr.read().methods_count
        }
    }

    pub fn fields_count(&self) -> usize {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *const ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolIndex;
        let ptr = unsafe { ptr.add(self.interfaces_count()) };
        let ptr = ptr as *const ClassHeaderPart3;
        unsafe {
            ptr.read().fields_count
        }
    }

    pub fn set_constant_pool_entry(&mut self, index: usize, entry: PoolEntry) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolEntry;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.write(entry);
        }
    }

    pub fn get_constant_pool_entry(&self, index: usize) -> PoolEntry {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.read()
        }
    }

    pub fn set_interface(&mut self, index: usize, interface: PoolIndex) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *mut ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolIndex;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.write(interface);
        }
    }

    pub fn get_interface(&self, index: usize) -> PoolIndex {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *const ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolIndex;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.read()
        }
    }

    pub fn set_field(&mut self, index: usize, field: FieldInfo) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *mut ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolIndex;
        let ptr = unsafe { ptr.add(self.interfaces_count()) };
        let ptr = ptr as *mut ClassHeaderPart3;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut FieldInfo;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.write(field);
        }
    }

    pub fn get_field(&self, index: usize) -> FieldInfo {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *const ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolIndex;
        let ptr = unsafe { ptr.add(self.interfaces_count()) };
        let ptr = ptr as *const ClassHeaderPart3;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const FieldInfo;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.read()
        }
    }

    pub fn set_method(&mut self, index: usize, method: MethodInfo) {
        let ptr = self.0 as *mut ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *mut ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut PoolIndex;
        let ptr = unsafe { ptr.add(self.interfaces_count()) };
        let ptr = ptr as *mut ClassHeaderPart3;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut FieldInfo;
        let ptr = unsafe { ptr.add(self.fields_count()) };
        let ptr = ptr as *mut ClassHeaderPart4;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *mut MethodInfo;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.write(method);
        }
    }

    pub fn get_method(&self, index: usize) -> MethodInfo {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(self.constant_pool_len()) };
        let ptr = ptr as *const ClassHeaderPart2;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolIndex;
        let ptr = unsafe { ptr.add(self.interfaces_count()) };
        let ptr = ptr as *const ClassHeaderPart3;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const FieldInfo;
        let ptr = unsafe { ptr.add(self.fields_count()) };
        let ptr = ptr as *const ClassHeaderPart4;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const MethodInfo;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.read()
        }
    }

    
}

#[cfg(test)]
mod tests {
    use crate::class::FieldFlags;
    use crate::class::{FieldInfo, MethodInfo, PoolEntry, MethodFlags};
    use super::*;

    #[test]
    fn test_class_header_creation() {
        let _ = ClassHeader::new(10, 5, 3, 4);

    }

    #[test]
    fn test_class_header_set_this_info() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_this_info(5);
        assert_eq!(header.get_this_info(), 5);
    }

    #[test]
    fn test_class_header_set_parent_info() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_parent_info(5);
        assert_eq!(header.get_parent_info(), 5);
    }

    #[test]
    fn test_class_header_set_class_flags() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_class_flags(ClassFlags::Public);
        assert_eq!(header.get_class_flags(), ClassFlags::Public);
    }

    #[test]
    fn test_class_header_set_constant_pool_entry() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_constant_pool_entry(5, PoolEntry::String("Hello"));
        assert_eq!(header.get_constant_pool_entry(5), PoolEntry::String("Hello"));
    }

    #[test]
    fn test_class_header_set_interface() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_interface(5, 10);
        assert_eq!(header.get_interface(5), 10);
    }

    #[test]
    fn test_class_header_set_field() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_field(5, FieldInfo {
            name: 10,
            flags: FieldFlags::Public,
            type_info: 2
        });
        assert_eq!(header.get_field(5), FieldInfo {
            name: 10,
            flags: FieldFlags::Public,
            type_info: 2
        });
    }

    #[test]
    fn test_class_header_set_method() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_method(5, MethodInfo {
            name: 10,
            flags: MethodFlags::Public,
            type_info: 2,
            location: 3
        });
        assert_eq!(header.get_method(5), MethodInfo {
            name: 10,
            flags: MethodFlags::Public,
            type_info: 2,
            location: 3
        });
    }
    
}
