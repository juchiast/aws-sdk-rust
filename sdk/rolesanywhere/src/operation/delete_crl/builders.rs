// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_crl::_delete_crl_output::DeleteCrlOutputBuilder;

pub use crate::operation::delete_crl::_delete_crl_input::DeleteCrlInputBuilder;

/// Fluent builder constructing a request to `DeleteCrl`.
///
/// <p>Deletes a certificate revocation list (CRL).</p>
/// <p> <b>Required permissions: </b> <code>rolesanywhere:DeleteCrl</code>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteCrlFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_crl::builders::DeleteCrlInputBuilder,
}
impl DeleteCrlFluentBuilder {
    /// Creates a new `DeleteCrl`.
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
            crate::operation::delete_crl::DeleteCrl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_crl::DeleteCrlError>,
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
        crate::operation::delete_crl::DeleteCrlOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_crl::DeleteCrlError>,
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
    /// <p>The unique identifier of the certificate revocation list (CRL).</p>
    pub fn crl_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.crl_id(input.into());
        self
    }
    /// <p>The unique identifier of the certificate revocation list (CRL).</p>
    pub fn set_crl_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_crl_id(input);
        self
    }
}
