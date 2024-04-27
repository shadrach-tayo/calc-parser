#[derive(Debug)]
pub struct SymbolTable {
    entries: Vec<(String, f64)>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { entries: vec![] }
    }

    pub fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        let exists = self.entries.iter().find(|entry| entry.0 == identifier);
        if exists.is_none() {
            self.entries.push((identifier.to_string(), 0.0));
            Ok(self.entries.len() - 1)
        } else {
            Err(format!("Error: Duplicate Identifier '{}'", identifier))
        }
    }

    pub fn find_symbol(&self, identifier: &str) -> Result<usize, String> {
        let result = self.entries.iter().position(|entry| entry.0 == identifier);

        match result {
            Some(value) => Ok(value),
            None => Err("".to_string()),
        }
    }

    pub fn get_name(&self, handle: usize) -> String {
        self.entries[handle].0.clone()
    }

    pub fn get_value(&self, handle: usize) -> f64 {
        self.entries[handle].1
    }
    pub fn set_value(&mut self, handle: usize, value: f64) {
        self.entries[handle].1 = value;
    }
    pub fn iter(&self) -> std::slice::Iter<'_, (String, f64)> {
        self.entries.iter()
    }
}
