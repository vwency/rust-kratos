use crate::application::config::Config;
use crate::infrastructure::adapters::graphql::mutations::recovery_mutation::RecoveryMutation;
use crate::infrastructure::adapters::graphql::mutations::register_mutation::RegisterMutation;
use crate::infrastructure::adapters::graphql::queries::current_user_query::CurrentUserQuery;
use crate::infrastructure::adapters::graphql::{
    mutations::login_mutation::LoginMutation, queries::logout_query::LogoutQuery,
};
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(CurrentUserQuery, LogoutQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(RegisterMutation, LoginMutation, RecoveryMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(config: &Config) -> AppSchema {
    let kratos_client = KratosClient::new(&config.kratos);

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(kratos_client)
    .finish()
}
