// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_smb_guest_password::_set_smb_guest_password_output::SetSmbGuestPasswordOutputBuilder;

pub use crate::operation::set_smb_guest_password::_set_smb_guest_password_input::SetSmbGuestPasswordInputBuilder;

/// Fluent builder constructing a request to `SetSMBGuestPassword`.
///
/// <p>Sets the password for the guest user <code>smbguest</code>. The <code>smbguest</code> user is the user when the authentication method for the file share is set to <code>GuestAccess</code>. This operation only supported for S3 File Gateways</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SetSMBGuestPasswordFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_smb_guest_password::builders::SetSmbGuestPasswordInputBuilder,
}
impl SetSMBGuestPasswordFluentBuilder {
    /// Creates a new `SetSMBGuestPassword`.
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
            crate::operation::set_smb_guest_password::SetSMBGuestPassword,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::set_smb_guest_password::SetSMBGuestPasswordError,
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
        crate::operation::set_smb_guest_password::SetSmbGuestPasswordOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::set_smb_guest_password::SetSMBGuestPasswordError,
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
    /// <p>The Amazon Resource Name (ARN) of the S3 File Gateway the SMB file share is associated with.</p>
    pub fn gateway_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the S3 File Gateway the SMB file share is associated with.</p>
    pub fn set_gateway_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The password that you want to set for your SMB server.</p>
    pub fn password(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.password(input.into());
        self
    }
    /// <p>The password that you want to set for your SMB server.</p>
    pub fn set_password(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_password(input);
        self
    }
}
