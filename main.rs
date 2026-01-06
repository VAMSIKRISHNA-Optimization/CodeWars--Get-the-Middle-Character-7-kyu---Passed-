fn get_middle(s:&str) -> &str 
{
    if s.is_empty() { return "";}
    if s.len() % 2 == 1 
    { 
        // let s_char = s.chars().nth((s.len()/2) + 0).unwrap().to_string();
        // return &s_char;
        &s[s.len()/2..=s.len()/2]
    }
    else
    {
        // let s_char = format!("{}{}", s.chars().nth((s.len()/2) - 1).unwrap(), s.chars().nth((s.len()/2) + 0).unwrap()); // Creates String "ab"
        // return &s_char;
          &s[(s.len()/2) - 1..=(s.len()/2)]
    }
}
