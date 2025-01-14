// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about when an on-call shift begins and ends.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CoverageTime {
    /// <p>Information about when the on-call rotation shift begins.</p>
    #[doc(hidden)]
    pub start: std::option::Option<crate::types::HandOffTime>,
    /// <p>Information about when the on-call rotation shift ends.</p>
    #[doc(hidden)]
    pub end: std::option::Option<crate::types::HandOffTime>,
}
impl CoverageTime {
    /// <p>Information about when the on-call rotation shift begins.</p>
    pub fn start(&self) -> std::option::Option<&crate::types::HandOffTime> {
        self.start.as_ref()
    }
    /// <p>Information about when the on-call rotation shift ends.</p>
    pub fn end(&self) -> std::option::Option<&crate::types::HandOffTime> {
        self.end.as_ref()
    }
}
impl CoverageTime {
    /// Creates a new builder-style object to manufacture [`CoverageTime`](crate::types::CoverageTime).
    pub fn builder() -> crate::types::builders::CoverageTimeBuilder {
        crate::types::builders::CoverageTimeBuilder::default()
    }
}

/// A builder for [`CoverageTime`](crate::types::CoverageTime).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct CoverageTimeBuilder {
    pub(crate) start: std::option::Option<crate::types::HandOffTime>,
    pub(crate) end: std::option::Option<crate::types::HandOffTime>,
}
impl CoverageTimeBuilder {
    /// <p>Information about when the on-call rotation shift begins.</p>
    pub fn start(mut self, input: crate::types::HandOffTime) -> Self {
        self.start = Some(input);
        self
    }
    /// <p>Information about when the on-call rotation shift begins.</p>
    pub fn set_start(mut self, input: std::option::Option<crate::types::HandOffTime>) -> Self {
        self.start = input;
        self
    }
    /// <p>Information about when the on-call rotation shift ends.</p>
    pub fn end(mut self, input: crate::types::HandOffTime) -> Self {
        self.end = Some(input);
        self
    }
    /// <p>Information about when the on-call rotation shift ends.</p>
    pub fn set_end(mut self, input: std::option::Option<crate::types::HandOffTime>) -> Self {
        self.end = input;
        self
    }
    /// Consumes the builder and constructs a [`CoverageTime`](crate::types::CoverageTime).
    pub fn build(self) -> crate::types::CoverageTime {
        crate::types::CoverageTime {
            start: self.start,
            end: self.end,
        }
    }
}
