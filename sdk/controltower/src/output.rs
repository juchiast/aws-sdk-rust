// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListEnabledControlsOutput {
    /// <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains.</p>
    #[doc(hidden)]
    pub enabled_controls: std::option::Option<std::vec::Vec<crate::model::EnabledControlSummary>>,
    /// <p>Retrieves the next page of results. If the string is empty, the current response is the end of the results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListEnabledControlsOutput {
    /// <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains.</p>
    pub fn enabled_controls(&self) -> std::option::Option<&[crate::model::EnabledControlSummary]> {
        self.enabled_controls.as_deref()
    }
    /// <p>Retrieves the next page of results. If the string is empty, the current response is the end of the results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListEnabledControlsOutput`](crate::output::ListEnabledControlsOutput).
pub mod list_enabled_controls_output {

    /// A builder for [`ListEnabledControlsOutput`](crate::output::ListEnabledControlsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) enabled_controls:
            std::option::Option<std::vec::Vec<crate::model::EnabledControlSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `enabled_controls`.
        ///
        /// To override the contents of this collection use [`set_enabled_controls`](Self::set_enabled_controls).
        ///
        /// <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains.</p>
        pub fn enabled_controls(mut self, input: crate::model::EnabledControlSummary) -> Self {
            let mut v = self.enabled_controls.unwrap_or_default();
            v.push(input);
            self.enabled_controls = Some(v);
            self
        }
        /// <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains.</p>
        pub fn set_enabled_controls(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EnabledControlSummary>>,
        ) -> Self {
            self.enabled_controls = input;
            self
        }
        /// <p>Retrieves the next page of results. If the string is empty, the current response is the end of the results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>Retrieves the next page of results. If the string is empty, the current response is the end of the results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEnabledControlsOutput`](crate::output::ListEnabledControlsOutput).
        pub fn build(self) -> crate::output::ListEnabledControlsOutput {
            crate::output::ListEnabledControlsOutput {
                enabled_controls: self.enabled_controls,
                next_token: self.next_token,
            }
        }
    }
}
impl ListEnabledControlsOutput {
    /// Creates a new builder-style object to manufacture [`ListEnabledControlsOutput`](crate::output::ListEnabledControlsOutput).
    pub fn builder() -> crate::output::list_enabled_controls_output::Builder {
        crate::output::list_enabled_controls_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetControlOperationOutput {
    /// <p></p>
    #[doc(hidden)]
    pub control_operation: std::option::Option<crate::model::ControlOperation>,
}
impl GetControlOperationOutput {
    /// <p></p>
    pub fn control_operation(&self) -> std::option::Option<&crate::model::ControlOperation> {
        self.control_operation.as_ref()
    }
}
/// See [`GetControlOperationOutput`](crate::output::GetControlOperationOutput).
pub mod get_control_operation_output {

    /// A builder for [`GetControlOperationOutput`](crate::output::GetControlOperationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_operation: std::option::Option<crate::model::ControlOperation>,
    }
    impl Builder {
        /// <p></p>
        pub fn control_operation(mut self, input: crate::model::ControlOperation) -> Self {
            self.control_operation = Some(input);
            self
        }
        /// <p></p>
        pub fn set_control_operation(
            mut self,
            input: std::option::Option<crate::model::ControlOperation>,
        ) -> Self {
            self.control_operation = input;
            self
        }
        /// Consumes the builder and constructs a [`GetControlOperationOutput`](crate::output::GetControlOperationOutput).
        pub fn build(self) -> crate::output::GetControlOperationOutput {
            crate::output::GetControlOperationOutput {
                control_operation: self.control_operation,
            }
        }
    }
}
impl GetControlOperationOutput {
    /// Creates a new builder-style object to manufacture [`GetControlOperationOutput`](crate::output::GetControlOperationOutput).
    pub fn builder() -> crate::output::get_control_operation_output::Builder {
        crate::output::get_control_operation_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct EnableControlOutput {
    /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    #[doc(hidden)]
    pub operation_identifier: std::option::Option<std::string::String>,
}
impl EnableControlOutput {
    /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    pub fn operation_identifier(&self) -> std::option::Option<&str> {
        self.operation_identifier.as_deref()
    }
}
/// See [`EnableControlOutput`](crate::output::EnableControlOutput).
pub mod enable_control_output {

    /// A builder for [`EnableControlOutput`](crate::output::EnableControlOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) operation_identifier: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn operation_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.operation_identifier = Some(input.into());
            self
        }
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn set_operation_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.operation_identifier = input;
            self
        }
        /// Consumes the builder and constructs a [`EnableControlOutput`](crate::output::EnableControlOutput).
        pub fn build(self) -> crate::output::EnableControlOutput {
            crate::output::EnableControlOutput {
                operation_identifier: self.operation_identifier,
            }
        }
    }
}
impl EnableControlOutput {
    /// Creates a new builder-style object to manufacture [`EnableControlOutput`](crate::output::EnableControlOutput).
    pub fn builder() -> crate::output::enable_control_output::Builder {
        crate::output::enable_control_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DisableControlOutput {
    /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    #[doc(hidden)]
    pub operation_identifier: std::option::Option<std::string::String>,
}
impl DisableControlOutput {
    /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    pub fn operation_identifier(&self) -> std::option::Option<&str> {
        self.operation_identifier.as_deref()
    }
}
/// See [`DisableControlOutput`](crate::output::DisableControlOutput).
pub mod disable_control_output {

    /// A builder for [`DisableControlOutput`](crate::output::DisableControlOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) operation_identifier: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn operation_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.operation_identifier = Some(input.into());
            self
        }
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn set_operation_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.operation_identifier = input;
            self
        }
        /// Consumes the builder and constructs a [`DisableControlOutput`](crate::output::DisableControlOutput).
        pub fn build(self) -> crate::output::DisableControlOutput {
            crate::output::DisableControlOutput {
                operation_identifier: self.operation_identifier,
            }
        }
    }
}
impl DisableControlOutput {
    /// Creates a new builder-style object to manufacture [`DisableControlOutput`](crate::output::DisableControlOutput).
    pub fn builder() -> crate::output::disable_control_output::Builder {
        crate::output::disable_control_output::Builder::default()
    }
}
