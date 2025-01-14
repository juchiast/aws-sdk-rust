// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::exchange_code_for_token::_exchange_code_for_token_output::ExchangeCodeForTokenOutputBuilder;

pub use crate::operation::exchange_code_for_token::_exchange_code_for_token_input::ExchangeCodeForTokenInputBuilder;

/// Fluent builder constructing a request to `ExchangeCodeForToken`.
///
/// <p>Exchanges an access code for a token.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ExchangeCodeForTokenFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::exchange_code_for_token::builders::ExchangeCodeForTokenInputBuilder,
}
impl ExchangeCodeForTokenFluentBuilder {
    /// Creates a new `ExchangeCodeForToken`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::exchange_code_for_token::ExchangeCodeForToken,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::exchange_code_for_token::ExchangeCodeForTokenError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::exchange_code_for_token::ExchangeCodeForTokenOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::exchange_code_for_token::ExchangeCodeForTokenError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The third-party provider for the token. The only valid value is <code>figma</code>.</p>
    pub fn provider(mut self, input: crate::types::TokenProviders) -> Self {
        self.inner = self.inner.provider(input);
        self
    }
    /// <p>The third-party provider for the token. The only valid value is <code>figma</code>.</p>
    pub fn set_provider(
        mut self,
        input: std::option::Option<crate::types::TokenProviders>,
    ) -> Self {
        self.inner = self.inner.set_provider(input);
        self
    }
    /// <p>Describes the configuration of the request.</p>
    pub fn request(mut self, input: crate::types::ExchangeCodeForTokenRequestBody) -> Self {
        self.inner = self.inner.request(input);
        self
    }
    /// <p>Describes the configuration of the request.</p>
    pub fn set_request(
        mut self,
        input: std::option::Option<crate::types::ExchangeCodeForTokenRequestBody>,
    ) -> Self {
        self.inner = self.inner.set_request(input);
        self
    }
}
