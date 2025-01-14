// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_certificate::_associate_certificate_output::AssociateCertificateOutputBuilder;

pub use crate::operation::associate_certificate::_associate_certificate_input::AssociateCertificateInputBuilder;

/// Fluent builder constructing a request to `AssociateCertificate`.
///
/// Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateCertificateFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_certificate::builders::AssociateCertificateInputBuilder,
}
impl AssociateCertificateFluentBuilder {
    /// Creates a new `AssociateCertificate`.
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
            crate::operation::associate_certificate::AssociateCertificate,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_certificate::AssociateCertificateError,
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
        crate::operation::associate_certificate::AssociateCertificateOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_certificate::AssociateCertificateError,
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
    /// The ARN of the ACM certificate that you want to associate with your MediaConvert resource.
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// The ARN of the ACM certificate that you want to associate with your MediaConvert resource.
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
}
