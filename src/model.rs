#[derive(Debug, PartialEq)]
pub struct Super<'a> {
    pub super_name: &'a str,
    pub real_name: &'a str,
    pub power: u16,
    pub sidekick: Option<Box<Option<Super<'a>>>>,
}

#[derive(Debug, PartialEq)]
pub struct Group<'a> {
    pub name: &'a str,
    pub members: Vec<Super<'a>>,
}
