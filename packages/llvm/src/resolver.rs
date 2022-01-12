use std::collections::HashMap;

pub struct Resolver {
    pub main: Option<String>,
    pub dependencies: HashMap<String, String>,
}

impl Resolver {
    pub fn new() -> Self {
        return Resolver {
            main: None,
            dependencies: HashMap::new(),
        };
    }

    pub fn set_main(&mut self, main_content: &str) {
        self.main = Some(main_content.to_string());
    }

    pub fn add_dependencies(&mut self, file_name: &str, content: &str) {
        self.dependencies
            .insert(file_name.to_string(), content.to_string());
    }
}
