#[cfg(test)]
mod database_tests {
    use crate::{
        database::{
            database_context::DatabaseContext, entity_context::EntityContextTrait,
            model_context::ModelContext, query_context::QueryContext, user_context::UserContext,
        },
        entities::{model, query, user},
    };
    use sea_orm::{
        entity::prelude::*, sea_query::TableCreateStatement, Database, DatabaseBackend,
        DatabaseConnection, Schema,
    };

    async fn setup_schema(db: &DatabaseConnection) {
        // Setup Schema helper
        let schema = Schema::new(DatabaseBackend::Sqlite);

        // Derive from Entity
        let stmt: TableCreateStatement = schema.create_table_from_entity(user::Entity);
        let _ = db.execute(db.get_database_backend().build(&stmt)).await;
        let stmt: TableCreateStatement = schema.create_table_from_entity(model::Entity);
        let _ = db.execute(db.get_database_backend().build(&stmt)).await;
        let stmt: TableCreateStatement = schema.create_table_from_entity(query::Entity);
        let _ = db.execute(db.get_database_backend().build(&stmt)).await;
    }

    #[tokio::test]
    async fn create_test() -> Result<(), DbErr> {
        let db_connection = Database::connect("sqlite::memory:").await.unwrap();
        setup_schema(&db_connection).await;
        setup_schema(&db_connection).await;
        let db_context = Box::new(DatabaseContext { db_connection });

        let user_context = UserContext::new(db_context.clone());
        let model_context = ModelContext::new(db_context.clone());
        let query_context = QueryContext::new(db_context.clone());

        let user = user::Model {
            id: 1,
            email: "test@test.com".to_string(),
            username: "anders".to_string(),
            password: "qwerty".to_string(),
        };
        user_context.create(user).await?;

        let model = model::Model {
            id: 1,
            name: "Test".to_string(),
            components_info: "{}".to_owned().parse().unwrap(),
            owner_id: 1,
        };
        model_context.create(model).await?;

        let new_query = query::Model {
            id: 1,
            string: "query_string".to_owned(),
            result: Some("{}".to_owned().parse().unwrap()),
            model_id: 1,
            outdated: false,
        };
        let created_query = query_context.create(new_query).await?;

        let fetched_query = query::Entity::find_by_id(created_query.id)
            .one(&query_context.db_context.get_connection())
            .await?
            .clone()
            .unwrap();

        assert_eq!(fetched_query.id, created_query.id);
        assert_eq!(fetched_query.model_id, created_query.model_id);
        assert_eq!(fetched_query.string, created_query.string);

        Ok(())
    }

    #[tokio::test]
    async fn update_test() -> Result<(), DbErr> {
        let db_connection = Database::connect("sqlite::memory:").await.unwrap();
        setup_schema(&db_connection).await;
        setup_schema(&db_connection).await;
        let db_context = Box::new(DatabaseContext { db_connection });

        let user_context = UserContext::new(db_context.clone());
        let model_context = ModelContext::new(db_context.clone());
        let query_context = QueryContext::new(db_context.clone());

        let user = user::Model {
            id: 1,
            email: "test@test.com".to_string(),
            username: "anders".to_string(),
            password: "qwerty".to_string(),
        };
        user_context.create(user).await?;

        let model = model::Model {
            id: 1,
            name: "Test".to_string(),
            components_info: "{}".to_owned().parse().unwrap(),
            owner_id: 1,
        };
        model_context.create(model).await?;

        let new_query = query::Model {
            id: 1,
            string: "query_string".to_owned(),
            result: Some("{}".to_owned().parse().unwrap()),
            model_id: 1,
            outdated: false,
        };
        let created_query = query_context.create(new_query).await?;

        let fetched_query = query::Entity::find_by_id(created_query.id)
            .one(&query_context.db_context.get_connection())
            .await?
            .clone()
            .unwrap();

        let updated_query = query::Model {
            id: fetched_query.id,
            string: "updated query string".to_owned(),
            result: fetched_query.result,
            model_id: fetched_query.model_id,
            outdated: true,
        };

        let result = query_context.update(updated_query).await?;

        assert_eq!(result.id, created_query.id);
        assert_ne!(result.string, created_query.string);
        assert_ne!(result.outdated, created_query.outdated);

        Ok(())
    }
}
