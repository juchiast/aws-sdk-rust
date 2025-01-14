// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_mobile_device_access_effect::_get_mobile_device_access_effect_output::GetMobileDeviceAccessEffectOutputBuilder;

pub use crate::operation::get_mobile_device_access_effect::_get_mobile_device_access_effect_input::GetMobileDeviceAccessEffectInputBuilder;

/// Fluent builder constructing a request to `GetMobileDeviceAccessEffect`.
///
/// <p>Simulates the effect of the mobile device access rules for the given attributes of a sample access event. Use this method to test the effects of the current set of mobile device access rules for the WorkMail organization for a particular user's attributes.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetMobileDeviceAccessEffectFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_mobile_device_access_effect::builders::GetMobileDeviceAccessEffectInputBuilder
            }
impl GetMobileDeviceAccessEffectFluentBuilder {
    /// Creates a new `GetMobileDeviceAccessEffect`.
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
            crate::operation::get_mobile_device_access_effect::GetMobileDeviceAccessEffect,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_mobile_device_access_effect::GetMobileDeviceAccessEffectError,
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
        crate::operation::get_mobile_device_access_effect::GetMobileDeviceAccessEffectOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_mobile_device_access_effect::GetMobileDeviceAccessEffectError,
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
    /// <p>The WorkMail organization to simulate the access effect for.</p>
    pub fn organization_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The WorkMail organization to simulate the access effect for.</p>
    pub fn set_organization_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>Device type the simulated user will report.</p>
    pub fn device_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_type(input.into());
        self
    }
    /// <p>Device type the simulated user will report.</p>
    pub fn set_device_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_device_type(input);
        self
    }
    /// <p>Device model the simulated user will report.</p>
    pub fn device_model(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_model(input.into());
        self
    }
    /// <p>Device model the simulated user will report.</p>
    pub fn set_device_model(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_device_model(input);
        self
    }
    /// <p>Device operating system the simulated user will report.</p>
    pub fn device_operating_system(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_operating_system(input.into());
        self
    }
    /// <p>Device operating system the simulated user will report.</p>
    pub fn set_device_operating_system(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_device_operating_system(input);
        self
    }
    /// <p>Device user agent the simulated user will report.</p>
    pub fn device_user_agent(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_user_agent(input.into());
        self
    }
    /// <p>Device user agent the simulated user will report.</p>
    pub fn set_device_user_agent(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_device_user_agent(input);
        self
    }
}
