mod api;
mod database;
mod entities;

use api::server::start_grpc_server;
use dotenv::dotenv;
use crate::database::access_context::AccessContext;
use crate::database::model_context::ModelContext;
use crate::database::query_context::QueryContext;
use crate::database::session_context::SessionContext;
use crate::database::user_context::UserContext;
use database::database_context::{DatabaseContext, DatabaseContextTrait};
use database::entity_context::EntityContextTrait;
use std::error::Error;
use std::fmt::Debug;
use crate::database::in_use_context::InUseContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_context = Box::new(DatabaseContext::new().await?);
    let model_context = Box::new(ModelContext::new(db_context.clone()));
    let user_context = Box::new(UserContext::new(db_context.clone()));
    let access_context = Box::new(AccessContext::new(db_context.clone()));
    let query_context = Box::new(QueryContext::new(db_context.clone()));
    let session_context = Box::new(SessionContext::new(db_context.clone()));
    let in_use_context = Box::new(InUseContext::new(db_context.clone()));
    
    print_all_entities(model_context).await;
    print_all_entities(user_context).await;
    print_all_entities(access_context).await;
    print_all_entities(query_context).await;
    print_all_entities(session_context).await;
    print_all_entities(in_use_context).await;
    
    start_grpc_server().await.unwrap();
    
    Ok(())
}

async fn print_all_entities<T: Debug>(entity_context: Box<dyn EntityContextTrait<T>>) {
    let res = entity_context.get_all().await;
    println!("{:?}", res.unwrap())
}
