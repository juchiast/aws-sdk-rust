// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_table_objects::_update_table_objects_output::UpdateTableObjectsOutputBuilder;

pub use crate::operation::update_table_objects::_update_table_objects_input::UpdateTableObjectsInputBuilder;

/// Fluent builder constructing a request to `UpdateTableObjects`.
///
/// <p>Updates the manifest of Amazon S3 objects that make up the specified governed table.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTableObjectsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_table_objects::builders::UpdateTableObjectsInputBuilder,
}
impl UpdateTableObjectsFluentBuilder {
    /// Creates a new `UpdateTableObjects`.
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
            crate::operation::update_table_objects::UpdateTableObjects,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_table_objects::UpdateTableObjectsError,
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
        crate::operation::update_table_objects::UpdateTableObjectsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_table_objects::UpdateTableObjectsError,
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
    /// <p>The catalog containing the governed table to update. Defaults to the caller’s account ID.</p>
    pub fn catalog_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The catalog containing the governed table to update. Defaults to the caller’s account ID.</p>
    pub fn set_catalog_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The database containing the governed table to update.</p>
    pub fn database_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The database containing the governed table to update.</p>
    pub fn set_database_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The governed table to update.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The governed table to update.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The transaction at which to do the write.</p>
    pub fn transaction_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transaction_id(input.into());
        self
    }
    /// <p>The transaction at which to do the write.</p>
    pub fn set_transaction_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_transaction_id(input);
        self
    }
    /// Appends an item to `WriteOperations`.
    ///
    /// To override the contents of this collection use [`set_write_operations`](Self::set_write_operations).
    ///
    /// <p>A list of <code>WriteOperation</code> objects that define an object to add to or delete from the manifest for a governed table.</p>
    pub fn write_operations(mut self, input: crate::types::WriteOperation) -> Self {
        self.inner = self.inner.write_operations(input);
        self
    }
    /// <p>A list of <code>WriteOperation</code> objects that define an object to add to or delete from the manifest for a governed table.</p>
    pub fn set_write_operations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::WriteOperation>>,
    ) -> Self {
        self.inner = self.inner.set_write_operations(input);
        self
    }
}
