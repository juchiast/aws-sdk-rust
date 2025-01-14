// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_associate_user_stack::_batch_associate_user_stack_output::BatchAssociateUserStackOutputBuilder;

pub use crate::operation::batch_associate_user_stack::_batch_associate_user_stack_input::BatchAssociateUserStackInputBuilder;

/// Fluent builder constructing a request to `BatchAssociateUserStack`.
///
/// <p>Associates the specified users with the specified stacks. Users in a user pool cannot be assigned to stacks with fleets that are joined to an Active Directory domain.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchAssociateUserStackFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::batch_associate_user_stack::builders::BatchAssociateUserStackInputBuilder,
}
impl BatchAssociateUserStackFluentBuilder {
    /// Creates a new `BatchAssociateUserStack`.
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
            crate::operation::batch_associate_user_stack::BatchAssociateUserStack,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_associate_user_stack::BatchAssociateUserStackError,
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
        crate::operation::batch_associate_user_stack::BatchAssociateUserStackOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_associate_user_stack::BatchAssociateUserStackError,
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
    /// Appends an item to `UserStackAssociations`.
    ///
    /// To override the contents of this collection use [`set_user_stack_associations`](Self::set_user_stack_associations).
    ///
    /// <p>The list of UserStackAssociation objects.</p>
    pub fn user_stack_associations(mut self, input: crate::types::UserStackAssociation) -> Self {
        self.inner = self.inner.user_stack_associations(input);
        self
    }
    /// <p>The list of UserStackAssociation objects.</p>
    pub fn set_user_stack_associations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::UserStackAssociation>>,
    ) -> Self {
        self.inner = self.inner.set_user_stack_associations(input);
        self
    }
}
