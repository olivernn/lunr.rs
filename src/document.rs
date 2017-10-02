pub trait Document<'a> {
    fn id(&self) -> &'a str;
    fn fields(&self) -> Vec<Field>;
}

pub struct Field {
    pub name: String,
    pub text: String,
}