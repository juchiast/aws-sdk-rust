// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_recovery_group::_get_recovery_group_output::GetRecoveryGroupOutputBuilder;

pub use crate::operation::get_recovery_group::_get_recovery_group_input::GetRecoveryGroupInputBuilder;

/// Fluent builder constructing a request to `GetRecoveryGroup`.
///
/// <p>Gets details about a recovery group, including a list of the cells that are included in it.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetRecoveryGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_recovery_group::builders::GetRecoveryGroupInputBuilder,
}
impl GetRecoveryGroupFluentBuilder {
    /// Creates a new `GetRecoveryGroup`.
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
            crate::operation::get_recovery_group::GetRecoveryGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_recovery_group::GetRecoveryGroupError,
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
        crate::operation::get_recovery_group::GetRecoveryGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_recovery_group::GetRecoveryGroupError,
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
    /// <p>The name of a recovery group.</p>
    pub fn recovery_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.recovery_group_name(input.into());
        self
    }
    /// <p>The name of a recovery group.</p>
    pub fn set_recovery_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_recovery_group_name(input);
        self
    }
}
