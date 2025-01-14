// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_config::_update_config_output::UpdateConfigOutputBuilder;

pub use crate::operation::update_config::_update_config_input::UpdateConfigInputBuilder;

/// Fluent builder constructing a request to `UpdateConfig`.
///
/// <p>Updates the <code>Config</code> used when scheduling contacts.</p>
/// <p>Updating a <code>Config</code> will not update the execution parameters for existing future contacts scheduled with this <code>Config</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConfigFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_config::builders::UpdateConfigInputBuilder,
}
impl UpdateConfigFluentBuilder {
    /// Creates a new `UpdateConfig`.
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
            crate::operation::update_config::UpdateConfig,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_config::UpdateConfigError>,
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
        crate::operation::update_config::UpdateConfigOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_config::UpdateConfigError>,
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
    /// <p>UUID of a <code>Config</code>.</p>
    pub fn config_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.config_id(input.into());
        self
    }
    /// <p>UUID of a <code>Config</code>.</p>
    pub fn set_config_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_config_id(input);
        self
    }
    /// <p>Name of a <code>Config</code>.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Name of a <code>Config</code>.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>Type of a <code>Config</code>.</p>
    pub fn config_type(mut self, input: crate::types::ConfigCapabilityType) -> Self {
        self.inner = self.inner.config_type(input);
        self
    }
    /// <p>Type of a <code>Config</code>.</p>
    pub fn set_config_type(
        mut self,
        input: std::option::Option<crate::types::ConfigCapabilityType>,
    ) -> Self {
        self.inner = self.inner.set_config_type(input);
        self
    }
    /// <p>Parameters of a <code>Config</code>.</p>
    pub fn config_data(mut self, input: crate::types::ConfigTypeData) -> Self {
        self.inner = self.inner.config_data(input);
        self
    }
    /// <p>Parameters of a <code>Config</code>.</p>
    pub fn set_config_data(
        mut self,
        input: std::option::Option<crate::types::ConfigTypeData>,
    ) -> Self {
        self.inner = self.inner.set_config_data(input);
        self
    }
}
