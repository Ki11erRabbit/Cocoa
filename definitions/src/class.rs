
use crate::{bytecode::Bytecode, object::Reference};



pub type PoolIndex = usize;
pub type NativeMethodIndex = usize;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClassFlags: u8 {
        const Public = 0x01;
        const Final = 0x02;
        const Super = 0x20;
        const Interface = 0x40;
        const Abstract = 0x80;
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PoolEntry {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    ClassInfo(ClassInfo),
    Method(Method),
    TypeInfo(TypeInfo),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassInfo {
    pub name: PoolIndex,
    pub class_ref: Option<Reference>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Native(NativeMethodIndex),
    Bytecode(Box<[Bytecode]>),
    Foreign,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeInfo {
    Unit,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Bool,
    String,
    Array(Box<TypeInfo>),
    Object(PoolIndex),
    Method {
        args: Vec<TypeInfo>,
        ret: Box<TypeInfo>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldInfo {
    name: PoolIndex,
    flags: FieldFlags,
    type_info: PoolIndex,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FieldFlags: u8 {
        const Public = 0x01;
        const Private = 0x02;
        const Protected = 0x04;
        const Static = 0x08;
        const Const = 0x10;
        const Synthetic = 0x20;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MethodInfo {
    pub flags: MethodFlags,
    pub name: PoolIndex,
    pub type_info: PoolIndex,
    pub location: PoolIndex,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MethodFlags: u8 {
        const Public = 0x01;
        const Private = 0x02;
        const Protected = 0x04;
        const Static = 0x08;
        const Const = 0x10;
        const Abstract = 0x20;
        const VaArgs = 0x40;
    }
}

pub struct ClassHeaderBody {
    this_info: PoolIndex,
    parent_info: PoolIndex,
    class_flags: ClassFlags,
    constant_pool: Vec<PoolEntry>,
    interfaces: Vec<PoolIndex>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
}

impl ClassHeaderBody {
    pub fn new(constant_pool_size: usize, interfaces_count: usize, fields_count: usize, methods_count: usize) -> Self {
        let mut constant_pool = Vec::with_capacity(constant_pool_size);
        constant_pool.resize_with(constant_pool_size, || PoolEntry::U8(0));
        let mut interfaces = Vec::with_capacity(interfaces_count);
        interfaces.resize_with(interfaces_count, || 0);
        let mut fields = Vec::with_capacity(fields_count);
        fields.resize_with(fields_count, || FieldInfo {
            name: 0,
            flags: FieldFlags::empty(),
            type_info: 0,
        });
        let mut methods = Vec::with_capacity(methods_count);
        methods.resize_with(methods_count, || MethodInfo {
            flags: MethodFlags::empty(),
            name: 0,
            type_info: 0,
            location: 0,
        });
        
        ClassHeaderBody {
            this_info: 0,
            parent_info: 0,
            class_flags: ClassFlags::empty(),
            constant_pool,
            interfaces,
            fields,
            methods,
        }
    }

}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClassHeader(*mut ClassHeaderBody);

impl ClassHeader {
    pub fn new(constant_pool_size: usize, interfaces_count: usize, fields_count: usize, methods_count: usize) -> Self {
        use std::alloc::{alloc, Layout};

        let layout = Layout::new::<ClassHeaderBody>();
        unsafe {
            let ptr = alloc(layout) as *mut ClassHeaderBody;
            ptr.write(ClassHeaderBody::new(constant_pool_size, interfaces_count, fields_count, methods_count));
            ClassHeader(ptr)
        }
    }

    pub fn set_this_info(&mut self, this_info: PoolIndex) {
        unsafe {
            (*self.0).this_info = this_info;
        }
    }

    pub fn get_this_info(&self) -> PoolIndex {
        unsafe {
            (*self.0).this_info
        }
    }

    pub fn set_parent_info(&mut self, parent_info: PoolIndex) {
        unsafe {
            (*self.0).parent_info = parent_info;
        }
    }

    pub fn get_parent_info(&self) -> PoolIndex {
        unsafe {
            (*self.0).parent_info
        }
    }

    pub fn set_class_flags(&mut self, class_flags: ClassFlags) {
        unsafe {
            (*self.0).class_flags = class_flags;
        }
    }

    pub fn get_class_flags(&self) -> ClassFlags {
        unsafe {
            (*self.0).class_flags
        }
    }

    pub fn constant_pool_len(&self) -> usize {
        unsafe {
            (*self.0).constant_pool.len()
        }
    }

    pub fn interfaces_count(&self) -> usize {
        unsafe {
            (*self.0).interfaces.len()
        }
    }

    pub fn methods_count(&self) -> usize {
        unsafe {
            (*self.0).methods.len()
        }
    }

    pub fn fields_count(&self) -> usize {
        unsafe {
            (*self.0).fields.len()
        }
    }

    pub fn set_constant_pool_entry(&mut self, index: usize, entry: PoolEntry) {
        unsafe {
            (*self.0).constant_pool[index] = entry;
        }
    }

    pub fn get_constant_pool_entry(&self, index: usize) -> &PoolEntry {
        unsafe {
            &(*self.0).constant_pool[index]
        }
    }

    pub fn set_interface(&mut self, index: usize, interface: PoolIndex) {
        unsafe {
            (*self.0).interfaces[index] = interface;
        }
    }

    pub fn get_interface(&self, index: usize) -> PoolIndex {
        unsafe {
            (*self.0).interfaces[index]
        }
    }

    pub fn set_field(&mut self, index: usize, field: FieldInfo) {
        unsafe {
            (*self.0).fields[index] = field;
        }
    }

    pub fn get_field(&self, index: usize) -> &FieldInfo {
        unsafe {
            &(*self.0).fields[index]
        }
    }

    pub fn set_method(&mut self, index: usize, method: MethodInfo) {
        unsafe {
            (*self.0).methods[index] = method;
        }
    }

    pub fn get_method(&self, index: usize) -> &MethodInfo {
        unsafe {
            &(*self.0).methods[index]
        }
    }

    pub fn deallocate(&mut self) {
        use std::alloc::{Layout, dealloc};
        let layout = Layout::new::<ClassHeaderBody>();
        let ptr = self.0 as *mut ClassHeaderBody;
        unsafe {
            core::ptr::drop_in_place(ptr);
            dealloc(ptr as *mut u8, layout);
        }
    }

    pub fn constants(&self) -> &[PoolEntry] {
        unsafe {
            &(*self.0).constant_pool
        }
    }

    pub fn constants_mut(&mut self) -> &mut [PoolEntry] {
        unsafe {
            &mut (*self.0).constant_pool
        }
    }

    pub fn interfaces(&self) -> &[PoolIndex] {
        unsafe {
            &(*self.0).interfaces
        }
    }

    pub fn interfaces_mut(&mut self) -> &mut [PoolIndex] {
        unsafe {
            &mut (*self.0).interfaces
        }
    }

    pub fn fields(&self) -> &[FieldInfo] {
        unsafe {
            &(*self.0).fields
        }
    }

    pub fn fields_mut(&mut self) -> &mut [FieldInfo] {
        unsafe {
            &mut (*self.0).fields
        }
    }

    pub fn methods(&self) -> &[MethodInfo] {
        unsafe {
            &(*self.0).methods
        }
    }

    pub fn methods_mut(&mut self) -> &mut [MethodInfo] {
        unsafe {
            &mut (*self.0).methods
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
        assert_eq!(*header.get_constant_pool_entry(5), PoolEntry::String("Hello"));
    }

    #[test]
    fn test_class_header_set_interface() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_interface(4, 10);
        assert_eq!(header.get_interface(4), 10);
    }

    #[test]
    fn test_class_header_set_field() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_field(2, FieldInfo {
            name: 10,
            flags: FieldFlags::Public,
            type_info: 2
        });
        assert_eq!(*header.get_field(2), FieldInfo {
            name: 10,
            flags: FieldFlags::Public,
            type_info: 2
        });
    }

    #[test]
    fn test_class_header_set_method() {
        let mut header = ClassHeader::new(10, 5, 3, 4);
        header.set_method(3, MethodInfo {
            name: 10,
            flags: MethodFlags::Public,
            type_info: 2,
            location: 3
        });
        assert_eq!(*header.get_method(3), MethodInfo {
            name: 10,
            flags: MethodFlags::Public,
            type_info: 2,
            location: 3
        });
    }
    
}
