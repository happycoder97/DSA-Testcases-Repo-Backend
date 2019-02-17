macro_rules! model_struct {
        ($(
            pub struct $struct:ident, $new_struct:ident [$table:literal] {
                $(pub $field:ident: $type:ty),+
            }
        )+) => {
            $(
                use crate::database::schema::*;
                #[derive(Queryable, AsChangeset)]
                #[table_name=$table]
                pub struct $struct {
                    pub id: i32,
                    $(pub $field: $type),+
                }

                #[derive(Insertable)]
                #[table_name=$table]
                pub struct $new_struct {
                    $(pub $field: $type),+
                }
            )+
        }
}
