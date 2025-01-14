// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_target_group::_update_target_group_output::UpdateTargetGroupOutputBuilder;

pub use crate::operation::update_target_group::_update_target_group_input::UpdateTargetGroupInputBuilder;

/// Fluent builder constructing a request to `UpdateTargetGroup`.
///
/// <p>Updates the specified target group.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTargetGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_target_group::builders::UpdateTargetGroupInputBuilder,
}
impl UpdateTargetGroupFluentBuilder {
    /// Creates a new `UpdateTargetGroup`.
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
            crate::operation::update_target_group::UpdateTargetGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_target_group::UpdateTargetGroupError,
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
        crate::operation::update_target_group::UpdateTargetGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_target_group::UpdateTargetGroupError,
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
    /// <p>The ID or Amazon Resource Name (ARN) of the target group.</p>
    pub fn target_group_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_group_identifier(input.into());
        self
    }
    /// <p>The ID or Amazon Resource Name (ARN) of the target group.</p>
    pub fn set_target_group_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_target_group_identifier(input);
        self
    }
    /// <p>The health check configuration.</p>
    pub fn health_check(mut self, input: crate::types::HealthCheckConfig) -> Self {
        self.inner = self.inner.health_check(input);
        self
    }
    /// <p>The health check configuration.</p>
    pub fn set_health_check(
        mut self,
        input: std::option::Option<crate::types::HealthCheckConfig>,
    ) -> Self {
        self.inner = self.inner.set_health_check(input);
        self
    }
}
