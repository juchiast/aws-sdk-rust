// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_confirm_sign_up::_admin_confirm_sign_up_output::AdminConfirmSignUpOutputBuilder;

pub use crate::operation::admin_confirm_sign_up::_admin_confirm_sign_up_input::AdminConfirmSignUpInputBuilder;

/// Fluent builder constructing a request to `AdminConfirmSignUp`.
///
/// <p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p>
/// <p>Calling this action requires developer credentials.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AdminConfirmSignUpFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_confirm_sign_up::builders::AdminConfirmSignUpInputBuilder,
}
impl AdminConfirmSignUpFluentBuilder {
    /// Creates a new `AdminConfirmSignUp`.
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
            crate::operation::admin_confirm_sign_up::AdminConfirmSignUp,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::admin_confirm_sign_up::AdminConfirmSignUpError,
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
        crate::operation::admin_confirm_sign_up::AdminConfirmSignUpOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::admin_confirm_sign_up::AdminConfirmSignUpError,
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
    /// <p>The user pool ID for which you want to confirm user registration.</p>
    pub fn user_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID for which you want to confirm user registration.</p>
    pub fn set_user_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user name for which you want to confirm user registration.</p>
    pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name for which you want to confirm user registration.</p>
    pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// Adds a key-value pair to `ClientMetadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>If your user pool configuration includes triggers, the AdminConfirmSignUp API action invokes the Lambda function that is specified for the <i>post confirmation</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. In this payload, the <code>clientMetadata</code> attribute provides the data that you assigned to the ClientMetadata parameter in your AdminConfirmSignUp request. In your function code in Lambda, you can process the ClientMetadata value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_metadata(k.into(), v.into());
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>If your user pool configuration includes triggers, the AdminConfirmSignUp API action invokes the Lambda function that is specified for the <i>post confirmation</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. In this payload, the <code>clientMetadata</code> attribute provides the data that you assigned to the ClientMetadata parameter in your AdminConfirmSignUp request. In your function code in Lambda, you can process the ClientMetadata value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn set_client_metadata(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_client_metadata(input);
        self
    }
}
