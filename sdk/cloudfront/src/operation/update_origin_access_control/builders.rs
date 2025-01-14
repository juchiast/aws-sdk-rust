// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_origin_access_control::_update_origin_access_control_output::UpdateOriginAccessControlOutputBuilder;

pub use crate::operation::update_origin_access_control::_update_origin_access_control_input::UpdateOriginAccessControlInputBuilder;

/// Fluent builder constructing a request to `UpdateOriginAccessControl`.
///
/// <p>Updates a CloudFront origin access control.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateOriginAccessControlFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_origin_access_control::builders::UpdateOriginAccessControlInputBuilder
            }
impl UpdateOriginAccessControlFluentBuilder {
    /// Creates a new `UpdateOriginAccessControl`.
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
            crate::operation::update_origin_access_control::UpdateOriginAccessControl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
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
        crate::operation::update_origin_access_control::UpdateOriginAccessControlOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
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
    /// <p>An origin access control.</p>
    pub fn origin_access_control_config(
        mut self,
        input: crate::types::OriginAccessControlConfig,
    ) -> Self {
        self.inner = self.inner.origin_access_control_config(input);
        self
    }
    /// <p>An origin access control.</p>
    pub fn set_origin_access_control_config(
        mut self,
        input: std::option::Option<crate::types::OriginAccessControlConfig>,
    ) -> Self {
        self.inner = self.inner.set_origin_access_control_config(input);
        self
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn if_match(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn set_if_match(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
}
