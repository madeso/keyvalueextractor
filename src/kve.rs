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
enum Node
{
    StaticText(String),
    Argument(String),
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
    nodes: Vec<Node>,
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
        self.nodes.push(Node::Argument(String::from(t)));
    }

    fn add_text(&mut self, t: &str)
    {
        self.nodes.push(Node::StaticText(String::from(t)));
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
            Some(text) => {
                let mut start = 0; // start position in search string
                let mut key_name = ""; // the key name to set
                for node in self.nodes.iter()
                {
                    match node
                    {
                        &Node::StaticText(ref data) =>
                        {
                            let f = text[start..].find(data);
                            if f == None
                            {
                                let err = format!("Unable to find {} in {}", data, text[start..].to_string());
                                m.err(&err);
                                return m;
                            }
                            // since we cant do find with start and end, move up end to match the lacking find function
                            let end = f.unwrap() + start;
                            if key_name != ""
                            {
                                let val = text[start..end].to_string();
                                if !m.set(key_name, &val)
                                {
                                    let err = format!("Unable to apply {} to {}, already contains {:?}", val, key_name, m.get(key_name));
                                    m.err(&err);
                                    return m;
                                }
                                key_name = ""
                            }
                            start = end + data.chars().count();
                        }
                        &Node::Argument(ref data) =>
                        {
                            // not text
                            if key_name != ""
                            {
                                // this should be caught when parsing
                                m.err("argument specified twice..");
                                return m;
                            }
                            key_name = &data;
                        }
                    }
                }
                if key_name != ""
                {
                    // remove to_string for hard to debug compiler error
                    let val = text[start..].to_string();
                    if !m.set(key_name, &val)
                    {
                        let err = format!("Unable to apply {} to {}, already contains {:?}", val, key_name, m.get(key_name));
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
        let mut p = KeyValueExtractor{number_of_directory_seperators: 0, nodes: vec![]};
        let argument_char = '%';
        let mut inside_argument = false;
        let mut mem : String = "".to_string();

        for c in pattern.chars()
        {
            if c == argument_char
            {
                if inside_argument
                {
                    if mem == ""
                    {
                        // two %% means escape %
                        mem.push(argument_char)
                    }
                    else
                    {
                        p.add_argument(&mem);
                        mem = "".to_string();
                    }
                }
                else
                {
                    if mem == ""
                    {
                        // this might happen between patterns
                        // not sure how the pattern matching works then and what is extracted
                        // need to define behaviour and add tests
                    }
                    else
                    {
                        p.add_text(&mem);
                        mem = "".to_string();
                    }
                }

                inside_argument = !inside_argument;
            }
            else
            {
                mem.push(c);
            }
        }

        if inside_argument
        {
            // this means that a argument was started but did not end before the pattern had ended
            return Err(CompileError::Fail)
        }
        if mem != ""
        {
            p.add_text(&mem)
        }

        Ok(p)
    }
}
