use std::collections::HashMap;

use crate::symbol_table::SymbolMetaInsert;

pub trait ImportResolver {
    fn get(
        &self,
        relative_file_name: &str,
        cur_file_name: &str,
    ) -> Option<&HashMap<String, SymbolMetaInsert>>;
    fn contains(&self, relative_file_name: &str, cur_file_name: &str) -> bool;
    fn get_id(&self, relative_file_name: &str, cur_file_name: &str) -> usize;

    fn resolve(&mut self, relative_file_name: &str, cur_file_name: &str) -> Result<(), String>;
}

pub struct DummyImportResolver(());

impl DummyImportResolver {
    pub fn new() -> Self {
        return DummyImportResolver(());
    }
}

impl ImportResolver for DummyImportResolver {
    fn get(
        &self,
        _relative_file_name: &str,
        _cur_file_name: &str,
    ) -> Option<&HashMap<String, SymbolMetaInsert>> {
        unreachable!();
    }
    fn contains(&self, _relative_file_name: &str, _cur_file_name: &str) -> bool {
        unreachable!();
    }
    fn get_id(&self, _relative_file_name: &str, _cur_file_name: &str) -> usize {
        unreachable!();
    }

    fn resolve(&mut self, _relative_file_name: &str, _cur_file_name: &str) -> Result<(), String> {
        unreachable!();
    }
}
