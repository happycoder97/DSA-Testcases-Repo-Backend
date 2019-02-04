pub struct Subject {
    pub id: i32,
    pub title: String
}

pub trait ISubjectDB {
    fn add_subject(&self, title: String);
    fn delete_subject(&self, id: i32);
    fn update_subject(&self, id: i32, title: String);
    fn get_all(&self) -> Vec<Subject>;
}