use crate::application::config::Config;
use crate::infrastructure::adapters::kratos::KratosClient;
use crate::presentation::graphql::mutations::recovery_mutation::RecoveryMutation;
use crate::presentation::graphql::mutations::register_mutation::RegisterMutation;
use crate::presentation::graphql::mutations::verify_mutation::VerificationMutation;
use crate::presentation::graphql::queries::current_user_query::CurrentUserQuery;
use crate::presentation::graphql::{
    mutations::login_mutation::LoginMutation, queries::logout_query::LogoutQuery,
};
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(CurrentUserQuery, LogoutQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    RegisterMutation,
    LoginMutation,
    RecoveryMutation,
    VerificationMutation,
);

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
