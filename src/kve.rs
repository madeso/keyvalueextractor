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
struct Complexity
{
    arguments : i32,
    verifiers : i32,
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
