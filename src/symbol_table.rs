#[derive(Debug)]
pub struct SymbolTable {
    entries: Vec<(String, f64)>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            entries: vec![]
        }
    }

    pub fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        let exists = self.entries.iter().find(|entry| &entry.0 == identifier);
        if exists.is_none() {
            self.entries.push((identifier.to_string(), 0.0));
            Ok(self.entries.len())
        } else {
            Err(format!("Error: Duplicate Identifier '{}'", identifier))
        }
    }

    pub fn find_symbol(&self, identifier: &str) -> Result<usize, String> {
        let result = self
            .entries
            .iter()
            .position(|entry| &entry.0 == identifier);

        match result {
            Some(value) => Ok(value),
            None => Err("".to_string())
        }

    }
}