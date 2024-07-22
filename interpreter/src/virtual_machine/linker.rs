use std::collections::HashMap;

use definitions::{bytecode::MethodIndex, class::{ClassHeader, ClassInfo, PoolEntry, PoolIndex}, object::Reference};

use super::{ConstantPool, ObjectTable};






pub struct Linker<'a> {
    pool_mapper: HashMap<String, PoolIndex>,
    constant_pool: &'a dyn ConstantPool,
    object_table: &'a dyn ObjectTable,
    added_classes: HashMap<String, Reference>,
}

impl<'a> Linker<'a> {
    pub fn new(constant_pool: &'a dyn ConstantPool, object_table: &'a dyn ObjectTable) -> Self {
        Self {
            pool_mapper: HashMap::new(),
            constant_pool,
            object_table,
            added_classes: HashMap::new(),
        }
    }
}
impl Linker<'_> {

    pub fn link_classes(&mut self, classes: Vec<ClassHeader>, main_class: &str, main_method: &str) -> (Reference, MethodIndex) {
        let mut deffered = Vec::new();

        for class in classes.into_iter() {
            if let Some(class) = self.link_class(class) {
                deffered.push(class);
            }
        }

        while !deffered.is_empty() {
            let mut new_deffered = Vec::new();
            for class in deffered.into_iter() {
                if let Some(class) = self.link_class(class) {
                    new_deffered.push(class);
                }
            }
            deffered = new_deffered;
        }

        let class_info_location = self.pool_mapper.get(&format!("ClassInfo: {}", main_class)).expect("Main class not found");
        let class_info = self.constant_pool.get_constant(*class_info_location);
        let class_info = match class_info {
            PoolEntry::ClassInfo(class_info) => class_info,
            x => panic!("Invalid class info {:?}", x),
        };

        let class_ref = class_info.class_ref.expect("Main class not linked");

        let class = self.object_table.get_class(class_ref);
        let (method, _) = class.methods().iter().enumerate().find(|(_, method)| {
            let name = self.constant_pool.get_constant(method.name);
            let name = match name {
                PoolEntry::String(string) => string,
                x => panic!("Invalid method name {:?}", x),
            };
            name == main_method
        }).expect("Main method not found");

        (class_ref, method)
    }

    fn link_class(&mut self, mut class: ClassHeader) -> Option<ClassHeader> {
        let mut skip_indicies = Vec::new();
        let (name, name_location, this_info_location) = self.link_class_info(&mut class, &mut skip_indicies);

        let (class_ref, mut class) = if !self.added_classes.contains_key(&name) {
            let class_ref = self.object_table.add_class(class);
            let mut class = self.object_table.get_class(class_ref);
            self.added_classes.insert(name.clone(), class_ref);
            class.set_constant_pool_entry(class.get_this_info(), PoolEntry::ClassInfo(ClassInfo {
                name: name_location,
                class_ref: Some(class_ref),
            }));
            (class_ref, class)
        } else {
            let class_ref = *self.added_classes.get(&name).unwrap();
            let mut class = self.object_table.get_class(class_ref);
            class.set_constant_pool_entry(class.get_this_info(), PoolEntry::ClassInfo(ClassInfo {
                name: name_location,
                class_ref: Some(class_ref),
            }));
            (class_ref, class)
        };


        
        let entry = self.constant_pool.get_constant(this_info_location);
        match entry {
            PoolEntry::ClassInfo(mut class_info) => {
                class_info.class_ref = Some(class_ref);
                self.constant_pool.set_constant(this_info_location, PoolEntry::ClassInfo(class_info));
            },
            _ => panic!("Invalid class info"),
        }
        
        self.link_interfaces(&mut class, &mut skip_indicies);

        if !self.link_methods(&mut class, &mut skip_indicies, &name) {
            return Some(class);
        }

        self.link_fields(&mut class);
        self.link_strings(&mut class);


        None
    }

    fn link_class_info(&mut self, class: &mut ClassHeader, skip_indices: &mut Vec<PoolIndex>) -> (String, PoolIndex, PoolIndex) {
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

        let name = name.clone();
        let name_location = location;

        let mut class_info = class_info.clone();
        class_info.name = location;
        class_info.class_ref = self.added_classes.get(&name).cloned();

        class.set_constant_pool_entry(index, PoolEntry::ClassInfo(class_info));

        let key = format!("ClassInfo: {}", name);
        let location = if !self.pool_mapper.contains_key(&key) {
            let location = self.constant_pool.add_constant(PoolEntry::ClassInfo(class_info));
            self.pool_mapper.insert(key, location);
            location
        } else {
            *self.pool_mapper.get(&key).unwrap()
        };

        let this_info_location = location;
        
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
        class_info.class_ref = self.added_classes.get(&parent_name).cloned();

        let class_info_key = format!("ClassInfo: {}", parent_name);
        let _location = if !self.pool_mapper.contains_key(&class_info_key) {
            let location = self.constant_pool.add_constant(PoolEntry::ClassInfo(class_info));
            self.pool_mapper.insert(class_info_key, location);
            location
        } else {
            *self.pool_mapper.get(&class_info_key).unwrap()
        };

        class.set_constant_pool_entry(index, PoolEntry::ClassInfo(class_info));

        (name, name_location, this_info_location)
    }

    fn link_interfaces(&mut self, class: &mut ClassHeader, skip_indices: &mut Vec<PoolIndex>) {
        let mut interface_info_locations = Vec::new();
        let mut interface_locations = Vec::new();
        for interface in class.interfaces() {
            skip_indices.push(interface.info);

            let interface_info = class.get_constant_pool_entry(interface.info);
            let class_info = match interface_info {
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
            let old_location = interface.info;
            class_info.name = location;

            let class_info_key = format!("ClassInfo: {}", interface_name);
            let location = if !self.pool_mapper.contains_key(&class_info_key) {
                let location = self.constant_pool.add_constant(PoolEntry::ClassInfo(class_info));
                self.pool_mapper.insert(class_info_key, location);
                location
            } else {
                *self.pool_mapper.get(&class_info_key).unwrap()
            };

            interface_locations.push((old_location, location));

            interface_info_locations.push(location);
        }
        for (old_location, location) in interface_locations {
            let redirect = PoolEntry::Redirect(location);
            class.set_constant_pool_entry(old_location, redirect);
        }
        
        for (i, interface) in class.interfaces_mut().iter_mut().enumerate() {
            interface.info = interface_info_locations[i];
        }
    }

    fn link_methods(&mut self, class: &mut ClassHeader, skip_indices: &mut Vec<PoolIndex>, name: &str) -> bool {
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
                x => panic!("Invalid String {:?}", x),
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

    fn link_fields(&mut self, class: &mut ClassHeader) {
        let mut new_fields = Vec::new();

        for field in class.fields() {
            let name = class.get_constant_pool_entry(field.name);
            let name = match name {
                PoolEntry::String(string) => string,
                x => panic!("Invalid String {:?}", x),
            };

            let location = if !self.pool_mapper.contains_key(name) {
                let location = self.constant_pool.add_constant(PoolEntry::String(name.to_owned()));
                self.pool_mapper.insert(format!("{}", name), location);
                location
            } else {
                *self.pool_mapper.get(name).unwrap()
            };

            let mut field = field.clone();
            field.name = location;

            let type_info = class.get_constant_pool_entry(field.type_info);
            let type_info = match type_info {
                PoolEntry::TypeInfo(info) => info,
                x => panic!("Invalid Type Info {:?}", x),
            };

            let key = format!("FieldTypeInfo: {}", name);
            let location = if !self.pool_mapper.contains_key(&key) {
                let location = self.constant_pool.add_constant(PoolEntry::TypeInfo(type_info.clone()));
                self.pool_mapper.insert(key, location);
                location
            } else {
                *self.pool_mapper.get(&key).unwrap()
            };

            field.type_info = location;

            if let Some(index) = field.location {
                let location = self.constant_pool.add_constant(class.get_constant_pool_entry(index).clone());
                field.location = Some(location);
            }

            
            new_fields.push(field);
        }

        for (i, field) in new_fields.into_iter().enumerate() {
            class.set_field(i, field);
        }
    }

    fn link_strings(&mut self, class: &mut ClassHeader) {
        let mut new_strings = Vec::new();

        for string in class.strings() {
            let string = class.get_constant_pool_entry(*string);
            let string = match string {
                PoolEntry::String(string) => string,
                x => panic!("Invalid String {:?}", x),
            };
            
            let location = if !self.pool_mapper.contains_key(string) {
                let location = self.constant_pool.add_constant(PoolEntry::String(string.to_owned()));
                self.pool_mapper.insert(format!("{}", string), location);
                location
            } else {
                *self.pool_mapper.get(string).unwrap()
            };

            new_strings.push(location);
        }

        for (i, location) in new_strings.into_iter().enumerate() {
            class.set_string(i, location);
        }
    }
}

