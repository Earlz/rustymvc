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
    fn index(&self, rhs: &~str) -> ~str {
        self.params.get(rhs).head().clone()
    }
}

pub trait PatternMatcher { 
    fn matches(&self, input: &str) -> MatchResult;
}


struct MatchResult { 
    matches: bool,
    params: ParameterDictionary
}


struct SimplePattern {
    pattern: ~str,
}

impl SimplePattern {
    fn new(pattern: &str) -> SimplePattern {
        SimplePattern { pattern: pattern.into_owned()}
    }
}


impl PatternMatcher for SimplePattern {
    fn matches(&self, input: &str) -> MatchResult {
        MatchResult{matches: false, params: ParameterDictionary::new()}
    }
}



