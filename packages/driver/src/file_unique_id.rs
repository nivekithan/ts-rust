use std::collections::HashMap;

/*
 * 0 id is reserved for main file
 * 1 id is reserved for compilerInternal
 *  */
pub struct FileUniqueId {
    counter: usize,
    map: HashMap<String, usize>,
}

impl FileUniqueId {
    pub fn new() -> Self {
        return FileUniqueId {
            counter: 2,
            map: HashMap::new(),
        };
    }

    pub fn get(&self, file_name: &str) -> Option<&usize> {
        return self.map.get(file_name);
    }

    #[allow(dead_code)]
    pub fn contains(&self, file_name: &str) -> bool {
        return self.map.contains_key(file_name);
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, file_name: &str) {
        if self.contains(file_name) {
            panic!("There is already a file with name {}", file_name);
        }

        self.map.insert(file_name.to_string(), self.counter);
        self.counter += 1;
    }

    pub fn insert_main(&mut self, main_file_name: &str) {
        if self.contains(main_file_name) {
            panic!(
                "There is already a file with name {}. So it can't be main file",
                main_file_name
            );
        }

        self.map.insert(main_file_name.to_string(), 0);
    }
}
