use std::hashmap::HashMap;
mod simplepattern_test;


pub struct ParameterDictionary { 
    params: HashMap<~str, ~[~str]> 
}

impl ParameterDictionary { 
    pub fn new() -> ParameterDictionary {
        ParameterDictionary{params: HashMap::new()}
    }
    fn push(&mut self, key: &str, value: &str) {
        let v=self.params.find_or_insert(key.into_owned(), ~[]);
        v.push(value.into_owned());
    }
    fn get(&self, rhs: &~str) -> ~str {
        self.params.get(rhs).head().clone()
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
    is_match: bool,
    params: ParameterDictionary //the contents of this are undefined when not matching
}

impl MatchResult {
    fn a_match(params: ParameterDictionary) -> MatchResult {
        MatchResult{is_match: true, params: params}
    }
    fn no_match() -> MatchResult{
        MatchResult{is_match: false, params: ParameterDictionary::new()}
    }
}




priv struct PatternGroup{
    name: ~str,
    text: ~str,
    valid_values: ~[~str],
    match_all: bool,
}


pub struct SimplePattern {
    pattern: ~str,
    groups: ~[PatternGroup]
}

impl SimplePattern {
    pub fn new(pattern: &str) -> SimplePattern {
        let mut x=SimplePattern { pattern: pattern.into_owned(), groups: ~[]};
        x.update_groups();
        x
    }
    /*This will update the "groups", which is basically a way to pre-cache and break apart the important parts of the pattern*/
    fn update_groups(&mut self){
        for g in self.pattern.split_str("/").filter(|x| !x.is_empty()) {
            if(g.char_at(0) == '[') {
                //group
                if(g.chars().any(|x| x=='=')) {
                    //let 
                    //self.groups.push(PatternGroup{
                        
                }else{
                    //just name
                    self.groups.push(PatternGroup{
                        name: g.trim_chars(&'[').trim_chars(&']').to_owned(),
                        text: ~"",
                        valid_values: ~[~"*"],
                        match_all: false
                    });
                }
                
            }else{
                //just text
                self.groups.push(PatternGroup{ 
                    name: ~"",
                    text: g.into_owned(),
                    valid_values: ~[],
                    match_all: false,
                });
            }
        }
    }
}


impl PatternMatcher for SimplePattern {
    fn matches(&self, input: &str) -> MatchResult {
        let parts=input.split_str("/").filter(|x| !x.is_empty()).to_owned_vec();
        let mut res;
        if(parts.len() == 0 && self.groups.len() ==0 ){
            return MatchResult::a_match(ParameterDictionary::new());
        }
        if(parts.len() < self.groups.len()){
            return MatchResult::no_match();
        }
        
        //if(input.split_iter('/').len() != 
        res=true;
        let mut ran=false;
        let mut i=0;
        let mut params=ParameterDictionary::new();
        for inp in parts.iter() {
            ran=true;
            if(i >= self.groups.len()){
                return MatchResult::no_match();
            }
            let pat=&self.groups[i];
            if(pat.name == ~""){
                //just a raw match
                if(*inp != pat.text) {
                    res=false;
                    break;
                }
            }else{
                params.push(pat.name, *inp);
            }
            i+=1;
        }
        if(ran == false)
        {
            res=true;
        }
        MatchResult{is_match: res, params: params}
    }

}
