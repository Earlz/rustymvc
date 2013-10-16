use std::hashmap::HashMap;
mod simplepattern_test;


struct ParameterDictionary { 
    params: HashMap<~str, ~[~str]> 
}

impl ParameterDictionary { 
    fn new() -> ParameterDictionary {
        ParameterDictionary{params: HashMap::new()}
    }
    fn add(&mut self, key: &str, value: &str) {
        let v=self.params.find_or_insert(key.into_owned(), ~[]);
        v.push(value.into_owned());
    }
}
impl Index<~str, ~str> for ParameterDictionary {
    fn index(&self, _rhs: &~str) -> ~str {
        self.params.get(_rhs).head().clone()
    }
}



