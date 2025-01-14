// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_custom_plugin::_create_custom_plugin_output::CreateCustomPluginOutputBuilder;

pub use crate::operation::create_custom_plugin::_create_custom_plugin_input::CreateCustomPluginInputBuilder;

/// Fluent builder constructing a request to `CreateCustomPlugin`.
///
/// <p>Creates a custom plugin using the specified properties.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateCustomPluginFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_custom_plugin::builders::CreateCustomPluginInputBuilder,
}
impl CreateCustomPluginFluentBuilder {
    /// Creates a new `CreateCustomPlugin`.
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
            crate::operation::create_custom_plugin::CreateCustomPlugin,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_custom_plugin::CreateCustomPluginError,
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
        crate::operation::create_custom_plugin::CreateCustomPluginOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_custom_plugin::CreateCustomPluginError,
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
    /// <p>The type of the plugin file.</p>
    pub fn content_type(mut self, input: crate::types::CustomPluginContentType) -> Self {
        self.inner = self.inner.content_type(input);
        self
    }
    /// <p>The type of the plugin file.</p>
    pub fn set_content_type(
        mut self,
        input: std::option::Option<crate::types::CustomPluginContentType>,
    ) -> Self {
        self.inner = self.inner.set_content_type(input);
        self
    }
    /// <p>A summary description of the custom plugin.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A summary description of the custom plugin.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Information about the location of a custom plugin.</p>
    pub fn location(mut self, input: crate::types::CustomPluginLocation) -> Self {
        self.inner = self.inner.location(input);
        self
    }
    /// <p>Information about the location of a custom plugin.</p>
    pub fn set_location(
        mut self,
        input: std::option::Option<crate::types::CustomPluginLocation>,
    ) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The name of the custom plugin.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the custom plugin.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
}
