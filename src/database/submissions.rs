use super::models;
use super::schema::submissions::dsl;
use crate::repository::submissions::*;
use diesel::prelude::*;
use md5;
use uuid::Uuid;

pub struct SubmissionDB<'a> {
    connection: &'a PgConnection,
}

impl<'a> ISubmissionDB for SubmissionDB<'a> {
    fn get_all(&self, testcase_id: i32) -> Vec<SavedSubmission> {
        dsl::submissions
            .filter(dsl::testcase_id.eq(testcase_id))
            .load::<models::Submission>(self.connection)
            .expect("Unable to get_all submissions.")
            .into_iter()
            .map(|model| SavedSubmission {
                id: model.id,
                user_id: model.user_id,
                testcase_id: model.testcase_id,
                content: model.content,
                hash1: model.hash1,
                hash2: model.hash2,
            })
            .collect()
    }

    fn submit(&self, submission: &NewSubmission) {
        let content = &submission.content;
        let mut content1 = String::new();
        for line in content.lines() {
            content1 += line.trim();
            content1.push('\n');
        }
        let hash1 = md5::compute(content1);
        let content2 = content.replace(" ", "").replace("\n", "");
        let hash2 = md5::compute(content2);

        // Note: Diesel forces me to use uuid crate version 0.6
        //       v0.7 provides from slice method
        //       I don't think `from_random_bytes` care where bytes come from.
        let hash1 = Uuid::from_random_bytes(hash1.0);
        let hash2 = Uuid::from_random_bytes(hash2.0);

        diesel::insert_into(dsl::submissions)
            .values(models::NewSubmission {
                user_id: submission.user_id,
                testcase_id: submission.testcase_id,
                content: submission.content.clone(),
                hash1,
                hash2,
            })
            .execute(self.connection)
            .expect("Unable to insert submission.");
    }

    fn delete(&self, submission_id: i32, user_id: i32) -> Result<(), DeleteSubmissionError> {
        let model = dsl::submissions
            .find(submission_id)
            .get_result::<models::Submission>(self.connection)
            .optional()
            .expect("Delete submission: error querying.");
        if model.is_none() {
            return Err(DeleteSubmissionError::NotFound);
        }
        let model = model.unwrap();
        if model.user_id != user_id {
            return Err(DeleteSubmissionError::NotAuthorized);
        }
        diesel::delete(dsl::submissions.find(submission_id))
            .execute(self.connection)
            .expect("Unable to delete submission.");
        Ok(())
    }
}
