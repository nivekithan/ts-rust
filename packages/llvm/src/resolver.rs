use std::collections::HashMap;

pub struct Resolver<T> {
    pub main: Option<T>,
    pub dependencies: HashMap<String, T>,
    pub compiler_provided: Option<T>,
}

impl<T> Resolver<T> {
    pub fn new() -> Self {
        return Resolver {
            main: None,
            dependencies: HashMap::new(),
            compiler_provided: None,
        };
    }

    pub fn set_main(&mut self, main_content: T) {
        self.main = Some(main_content);
    }

    pub fn add_dependencies(&mut self, file_name: &str, content: T) {
        self.dependencies.insert(file_name.to_string(), content);
    }
}
