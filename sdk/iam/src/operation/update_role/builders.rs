// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_role::_update_role_output::UpdateRoleOutputBuilder;

pub use crate::operation::update_role::_update_role_input::UpdateRoleInputBuilder;

/// Fluent builder constructing a request to `UpdateRole`.
///
/// <p>Updates the description or maximum session duration setting of a role.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRoleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_role::builders::UpdateRoleInputBuilder,
}
impl UpdateRoleFluentBuilder {
    /// Creates a new `UpdateRole`.
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
            crate::operation::update_role::UpdateRole,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_role::UpdateRoleError>,
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
        crate::operation::update_role::UpdateRoleOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_role::UpdateRoleError>,
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
    /// <p>The name of the role that you want to modify.</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_name(input.into());
        self
    }
    /// <p>The name of the role that you want to modify.</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_name(input);
        self
    }
    /// <p>The new description that you want to apply to the specified role.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The new description that you want to apply to the specified role.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role. If you do not specify a value for this setting, the default value of one hour is applied. This setting can have a value from 1 hour to 12 hours.</p>
    /// <p>Anyone who assumes the role from the CLI or API can use the <code>DurationSeconds</code> API parameter or the <code>duration-seconds</code> CLI parameter to request a longer session. The <code>MaxSessionDuration</code> setting determines the maximum duration that can be requested using the <code>DurationSeconds</code> parameter. If users don't specify a value for the <code>DurationSeconds</code> parameter, their security credentials are valid for one hour by default. This applies when you use the <code>AssumeRole*</code> API operations or the <code>assume-role*</code> CLI operations but does not apply when you use those operations to create a console URL. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html">Using IAM roles</a> in the <i>IAM User Guide</i>.</p>
    pub fn max_session_duration(mut self, input: i32) -> Self {
        self.inner = self.inner.max_session_duration(input);
        self
    }
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role. If you do not specify a value for this setting, the default value of one hour is applied. This setting can have a value from 1 hour to 12 hours.</p>
    /// <p>Anyone who assumes the role from the CLI or API can use the <code>DurationSeconds</code> API parameter or the <code>duration-seconds</code> CLI parameter to request a longer session. The <code>MaxSessionDuration</code> setting determines the maximum duration that can be requested using the <code>DurationSeconds</code> parameter. If users don't specify a value for the <code>DurationSeconds</code> parameter, their security credentials are valid for one hour by default. This applies when you use the <code>AssumeRole*</code> API operations or the <code>assume-role*</code> CLI operations but does not apply when you use those operations to create a console URL. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html">Using IAM roles</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_max_session_duration(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_session_duration(input);
        self
    }
}
