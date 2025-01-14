// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_customer_managed_policy_reference_to_permission_set::_attach_customer_managed_policy_reference_to_permission_set_output::AttachCustomerManagedPolicyReferenceToPermissionSetOutputBuilder;

pub use crate::operation::attach_customer_managed_policy_reference_to_permission_set::_attach_customer_managed_policy_reference_to_permission_set_input::AttachCustomerManagedPolicyReferenceToPermissionSetInputBuilder;

/// Fluent builder constructing a request to `AttachCustomerManagedPolicyReferenceToPermissionSet`.
///
/// <p>Attaches the specified customer managed policy to the specified <code>PermissionSet</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AttachCustomerManagedPolicyReferenceToPermissionSetFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::attach_customer_managed_policy_reference_to_permission_set::builders::AttachCustomerManagedPolicyReferenceToPermissionSetInputBuilder
            }
impl AttachCustomerManagedPolicyReferenceToPermissionSetFluentBuilder {
    /// Creates a new `AttachCustomerManagedPolicyReferenceToPermissionSet`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSet, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetOutput, aws_smithy_http::result::SdkError<crate::operation::attach_customer_managed_policy_reference_to_permission_set::AttachCustomerManagedPolicyReferenceToPermissionSetError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p>
    pub fn instance_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p>
    pub fn set_instance_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code>.</p>
    pub fn permission_set_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.permission_set_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code>.</p>
    pub fn set_permission_set_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_permission_set_arn(input);
        self
    }
    /// <p>Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each AWS account where you want to deploy your permission set.</p>
    pub fn customer_managed_policy_reference(
        mut self,
        input: crate::types::CustomerManagedPolicyReference,
    ) -> Self {
        self.inner = self.inner.customer_managed_policy_reference(input);
        self
    }
    /// <p>Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each AWS account where you want to deploy your permission set.</p>
    pub fn set_customer_managed_policy_reference(
        mut self,
        input: std::option::Option<crate::types::CustomerManagedPolicyReference>,
    ) -> Self {
        self.inner = self.inner.set_customer_managed_policy_reference(input);
        self
    }
}
