use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn count_directory_seperators(pattern: &str) -> u32
{
    let mut count : u32 = 0;
    for c in pattern.chars()
    {
        if c == '/' {
            count += 1;
        }
    }
    count
}

#[derive(Debug)]
struct Match
{
    is_text : bool,
    data: String,
}

#[derive(Debug)]
pub struct FoundMatch
{
    message: Option<String>,
    data: HashMap<String, String>,
}

impl FoundMatch
{
    fn err(&mut self, message: &str)
    {
        self.message = Some(String::from(message))
    }
    fn get(&self, varname: &str) -> Option<String>
    {
        if let Some(r) = self.data.get(varname)
        {
            Some(r.to_string())
        }
        else
        {
            None
        }
    }
    fn set(&mut self, varname: &str, value: &str) -> bool
    {
        fn make_clean(val: &str) -> String
        {
            val.to_string().to_lowercase().replace("_", "").trim().trim_left_matches('0').to_string()
        }

        match self.data.entry(varname.to_string())
        {
            Entry::Occupied(current_value) => {
                make_clean(current_value.get()) == make_clean(value)
            }
            Entry::Vacant(entry) => {
                entry.insert(value.to_string());
                true
            }
        }
    }
}

#[derive(Debug)]
pub struct KeyValueExtractor
{
    number_of_directory_seperators: u32,
    matchers: Vec<Match>,
}

#[derive(Debug)]
pub enum CompileError
{
    Fail
}

impl KeyValueExtractor
{
    fn add_argument(&mut self, t: &str)
    {
        self.matchers.push(Match{ is_text: false, data: String::from(t)});
    }

    fn add_text(&mut self, t: &str)
    {
        self.matchers.push(Match{ is_text: true, data: String::from(t)});
        self.number_of_directory_seperators += count_directory_seperators(t);
    }

    pub fn get_searchable_string(&self, path: &Path) -> Option<String>
    {
        let mut s = path.file_stem()?.to_string_lossy().to_string();
        let mut p = path;
        for _ in 0..self.number_of_directory_seperators
        {
            p = p.parent()?;
            s = p.file_stem()?.to_string_lossy().to_string() + "/" + &s;
        }
        Some(s)
    }

    pub fn extract(&self, path: &Path) -> FoundMatch
    {
        let mut m = FoundMatch { message: None, data: HashMap::new() };
        return match self.get_searchable_string(path)
        {
            None => { m.err("Path was invalid."); m }
            Some(the_text) => {
                let text: String = the_text;
                // todo: implement data extract function
                let mut start = 0;
                let mut arg = "";
                for matcher in self.matchers.iter()
                {
                    if matcher.is_text
                    {
                        // remove & for unhelpful compilation error
                        let f = text[start..].find(&matcher.data);
                        if f == None
                        {
                            // remove to_string for unhelpful compilation error
                            let err = format!("Unable to find {} in {}", matcher.data, text[start..].to_string());
                            m.err(&err);
                            return m;
                        }
                        // since we cant do find with start and end, move up end to match the lacking find function
                        let end = f.unwrap() + start;
                        if arg != ""
                        {
                            // remove to_string for hard to debug compiler error
                            let val = text[start..end].to_string();
                            if !m.set(arg, &val)
                            {
                                let err = format!("Unable to apply {} to {}, already contains {:?}", val, arg, m.get(arg));
                                m.err(&err);
                                return m;
                            }
                            arg = ""
                        }
                        start = end + matcher.data.chars().count();
                    }
                    else
                    {
                        // not text
                        if arg != ""
                        {
                            // this should be caught when parsing
                            m.err("argument specified twice..");
                            return m;
                        }
                        arg = &matcher.data;
                    }
                }
                if arg != ""
                {
                    // remove to_string for hard to debug compiler error
                    let val = text[start..].to_string();
                    if !m.set(arg, &val)
                    {
                        let err = format!("Unable to apply {} to {}, already contains {:?}", val, arg, m.get(arg));
                        m.err(&err);
                        return m;
                    }
                }
                m
            }
        }
    }

    pub fn new(pattern: &str) -> Result<KeyValueExtractor, CompileError>
    {
        let mut p = KeyValueExtractor{number_of_directory_seperators: 0, matchers: vec![]};
        let k = '%';
        let mut special = false;
        let mut mem : String = "".to_string();

        for c in pattern.chars()
        {
            if c == k
            {
                let t = mem;
                mem = "".to_string();
                if special
                {
                    if t == ""
                    {
                        mem.push(k)
                    }
                    else
                    {
                        p.add_argument(&t);
                    }
                }
                else
                {
                    if t == ""
                    {
                    }
                    else
                    {
                        p.add_text(&t);
                    }
                }

                special = !special;
            }
            else
            {
                mem.push(c);
            }
        }

        if special
        {
            return Err(CompileError::Fail)
        }
        if mem != ""
        {
            p.add_text(&mem)
        }

        Ok(p)
    }
}
