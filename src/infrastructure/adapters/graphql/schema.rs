use crate::application::config::Config;
use crate::application::graphql::mutations::recovery_mutation::RecoveryMutation;
use crate::application::graphql::mutations::register_mutation::RegisterMutation;
use crate::application::graphql::queries::get_current_user::CurrentUserQuery;
use crate::application::graphql::{
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
