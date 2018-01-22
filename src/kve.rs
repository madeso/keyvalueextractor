pub fn count_directory_seperators(pattern: &str) -> u32
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
