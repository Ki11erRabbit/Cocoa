use crate::object::Reference;

use self::stackframe::{StackFrame, StackFrameUtils};

mod stackframe;


pub trait StackUtils<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> T;
    fn set_argument(&mut self, index: u8, value: T);
    fn get_argument(&mut self, index: u8) -> T;
}

pub struct Stack {
    stack: Vec<StackFrame>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push_frame(&mut self, class_index: Reference, method_index: usize) {
        self.stack.push(StackFrame::new(class_index, method_index));
    }

    pub fn pop_frame(&mut self) {
        self.stack.pop();
    }

    pub fn generic_pop(&mut self) {
        self.stack.last_mut().expect("Stack Underflow").generic_pop();
    }

    pub fn dup(&mut self) {
        self.stack.last_mut().expect("Stack Underflow").dup();
    }

    pub fn swap(&mut self) {
        self.stack.last_mut().expect("Stack Underflow").swap();
    }

    pub fn set_local(&mut self, index: u8) {
        self.stack.last_mut().expect("Stack Underflow").store_local(index);
    }

    pub fn get_local(&mut self, index: u8) {
        self.stack.last_mut().expect("Stack Underflow").load_local(index);
    }

    pub fn get_class_index(&self) -> Reference {
        self.stack.last().expect("Stack Underflow").get_class_reference()
    }

    pub fn get_current_pc(&self) -> usize {
        self.stack.last().expect("Stack Underflow").get_pc()
    }

    pub fn set_current_pc(&mut self, pc: usize) {
        self.stack.last_mut().expect("Stack Underflow").set_pc(pc);
    }

    pub fn get_current_method_index(&self) -> usize {
        self.stack.last().expect("Stack Underflow").get_method_index()
    }
}


impl StackUtils<i8> for Stack {
    fn push(&mut self, value: i8) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> i8 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: i8) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> i8 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<i32> for Stack {
    fn push(&mut self, value: i32) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> i32 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: i32) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> i32 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<i64> for Stack {
    fn push(&mut self, value: i64) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> i64 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: i64) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> i64 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<u8> for Stack {
    fn push(&mut self, value: u8) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> u8 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: u8) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> u8 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<u32> for Stack {
    fn push(&mut self, value: u32) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> u32 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: u32) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> u32 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<u64> for Stack {
    fn push(&mut self, value: u64) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> u64 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: u64) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> u64 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<f32> for Stack {
    fn push(&mut self, value: f32) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> f32 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: f32) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> f32 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<f64> for Stack {
    fn push(&mut self, value: f64) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> f64 {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: f64) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> f64 {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

impl StackUtils<Reference> for Stack {
    fn push(&mut self, value: Reference) {
        self.stack.last_mut().expect("Stack Underflow").push(value);
    }

    fn pop(&mut self) -> Reference {
        self.stack.last_mut().unwrap().pop()
    }

    fn set_argument(&mut self, index: u8, value: Reference) {
        self.stack.last_mut().expect("Stack Underflow").store_argument(index, value);
    }

    fn get_argument(&mut self, index: u8) -> Reference {
        self.stack.last_mut().expect("Stack Underflow").load_argument(index)
    }
}

