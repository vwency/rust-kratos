use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::domain::auth::inputs::{
    SendVerificationCodeInput, SubmitVerificationCodeInput, VerifyByLinkInput,
};
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct VerificationMutation;

#[Object]
impl VerificationMutation {
    async fn verify_by_link(&self, ctx: &Context<'_>, input: VerifyByLinkInput) -> Result<bool> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let cookies = VerificationUseCase::execute_link(
            &input.email,
            kratos_client,
            cookie,
            input.transient_payload,
        )
        .await
        .map_err(async_graphql::Error::new)?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            for cookie_str in cookies {
                response_cookies.add_cookie(cookie_str).await;
            }
        }

        Ok(true)
    }

    async fn send_verification_code(
        &self,
        ctx: &Context<'_>,
        input: SendVerificationCodeInput,
    ) -> Result<bool> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let cookies = VerificationUseCase::execute_code_send(
            &input.email,
            kratos_client,
            cookie,
            input.transient_payload,
        )
        .await
        .map_err(async_graphql::Error::new)?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            for cookie_str in cookies {
                response_cookies.add_cookie(cookie_str).await;
            }
        }

        Ok(true)
    }

    async fn submit_verification_code(
        &self,
        ctx: &Context<'_>,
        input: SubmitVerificationCodeInput,
    ) -> Result<bool> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .ok_or_else(|| {
                async_graphql::Error::new("Cookie is required to submit verification code")
            })?;

        let cookies = VerificationUseCase::execute_code_submit(
            &input.code,
            kratos_client,
            cookie,
            input.transient_payload,
        )
        .await
        .map_err(async_graphql::Error::new)?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            for cookie_str in cookies {
                response_cookies.add_cookie(cookie_str).await;
            }
        }

        Ok(true)
    }
}
