use std::collections::HashMap;

use definitions::class::{ClassHeader, Method, PoolEntry, PoolIndex};

use super::ConstantPool;






pub struct Linker<'a> {
    pool_mapper: HashMap<String, PoolIndex>,
    constant_pool: &'a dyn ConstantPool,
}

impl<'a> Linker<'a> {
    pub fn new(constant_pool: &'a dyn ConstantPool) -> Self {
        Self {
            pool_mapper: HashMap::new(),
            constant_pool,
        }
    }
}
impl Linker<'_> {

    pub fn link_classes(&mut self, classes: &mut [ClassHeader]) {
        let mut deffered = Vec::new();

        for (index, class) in classes.iter_mut().enumerate() {
            if !self.link_class(class) {
                deffered.push(index);
            }
        }

        while !deffered.is_empty() {
            let mut new_deffered = Vec::new();
            for index in deffered {
                if self.link_class(&mut classes[index]) {
                    new_deffered.push(index);
                }
            }
            deffered = new_deffered;
        }


    }

    fn link_class(&mut self, class: &mut ClassHeader) -> bool {
        let mut skip_indicies = Vec::new();
        let (name, parent_name) = self.link_class_info(class, &mut skip_indicies);
        self.link_interfaces(class, &mut skip_indicies);

        if !self.link_methods(class, &mut skip_indicies, &name, &parent_name) {
            return false;
        }

        // TODO: find static members and put their default values in the constant_pool

        true
    }

    fn link_class_info(&mut self, class: &ClassHeader, skip_indices: &mut Vec<PoolIndex>) -> (String, String) {
        let index = class.get_this_info();
        let entry = class.get_constant_pool_entry(index);
        let class_info = match entry {
            PoolEntry::ClassInfo(class_info) => class_info,
            _ => panic!("Invalid class info"),
        };

        skip_indices.push(class_info.name);
        let entry = class.get_constant_pool_entry(class_info.name);
        let name = match entry {
            PoolEntry::String(string) => string,
            _ => panic!("Invalid class name"),
        };

        let location = if !self.pool_mapper.contains_key(name) {
            let location = self.constant_pool.add_constant(PoolEntry::String(name.to_owned()));
            self.pool_mapper.insert(format!("{}", name), location);
            location
        } else {
            *self.pool_mapper.get(name).unwrap()
        };

        let mut class_info = class_info.clone();
        class_info.name = location;

        let key = format!("ClassInfo: {}", name);
        if !self.pool_mapper.contains_key(&key) {
            let location = self.constant_pool.add_constant(PoolEntry::ClassInfo(class_info));
            self.pool_mapper.insert(key, location);
            location
        } else {
            *self.pool_mapper.get(&key).unwrap()
        };


        let index = class.get_parent_info();
        let entry = class.get_constant_pool_entry(index);
        let class_info = match entry {
            PoolEntry::ClassInfo(class_info) => class_info,
            _ => panic!("Invalid class info"),
        };

        skip_indices.push(class_info.name);
        let entry = class.get_constant_pool_entry(class_info.name);
        let parent_name = match entry {
            PoolEntry::String(string) => string.clone(),
            _ => panic!("Invalid class name"),
        };

        let location = if !self.pool_mapper.contains_key(&parent_name) {
            let location = self.constant_pool.add_constant(PoolEntry::String(parent_name.clone()));
            self.pool_mapper.insert(format!("{}", parent_name), location);
            location
        } else {
            *self.pool_mapper.get(&parent_name).unwrap()
        };

        let mut class_info = class_info.clone();
        class_info.name = location;

        let class_info_key = format!("ClassInfo: {}", parent_name);
        if !self.pool_mapper.contains_key(&class_info_key) {
            let location = self.constant_pool.add_constant(PoolEntry::ClassInfo(class_info));
            self.pool_mapper.insert(class_info_key, location);
        }

        (name.clone(), parent_name)
    }

    fn link_interfaces(&mut self, class: &ClassHeader, skip_indices: &mut Vec<PoolIndex>) {
        for interface in class.interfaces() {
            skip_indices.push(*interface);

            let interface = class.get_constant_pool_entry(*interface);
            let class_info = match interface {
                PoolEntry::ClassInfo(info) => info,
                _ => panic!("Invalid class info"),
            };
            
            let interface_name = class.get_constant_pool_entry(class_info.name);
            let interface_name = match interface_name {
                PoolEntry::String(string) => string,
                _ => panic!("Invalid Interface Name"),

            };
            let location = if !self.pool_mapper.contains_key(interface_name) {
                let location = self.constant_pool.add_constant(PoolEntry::String(interface_name.to_owned()));
                self.pool_mapper.insert(format!("{}",interface_name), location);
                location
            } else {
                *self.pool_mapper.get(interface_name).unwrap()
            };

            let mut class_info = class_info.clone();
            class_info.name = location;

            let class_info_key = format!("ClassInfo: {}", interface_name);
            if !self.pool_mapper.contains_key(&class_info_key) {
                let location = self.constant_pool.add_constant(PoolEntry::ClassInfo(class_info));
                self.pool_mapper.insert(class_info_key, location);
            }
        }
    }

    fn link_methods(&mut self, class: &mut ClassHeader, skip_indices: &mut Vec<PoolIndex>, name: &str, parent_name: &str) -> bool {
        let mut method_indices = Vec::new();
        let mut type_indices = Vec::new();
        let mut name_indices = Vec::new();
        for method_info in class.methods() {
            let method_type_info = method_info.type_info;
            let method_name = method_info.name;
            let method_location = method_info.location;

            skip_indices.push(method_type_info);
            skip_indices.push(method_name);
            skip_indices.push(method_location);

            let method_name = class.get_constant_pool_entry(method_name);
            let method_name = match method_name {
                PoolEntry::String(string) => string,
                _ => panic!("Invalid String"),
            };

            let location = if !self.pool_mapper.contains_key(method_name) {
                let location = self.constant_pool.add_constant(PoolEntry::String(method_name.to_owned()));
                self.pool_mapper.insert(format!("{}", method_name), location);
                location
            } else {
                *self.pool_mapper.get(method_name).unwrap()
            };
            name_indices.push(location);

            let method_type_info = class.get_constant_pool_entry(method_type_info);
            let method_type_info = match method_type_info {
                PoolEntry::TypeInfo(info) => info,
                _ => panic!("Invalid Type Info"),
            };

            let key = format!("MethodTypeInfo: {} {}", name, method_name);
            let location = if !self.pool_mapper.contains_key(&key) {
                let location = self.constant_pool.add_constant(PoolEntry::TypeInfo(method_type_info.clone()));
                self.pool_mapper.insert(key, location);
                location
            } else {
                *self.pool_mapper.get(&key).unwrap()
            };
            type_indices.push(location);

            let method = class.get_constant_pool_entry(method_location);
            let method = match method {
                PoolEntry::Method(method) => method,
                _ => panic!("Invalid Method"),
            };

            let method_location = match method {
                Method::Foreign => {
                    let key = format!("Method: {} {}", parent_name, method_name);
                    let Some(location) = self.pool_mapper.get(&key) else {
                        return false;
                    };
                    *location
                }
                method => {
                    let key = format!("Method: {} {}", name, method_name);
                    if !self.pool_mapper.contains_key(&key) {
                        let location = self.constant_pool.add_constant(PoolEntry::Method(method.clone()));
                        self.pool_mapper.insert(key, location);
                        location
                    } else {
                        *self.pool_mapper.get(&key).unwrap()
                    }
                },
            };

            method_indices.push(method_location);
        }

        for (i, method_info) in class.methods_mut().iter_mut().enumerate() {
            method_info.location = method_indices[i];
            method_info.name = name_indices[i];
            method_info.type_info = type_indices[i];
        }
        
        true
    }
}
