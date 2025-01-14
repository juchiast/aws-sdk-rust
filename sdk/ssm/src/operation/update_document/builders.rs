// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_document::_update_document_output::UpdateDocumentOutputBuilder;

pub use crate::operation::update_document::_update_document_input::UpdateDocumentInputBuilder;

/// Fluent builder constructing a request to `UpdateDocument`.
///
/// <p>Updates one or more values for an SSM document.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDocumentFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_document::builders::UpdateDocumentInputBuilder,
}
impl UpdateDocumentFluentBuilder {
    /// Creates a new `UpdateDocument`.
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
            crate::operation::update_document::UpdateDocument,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_document::UpdateDocumentError>,
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
        crate::operation::update_document::UpdateDocumentOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_document::UpdateDocumentError>,
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
    /// <p>A valid JSON or YAML string.</p>
    pub fn content(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content(input.into());
        self
    }
    /// <p>A valid JSON or YAML string.</p>
    pub fn set_content(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
    /// Appends an item to `Attachments`.
    ///
    /// To override the contents of this collection use [`set_attachments`](Self::set_attachments).
    ///
    /// <p>A list of key-value pairs that describe attachments to a version of a document.</p>
    pub fn attachments(mut self, input: crate::types::AttachmentsSource) -> Self {
        self.inner = self.inner.attachments(input);
        self
    }
    /// <p>A list of key-value pairs that describe attachments to a version of a document.</p>
    pub fn set_attachments(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttachmentsSource>>,
    ) -> Self {
        self.inner = self.inner.set_attachments(input);
        self
    }
    /// <p>The name of the SSM document that you want to update.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the SSM document that you want to update.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The friendly name of the SSM document that you want to update. This value can differ for each version of the document. If you don't specify a value for this parameter in your request, the existing value is applied to the new document version.</p>
    pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p>The friendly name of the SSM document that you want to update. This value can differ for each version of the document. If you don't specify a value for this parameter in your request, the existing value is applied to the new document version.</p>
    pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p>An optional field specifying the version of the artifact you are updating with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and can't be changed.</p>
    pub fn version_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.version_name(input.into());
        self
    }
    /// <p>An optional field specifying the version of the artifact you are updating with the document. For example, "Release 12, Update 6". This value is unique across all versions of a document, and can't be changed.</p>
    pub fn set_version_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_version_name(input);
        self
    }
    /// <p>The version of the document that you want to update. Currently, Systems Manager supports updating only the latest version of the document. You can specify the version number of the latest version or use the <code>$LATEST</code> variable.</p> <note>
    /// <p>If you change a document version for a State Manager association, Systems Manager immediately runs the association unless you previously specifed the <code>apply-only-at-cron-interval</code> parameter.</p>
    /// </note>
    pub fn document_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.document_version(input.into());
        self
    }
    /// <p>The version of the document that you want to update. Currently, Systems Manager supports updating only the latest version of the document. You can specify the version number of the latest version or use the <code>$LATEST</code> variable.</p> <note>
    /// <p>If you change a document version for a State Manager association, Systems Manager immediately runs the association unless you previously specifed the <code>apply-only-at-cron-interval</code> parameter.</p>
    /// </note>
    pub fn set_document_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_document_version(input);
        self
    }
    /// <p>Specify the document format for the new document version. Systems Manager supports JSON and YAML documents. JSON is the default format.</p>
    pub fn document_format(mut self, input: crate::types::DocumentFormat) -> Self {
        self.inner = self.inner.document_format(input);
        self
    }
    /// <p>Specify the document format for the new document version. Systems Manager supports JSON and YAML documents. JSON is the default format.</p>
    pub fn set_document_format(
        mut self,
        input: std::option::Option<crate::types::DocumentFormat>,
    ) -> Self {
        self.inner = self.inner.set_document_format(input);
        self
    }
    /// <p>Specify a new target type for the document.</p>
    pub fn target_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_type(input.into());
        self
    }
    /// <p>Specify a new target type for the document.</p>
    pub fn set_target_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_type(input);
        self
    }
}
