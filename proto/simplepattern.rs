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



/*
        private List<Group> Groups;
        
        private class Group {
            public string ParamName;
            public string Text;
            public bool IsParam=false;
            public List<string> ValidMatches=new List<string>();
            public bool MatchAll=true;
            public bool Optional=false;
            public char End;
            public Regex MatchType=null;
        }
*/

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
    fn new(pattern: &str) -> SimplePattern {
        SimplePattern { pattern: pattern.into_owned(), groups: ~[]}
    }
    /*This will update the "groups", which is basically a way to pre-cache and break apart the important parts of the pattern*/
    fn update_groups(&mut self){
        for g in self.pattern.split_iter('/') {
            if(g.char_at(0) == '[') {
                //group
                
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
        let mut parts=input.split_iter('/').filter(|x| *x!="");
        let mut res=false; //input == self.pattern;
        if(parts.len() == 0 && self.groups.len() ==0 ){
            return MatchResult::a_match(ParameterDictionary::new());
        }
        if(parts.len() < self.groups.len()){
            return MatchResult::no_match();
        }
        //if(input.split_iter('/').len() != 
        for (inp,pat) in parts.zip(self.groups.iter()) {
            if(pat.name == ~""){
                //just a raw match
                if(inp == pat.text) {
                    res=true;
                    break;
                }
            }
            
        }
        MatchResult{is_match: res, params: ParameterDictionary::new()}
    }

}


/*
 /** This will parse the Pattern string one group at a time. **
        int ParseParam (int start, ref Group g)
        {
            start++;
            
            int end=Pattern.Substring(start).IndexOf('}')+start;
            if(end+1>=Pattern.Length-1){
                g.End='\0';
            }else{
                g.End=Pattern[end+1];
            }
            string p=CutString(Pattern,start,end);
            g.Text=p;
            int tmp=p.IndexOf('[');
            if(tmp==-1){ //not found. Just trim it up and get the paramname
                p=p.Trim();
                if(p=="*"){
                    g.Optional=true; //meh. Still add it as a match-all group for the hell of it
                }
                g.MatchAll=true;
            }else{
                //return end;
                g.MatchAll=false;
                string l=CutString(p,tmp+1,p.IndexOf(']'));
                l=l.Replace(" ","");
                p=p.Substring(0,p.IndexOf("=")).Trim();
                int count=0;
                while(true){
                    if(l.Length==0){
                        break;
                    }
                    int endm=l.IndexOf(',');
                    if(endm==-1){
                        endm=l.Length;
                        g.ValidMatches.Add(l);
                        break;
                    }
                    g.ValidMatches.Add(l.Remove(endm));
                    l=l.Substring(endm+1);
                    count++;
                    if(count>100){
                        throw new ApplicationException("inifinite loop detected");
                    }
                }
            }
            
            g.ParamName=p;
            return end;
        }
     /** This will update all of the "groups" or parameter names/values for the pattern string. *
        private void UpdateGroups ()
        {
            List<Group> groups = new List<Group> ();
            Group g=new Group();
            for(int i=0;i<Pattern.Length;i++){
                if(Pattern[i]=='{'){ 
                    if(g!=null) //g will never be null
                        groups.Add(g);
                    g=new Group();
                    g.IsParam=true;
                    i=ParseParam(i,ref g);
                    groups.Add(g);
                    g=null;
                }else if(g==null){
                    g=new Group();
                    g.IsParam=false;
                    g.Text+=Pattern[i];
                }else{
                    g.Text+=Pattern[i];
                }
            }
            if(g!=null){
                groups.Add(g);
            }
            Groups=groups;
        }
    a}
        /**Little helper method to cut a string from start to end point. Just shorter than typing .Remove(end).Substring(start) **
        private string CutString(string s,int start,int end){
            return s.Remove(end).Substring(start);
        }
*/

