// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_type_configuration::_set_type_configuration_output::SetTypeConfigurationOutputBuilder;

pub use crate::operation::set_type_configuration::_set_type_configuration_input::SetTypeConfigurationInputBuilder;

/// Fluent builder constructing a request to `SetTypeConfiguration`.
///
/// <p>Specifies the configuration data for a registered CloudFormation extension, in the given account and region.</p>
/// <p>To view the current configuration data for an extension, refer to the <code>ConfigurationSchema</code> element of <a href="AWSCloudFormation/latest/APIReference/API_DescribeType.html">DescribeType</a>. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/registry-register.html#registry-set-configuration">Configuring extensions at the account level</a> in the <i>CloudFormation User Guide</i>.</p> <important>
/// <p>It's strongly recommended that you use dynamic references to restrict sensitive configuration definitions, such as third-party credentials. For more details on dynamic references, see <a href="https://docs.aws.amazon.com/">Using dynamic references to specify template values</a> in the <i>CloudFormation User Guide</i>.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SetTypeConfigurationFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_type_configuration::builders::SetTypeConfigurationInputBuilder,
}
impl SetTypeConfigurationFluentBuilder {
    /// Creates a new `SetTypeConfiguration`.
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
            crate::operation::set_type_configuration::SetTypeConfiguration,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::set_type_configuration::SetTypeConfigurationError,
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
        crate::operation::set_type_configuration::SetTypeConfigurationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::set_type_configuration::SetTypeConfigurationError,
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
    /// <p>The Amazon Resource Name (ARN) for the extension, in this account and region.</p>
    /// <p>For public extensions, this will be the ARN assigned when you <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ActivateType.html">activate the type</a> in this account and region. For private extensions, this will be the ARN assigned when you <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_RegisterType.html">register the type</a> in this account and region.</p>
    /// <p>Do not include the extension versions suffix at the end of the ARN. You can set the configuration for an extension, but not for a specific extension version.</p>
    pub fn type_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.type_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the extension, in this account and region.</p>
    /// <p>For public extensions, this will be the ARN assigned when you <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ActivateType.html">activate the type</a> in this account and region. For private extensions, this will be the ARN assigned when you <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_RegisterType.html">register the type</a> in this account and region.</p>
    /// <p>Do not include the extension versions suffix at the end of the ARN. You can set the configuration for an extension, but not for a specific extension version.</p>
    pub fn set_type_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_type_arn(input);
        self
    }
    /// <p>The configuration data for the extension, in this account and region.</p>
    /// <p>The configuration data must be formatted as JSON, and validate against the schema returned in the <code>ConfigurationSchema</code> response element of <a href="AWSCloudFormation/latest/APIReference/API_DescribeType.html">API_DescribeType</a>. For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-model.html#resource-type-howto-configuration">Defining account-level configuration data for an extension</a> in the <i>CloudFormation CLI User Guide</i>.</p>
    pub fn configuration(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration(input.into());
        self
    }
    /// <p>The configuration data for the extension, in this account and region.</p>
    /// <p>The configuration data must be formatted as JSON, and validate against the schema returned in the <code>ConfigurationSchema</code> response element of <a href="AWSCloudFormation/latest/APIReference/API_DescribeType.html">API_DescribeType</a>. For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-model.html#resource-type-howto-configuration">Defining account-level configuration data for an extension</a> in the <i>CloudFormation CLI User Guide</i>.</p>
    pub fn set_configuration(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>An alias by which to refer to this extension configuration data.</p>
    /// <p>Conditional: Specifying a configuration alias is required when setting a configuration for a resource type extension.</p>
    pub fn configuration_alias(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration_alias(input.into());
        self
    }
    /// <p>An alias by which to refer to this extension configuration data.</p>
    /// <p>Conditional: Specifying a configuration alias is required when setting a configuration for a resource type extension.</p>
    pub fn set_configuration_alias(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_alias(input);
        self
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify <code>ConfigurationArn</code>, or <code>Type</code> and <code>TypeName</code>.</p>
    pub fn type_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.type_name(input.into());
        self
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify <code>ConfigurationArn</code>, or <code>Type</code> and <code>TypeName</code>.</p>
    pub fn set_type_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_type_name(input);
        self
    }
    /// <p>The type of extension.</p>
    /// <p>Conditional: You must specify <code>ConfigurationArn</code>, or <code>Type</code> and <code>TypeName</code>.</p>
    pub fn r#type(mut self, input: crate::types::ThirdPartyType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of extension.</p>
    /// <p>Conditional: You must specify <code>ConfigurationArn</code>, or <code>Type</code> and <code>TypeName</code>.</p>
    pub fn set_type(mut self, input: std::option::Option<crate::types::ThirdPartyType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
}
