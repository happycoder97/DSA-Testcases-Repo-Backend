macro_rules! impl_crud {
    ($DB:ident<$backend: path>, $Model: ident, $NewModel:ident, $table:path) => {
        impl<C> CRUD<$Model, $NewModel> for $DB<C>
        where
            C: Connection<Backend = $backend>,
        {
            fn create(&self, new_model: &$NewModel) {
                diesel::insert_into($table)
                    .values(new_model)
                    .execute(&self.connection)
                    .expect("Error inserting model.");
            }

            fn get(&self, id: i32) -> Option<$Model> {
                $table.find(id).first::<$Model>(&self.connection).ok()
            }

            fn get_all(&self) -> Vec<$Model> {
                $table
                    .load::<$Model>(&self.connection)
                    .expect("Failed to get_all models.")
                    .into_iter()
                    .collect()
            }

            fn update(&self, model: &$Model) {
                diesel::update($table.find(model.id))
                    .set(model)
                    .execute(&self.connection)
                    .expect("Failed to update model.");
            }

            fn delete(&self, id: i32) {
                diesel::delete($table.find(id))
                    .execute(&self.connection)
                    .expect("Failed to delete model.");
            }
        }
    };
}
