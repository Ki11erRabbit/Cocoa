
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
pub enum PoolEntry<'a> {
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
    String(&'a str),
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
    Native(NativeMethodIndex, usize),
    Bytecode(Box<[Bytecode]>, usize),
    Foreign {
        name: PoolIndex,
    },
    ForeignLinked {
        class_ref: Reference,
        method_index: PoolIndex,
        name: PoolIndex,
    },
}

impl Method {
    pub fn get_vtable_index(&self) -> usize {
        match self {
            Method::Native(index, _) => *index,
            Method::Bytecode(_, index) => *index,
            Method::Foreign { .. } => panic!("Foreign method has not yet been linked!"),
            Method::ForeignLinked { method_index, .. } => *method_index,
        }
    }
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

pub struct ClassHeaderBody<'a> {
    this_info: PoolIndex,
    parent_info: PoolIndex,
    class_flags: ClassFlags,
    constant_pool: Vec<PoolEntry<'a>>,
    interfaces: Vec<PoolIndex>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
}

impl<'a> ClassHeaderBody<'a> {
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClassHeader(*mut ClassHeaderBody<'static>);

impl ClassHeader {
    pub fn new(constant_pool_size: usize, interfaces_count: usize, fields_count: usize, methods_count: usize) -> Self {
        use std::alloc::{alloc, Layout};

        let layout = Layout::new::<ClassHeaderBody<'static>>();
        unsafe {
            let ptr = alloc(layout) as *mut ClassHeaderBody<'static>;
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

    pub fn set_constant_pool_entry(&mut self, index: usize, entry: PoolEntry<'static>) {
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
        let layout = Layout::new::<ClassHeaderBody<'static>>();
        let ptr = self.0 as *mut ClassHeaderBody<'static>;
        unsafe {
            core::ptr::drop_in_place(ptr);
            dealloc(ptr as *mut u8, layout);
        }
    }

}

/*impl ClassHeader {
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

    pub fn get_constant_pool_entry(&self, index: usize) -> &PoolEntry {
        let ptr = self.0 as *const ClassHeaderPart1;
        let ptr = unsafe { ptr.add(1) };
        let ptr = ptr as *const PoolEntry;
        let ptr = unsafe { ptr.add(index) };
        unsafe {
            ptr.as_ref().unwrap()
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

    pub fn get_field(&self, index: usize) -> &FieldInfo {
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
            ptr.as_ref().unwrap()
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
            println!("Setting method at index {}", index);
            println!("Method: {:?}", method);
            println!("Method at ptr: {:?}", ptr.read());
            ptr.write(method);
        }
    }

    pub fn get_method(&self, index: usize) -> &MethodInfo {
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
            ptr.as_ref().unwrap()
        }
    }

    pub fn deallocate(&mut self) {
        use std::alloc::{Layout, dealloc};
        println!("Deallocating class header");
        let layout = Layout::new::<ClassHeaderPart1>();
        let (layout, _) = layout.extend(Layout::array::<PoolEntry>(self.constant_pool_len()).unwrap()).unwrap();
        let (layout, _) = layout.extend(Layout::new::<ClassHeaderPart2>()).unwrap();
        let (layout, _) = layout.extend(Layout::array::<PoolIndex>(self.interfaces_count()).unwrap()).unwrap();
        let (layout, _) = layout.extend(Layout::new::<ClassHeaderPart3>()).unwrap();
        let (layout, _) = layout.extend(Layout::array::<FieldInfo>(self.fields_count()).unwrap()).unwrap();
        let (layout, _) = layout.extend(Layout::new::<ClassHeaderPart4>()).unwrap();
        let (layout, _) = layout.extend(Layout::array::<MethodInfo>(self.methods_count()).unwrap()).unwrap();

        let ptr = self.0 as *mut u8;
        unsafe {
            core::ptr::drop_in_place(ptr as *mut ClassHeaderPart1);
            let og_ptr = ptr;
            let ptr = ptr.add(1);
            let mut ptr = ptr as *mut PoolEntry;
            for i in 0..self.constant_pool_len() {
                ptr = ptr.add(i);
                core::ptr::drop_in_place(ptr);
            }
            let ptr = ptr as *mut ClassHeaderPart2;
            core::ptr::drop_in_place(ptr);
            let ptr = ptr.add(1);
            let mut ptr = ptr as *mut PoolIndex;
            for i in 0..self.interfaces_count() {
                ptr = ptr.add(i);
                core::ptr::drop_in_place(ptr);
            }
            let ptr = ptr as *mut ClassHeaderPart3;
            core::ptr::drop_in_place(ptr);
            let ptr = ptr.add(1);
            let mut ptr = ptr as *mut FieldInfo;
            for i in 0..self.fields_count() {
                ptr = ptr.add(i);
                core::ptr::drop_in_place(ptr);
            }
            let ptr = ptr as *mut ClassHeaderPart4;
            core::ptr::drop_in_place(ptr);
            let ptr = ptr.add(1);
            let mut ptr = ptr as *mut MethodInfo;
            for i in 0..self.methods_count() {
                ptr = ptr.add(i);
                core::ptr::drop_in_place(ptr);
            }
            
            dealloc(og_ptr, layout);
        }
    }

}*/


impl std::fmt::Debug for ClassHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassHeader")
            .field("this_info", &self.get_this_info())
            .field("parent_info", &self.get_parent_info())
            .field("class_flags", &self.get_class_flags())
            .field("constant_pool", &self.constant_pool_len())
            .field("interfaces", &self.interfaces_count())
            .field("fields", &self.fields_count())
            .field("methods", &self.methods_count())
            .finish()
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
