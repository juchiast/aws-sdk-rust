// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SendCommandOutput {
    /// <p>Contains the details of the started session that includes a session token. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[doc(hidden)]
    pub start_session: std::option::Option<crate::model::StartSessionResult>,
    /// <p>Contains the details of the started transaction.</p>
    #[doc(hidden)]
    pub start_transaction: std::option::Option<crate::model::StartTransactionResult>,
    /// <p>Contains the details of the ended session.</p>
    #[doc(hidden)]
    pub end_session: std::option::Option<crate::model::EndSessionResult>,
    /// <p>Contains the details of the committed transaction.</p>
    #[doc(hidden)]
    pub commit_transaction: std::option::Option<crate::model::CommitTransactionResult>,
    /// <p>Contains the details of the aborted transaction.</p>
    #[doc(hidden)]
    pub abort_transaction: std::option::Option<crate::model::AbortTransactionResult>,
    /// <p>Contains the details of the executed statement.</p>
    #[doc(hidden)]
    pub execute_statement: std::option::Option<crate::model::ExecuteStatementResult>,
    /// <p>Contains the details of the fetched page.</p>
    #[doc(hidden)]
    pub fetch_page: std::option::Option<crate::model::FetchPageResult>,
}
impl SendCommandOutput {
    /// <p>Contains the details of the started session that includes a session token. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    pub fn start_session(&self) -> std::option::Option<&crate::model::StartSessionResult> {
        self.start_session.as_ref()
    }
    /// <p>Contains the details of the started transaction.</p>
    pub fn start_transaction(&self) -> std::option::Option<&crate::model::StartTransactionResult> {
        self.start_transaction.as_ref()
    }
    /// <p>Contains the details of the ended session.</p>
    pub fn end_session(&self) -> std::option::Option<&crate::model::EndSessionResult> {
        self.end_session.as_ref()
    }
    /// <p>Contains the details of the committed transaction.</p>
    pub fn commit_transaction(
        &self,
    ) -> std::option::Option<&crate::model::CommitTransactionResult> {
        self.commit_transaction.as_ref()
    }
    /// <p>Contains the details of the aborted transaction.</p>
    pub fn abort_transaction(&self) -> std::option::Option<&crate::model::AbortTransactionResult> {
        self.abort_transaction.as_ref()
    }
    /// <p>Contains the details of the executed statement.</p>
    pub fn execute_statement(&self) -> std::option::Option<&crate::model::ExecuteStatementResult> {
        self.execute_statement.as_ref()
    }
    /// <p>Contains the details of the fetched page.</p>
    pub fn fetch_page(&self) -> std::option::Option<&crate::model::FetchPageResult> {
        self.fetch_page.as_ref()
    }
}
/// See [`SendCommandOutput`](crate::output::SendCommandOutput).
pub mod send_command_output {

    /// A builder for [`SendCommandOutput`](crate::output::SendCommandOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) start_session: std::option::Option<crate::model::StartSessionResult>,
        pub(crate) start_transaction: std::option::Option<crate::model::StartTransactionResult>,
        pub(crate) end_session: std::option::Option<crate::model::EndSessionResult>,
        pub(crate) commit_transaction: std::option::Option<crate::model::CommitTransactionResult>,
        pub(crate) abort_transaction: std::option::Option<crate::model::AbortTransactionResult>,
        pub(crate) execute_statement: std::option::Option<crate::model::ExecuteStatementResult>,
        pub(crate) fetch_page: std::option::Option<crate::model::FetchPageResult>,
    }
    impl Builder {
        /// <p>Contains the details of the started session that includes a session token. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
        pub fn start_session(mut self, input: crate::model::StartSessionResult) -> Self {
            self.start_session = Some(input);
            self
        }
        /// <p>Contains the details of the started session that includes a session token. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
        pub fn set_start_session(
            mut self,
            input: std::option::Option<crate::model::StartSessionResult>,
        ) -> Self {
            self.start_session = input;
            self
        }
        /// <p>Contains the details of the started transaction.</p>
        pub fn start_transaction(mut self, input: crate::model::StartTransactionResult) -> Self {
            self.start_transaction = Some(input);
            self
        }
        /// <p>Contains the details of the started transaction.</p>
        pub fn set_start_transaction(
            mut self,
            input: std::option::Option<crate::model::StartTransactionResult>,
        ) -> Self {
            self.start_transaction = input;
            self
        }
        /// <p>Contains the details of the ended session.</p>
        pub fn end_session(mut self, input: crate::model::EndSessionResult) -> Self {
            self.end_session = Some(input);
            self
        }
        /// <p>Contains the details of the ended session.</p>
        pub fn set_end_session(
            mut self,
            input: std::option::Option<crate::model::EndSessionResult>,
        ) -> Self {
            self.end_session = input;
            self
        }
        /// <p>Contains the details of the committed transaction.</p>
        pub fn commit_transaction(mut self, input: crate::model::CommitTransactionResult) -> Self {
            self.commit_transaction = Some(input);
            self
        }
        /// <p>Contains the details of the committed transaction.</p>
        pub fn set_commit_transaction(
            mut self,
            input: std::option::Option<crate::model::CommitTransactionResult>,
        ) -> Self {
            self.commit_transaction = input;
            self
        }
        /// <p>Contains the details of the aborted transaction.</p>
        pub fn abort_transaction(mut self, input: crate::model::AbortTransactionResult) -> Self {
            self.abort_transaction = Some(input);
            self
        }
        /// <p>Contains the details of the aborted transaction.</p>
        pub fn set_abort_transaction(
            mut self,
            input: std::option::Option<crate::model::AbortTransactionResult>,
        ) -> Self {
            self.abort_transaction = input;
            self
        }
        /// <p>Contains the details of the executed statement.</p>
        pub fn execute_statement(mut self, input: crate::model::ExecuteStatementResult) -> Self {
            self.execute_statement = Some(input);
            self
        }
        /// <p>Contains the details of the executed statement.</p>
        pub fn set_execute_statement(
            mut self,
            input: std::option::Option<crate::model::ExecuteStatementResult>,
        ) -> Self {
            self.execute_statement = input;
            self
        }
        /// <p>Contains the details of the fetched page.</p>
        pub fn fetch_page(mut self, input: crate::model::FetchPageResult) -> Self {
            self.fetch_page = Some(input);
            self
        }
        /// <p>Contains the details of the fetched page.</p>
        pub fn set_fetch_page(
            mut self,
            input: std::option::Option<crate::model::FetchPageResult>,
        ) -> Self {
            self.fetch_page = input;
            self
        }
        /// Consumes the builder and constructs a [`SendCommandOutput`](crate::output::SendCommandOutput).
        pub fn build(self) -> crate::output::SendCommandOutput {
            crate::output::SendCommandOutput {
                start_session: self.start_session,
                start_transaction: self.start_transaction,
                end_session: self.end_session,
                commit_transaction: self.commit_transaction,
                abort_transaction: self.abort_transaction,
                execute_statement: self.execute_statement,
                fetch_page: self.fetch_page,
            }
        }
    }
}
impl SendCommandOutput {
    /// Creates a new builder-style object to manufacture [`SendCommandOutput`](crate::output::SendCommandOutput).
    pub fn builder() -> crate::output::send_command_output::Builder {
        crate::output::send_command_output::Builder::default()
    }
}
