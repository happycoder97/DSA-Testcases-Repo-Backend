pub struct Subject {
    pub id: i32,
    pub title: String,
}

pub struct NewSubject {
    pub title: String,
}

pub trait ISubjectDB {
    fn insert(&self, new_subject: NewSubject);
    fn update(&self, subject: &Subject);
    fn delete(&self, id: i32);
    fn get_all(&self) -> Vec<Subject>;
}
