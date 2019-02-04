use super::models;
use super::schema::subjects;
use crate::repository::subjects::*;
use diesel::prelude::*;

pub struct SubjectDB<'conn> {
    connection: &'conn PgConnection,
}

impl<'conn> ISubjectDB for SubjectDB<'conn> {
    fn add_subject(&self, title: String) {
        let subject_model = models::NewSubject { title };
        let _: models::Subject = diesel::insert_into(subjects::table)
            .values(subject_model)
            .get_result(self.connection)
            .expect("Error inserting subject.");
    }
    fn delete_subject(&self, id: i32) {
        diesel::delete(subjects::table.find(id))
            .execute(self.connection)
            .expect("Failed to delete subject.");
    }
    fn update_subject(&self, id: i32, title: String) {
        diesel::update(subjects::table.find(id))
            .set(subjects::title.eq(title))
            .execute(self.connection)
            .expect("Failed to update subject");
    }
    fn get_all(&self) -> Vec<Subject> {
        subjects::table
            .load::<models::Subject>(self.connection)
            .expect("Failed to get_all subjects.")
            .into_iter()
            .map(|subject_model| Subject {
                id: subject_model.id,
                title: subject_model.title,
            })
            .collect()
    }
}
