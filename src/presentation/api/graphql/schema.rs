use crate::application::config::Config;
use crate::application::usecases::auth::get_current_user::GetCurrentUserUseCase;
use crate::application::usecases::auth::login::LoginUseCase;
use crate::application::usecases::auth::logout::LogoutUseCase;
use crate::application::usecases::auth::recovery::RecoveryUseCase;
use crate::application::usecases::auth::register::RegisterUseCase;
use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::adapters::kratos::KratosClient;
use crate::infrastructure::adapters::kratos::http::identity::KratosIdentityAdapter;
use crate::infrastructure::adapters::kratos::http::login::KratosAuthenticationAdapter;
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use crate::infrastructure::adapters::kratos::http::recovery::KratosRecoveryAdapter;
use crate::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;
use crate::infrastructure::adapters::kratos::http::verification::KratosVerificationAdapter;
use crate::presentation::api::graphql::mutations::recovery_mutation::RecoveryMutation;
use crate::presentation::api::graphql::mutations::register_mutation::RegisterMutation;
use crate::presentation::api::graphql::mutations::verify_mutation::VerificationMutation;
use crate::presentation::api::graphql::queries::current_user_query::CurrentUserQuery;
use crate::presentation::api::graphql::{
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

    let registration_adapter = KratosRegistrationAdapter::new(kratos_client.clone());
    let auth_adapter = KratosAuthenticationAdapter::new(kratos_client.clone());
    let session_adapter = KratosSessionAdapter::new(kratos_client.clone());
    let recovery_adapter = KratosRecoveryAdapter::new(kratos_client.clone());
    let verification_adapter = KratosVerificationAdapter::new(kratos_client.clone());
    let identity_adapter = KratosIdentityAdapter::new(kratos_client.clone());

    let register_use_case = RegisterUseCase::new(Box::new(registration_adapter));
    let login_use_case = LoginUseCase::new(Box::new(auth_adapter));
    let logout_use_case = LogoutUseCase::new(Box::new(session_adapter));
    let recovery_use_case = RecoveryUseCase::new(Box::new(recovery_adapter));
    let verification_use_case = VerificationUseCase::new(Box::new(verification_adapter));
    let get_current_user_use_case = GetCurrentUserUseCase::new(Box::new(identity_adapter));

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(kratos_client)
    .data(register_use_case)
    .data(login_use_case)
    .data(logout_use_case)
    .data(recovery_use_case)
    .data(verification_use_case)
    .data(get_current_user_use_case)
    .finish()
}
