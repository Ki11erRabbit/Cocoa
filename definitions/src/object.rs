
pub mod object_table;
pub mod class;


use crate::class::PoolIndex;

pub type Reference = usize;

pub trait Deallocate {
    fn deallocate(&mut self, layout: std::alloc::Layout);
}


pub struct VTable(Box<[PoolIndex]>);

impl VTable {
    pub fn new(table: Vec<PoolIndex>) -> VTable {
        VTable(table.into_boxed_slice())
    }
}



#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Object {
    pub vtable: Reference,
    pub super_vtable: Reference,
    pub class_ref: Reference,
    pub interfaces: Reference,
    pub data_ref: Reference,
}

impl Object {
    fn new(vtable: Reference, super_vtable: Reference, class_ref: Reference, interfaces: Reference, data_ref: Reference) -> *mut Self {
        let layout = std::alloc::Layout::new::<Self>();
        let ptr = unsafe { std::alloc::alloc(layout) };
        let ptr = ptr as *mut Self;
        unsafe {
            std::ptr::write(ptr, Self {
                vtable,
                super_vtable,
                class_ref,
                interfaces,
                data_ref,
            });
        }
        ptr
    }

    fn get_super_vtable(&self) -> Reference {
        self.super_vtable
    }

    fn get_class(&self) -> Reference {
        self.class_ref
    }

    fn get_interfaces(&self) -> Reference {
        self.interfaces
    }

    fn get_data(&self) -> Reference {
        self.data_ref
    }
}

impl Deallocate for Object {
    fn deallocate(&mut self, layout: std::alloc::Layout) {
        unsafe {
            std::alloc::dealloc(self as *mut Self as *mut u8, layout);
        }
    }
}

pub struct NormalObjectBody {}

impl NormalObjectBody {
    pub fn new(member_count: usize) -> *mut Self {
        use std::alloc::*;
        let body = Layout::array::<u64>(member_count).expect("Layout Overflowed");

        let ptr = unsafe { alloc(body) };

        let ptr = ptr as *mut u64;
        
        for i in 0..member_count {
            unsafe {
                ptr.add(i).write(0);
            }
        }
        
        ptr as *mut Self
    }

    /// Unsafe due to indexing into a pointer without a bounds check
    pub unsafe fn get<T: Copy>(&self, index: usize) -> T {
        let this = self as *const Self;
        let this = this as *const u64;

        let this = this.add(index);

        let this = this as *const T;
        this.read()
    }

    /// Unsafe due to indexing into a pointer without a bounds check
    pub unsafe fn set<T: Copy>(&mut self, index: usize, elem: T) {
        let this = self as *mut Self;
        let this = this as *mut u64;

        let this = this.add(index);

        let this = this as *mut T;
        this.write(elem)
    }
}

impl Deallocate for NormalObjectBody {
    fn deallocate(&mut self, layout: std::alloc::Layout) {
        unsafe {
            std::alloc::dealloc(self as *mut Self as *mut u8, layout);
        }
    }
}

pub struct ArrayObjectBody {}

impl ArrayObjectBody {
    pub fn new(len: usize, elem_size: usize) -> *mut Self {
        use std::alloc::*;
        let len_member = Layout::new::<u64>();
        let elem_member = Layout::new::<u64>();
        let body = Layout::array::<u8>(len * elem_size).expect("Layout Overflowed");

        let (layout, _) = len_member.extend(elem_member).expect("Layout Overflowed");
        let (layout, _) = layout.extend(body).expect("Layout Overflowed");

        let ptr = unsafe { alloc(layout) };

        let ptr = ptr as *mut u64;
        unsafe {
            ptr.write(len as u64)
        }
        
        
        ptr as *mut Self
    }

    /// Unsafe due to indexing into a pointer without knowing its size
    pub unsafe fn get<T: Copy>(&self, index: usize) -> T {
        let len = self.len() as usize;
        assert!(index < len, "Tried indexing an array out of bounds");
        use std::mem;
        let this = self as *const Self;
        let this = this as *const u64;

        let this = this.add(mem::size_of::<u64>() * 2);
        let this = this as *const T;

        let this = this.add(index);

        this.read()
    }

    /// Unsafe due to indexing into a pointer without knowing its size
    pub unsafe fn set<T: Copy>(&mut self, index: usize, elem: T) {
        use std::mem;
        let len = self.len() as usize;
        assert!(index < len, "Tried indexing an array out of bounds");
        let this = self as *mut Self;
        let this = this as *mut u64;

        let this = this.add(mem::size_of::<u64>() * 2);
        let this = this as *mut T;
        let this = this.add(index);

        this.write(elem)
    }

    pub fn len(&self) -> u64 {
        let this = self as *const Self;
        let this = this as *const u64;

        unsafe {
            this.read()
        }
    }

    pub fn elem_size(&self) -> u64 {
        let this = self as *const Self;
        let this = this as *const u64;

        unsafe {
            this.add(1).read()
        }
    }
}

impl Deallocate for ArrayObjectBody {
    fn deallocate(&mut self, layout: std::alloc::Layout) {
        unsafe {
            std::alloc::dealloc(self as *mut Self as *mut u8, layout);
        }
    }
}
