// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_user::_disassociate_user_output::DisassociateUserOutputBuilder;

pub use crate::operation::disassociate_user::_disassociate_user_input::DisassociateUserInputBuilder;

/// Fluent builder constructing a request to `DisassociateUser`.
///
/// <p>Disassociates the user from an EC2 instance providing user-based subscriptions.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateUserFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_user::builders::DisassociateUserInputBuilder,
}
impl DisassociateUserFluentBuilder {
    /// Creates a new `DisassociateUser`.
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
            crate::operation::disassociate_user::DisassociateUser,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disassociate_user::DisassociateUserError,
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
        crate::operation::disassociate_user::DisassociateUserOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disassociate_user::DisassociateUserError,
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
    /// <p>The user name from the identity provider for the user.</p>
    pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name from the identity provider for the user.</p>
    pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The ID of the EC2 instance, which provides user-based subscriptions.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the EC2 instance, which provides user-based subscriptions.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn identity_provider(mut self, input: crate::types::IdentityProvider) -> Self {
        self.inner = self.inner.identity_provider(input);
        self
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn set_identity_provider(
        mut self,
        input: std::option::Option<crate::types::IdentityProvider>,
    ) -> Self {
        self.inner = self.inner.set_identity_provider(input);
        self
    }
    /// <p>The domain name of the user.</p>
    pub fn domain(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The domain name of the user.</p>
    pub fn set_domain(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
}
