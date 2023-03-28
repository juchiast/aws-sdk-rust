// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of the <code>ResolveCustomer</code> operation. Contains the <code>CustomerIdentifier</code> along with the <code>CustomerAWSAccountId</code> and <code>ProductCode</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResolveCustomerOutput {
    /// <p>The <code>CustomerIdentifier</code> is used to identify an individual customer in your application. Calls to <code>BatchMeterUsage</code> require <code>CustomerIdentifiers</code> for each <code>UsageRecord</code>.</p>
    #[doc(hidden)]
    pub customer_identifier: std::option::Option<std::string::String>,
    /// <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent <code>BatchMeterUsage</code> calls should be made using this product code.</p>
    #[doc(hidden)]
    pub product_code: std::option::Option<std::string::String>,
    /// <p>The <code>CustomerAWSAccountId</code> provides the AWS account ID associated with the <code>CustomerIdentifier</code> for the individual customer.</p>
    #[doc(hidden)]
    pub customer_aws_account_id: std::option::Option<std::string::String>,
}
impl ResolveCustomerOutput {
    /// <p>The <code>CustomerIdentifier</code> is used to identify an individual customer in your application. Calls to <code>BatchMeterUsage</code> require <code>CustomerIdentifiers</code> for each <code>UsageRecord</code>.</p>
    pub fn customer_identifier(&self) -> std::option::Option<&str> {
        self.customer_identifier.as_deref()
    }
    /// <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent <code>BatchMeterUsage</code> calls should be made using this product code.</p>
    pub fn product_code(&self) -> std::option::Option<&str> {
        self.product_code.as_deref()
    }
    /// <p>The <code>CustomerAWSAccountId</code> provides the AWS account ID associated with the <code>CustomerIdentifier</code> for the individual customer.</p>
    pub fn customer_aws_account_id(&self) -> std::option::Option<&str> {
        self.customer_aws_account_id.as_deref()
    }
}
/// See [`ResolveCustomerOutput`](crate::output::ResolveCustomerOutput).
pub mod resolve_customer_output {

    /// A builder for [`ResolveCustomerOutput`](crate::output::ResolveCustomerOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) customer_identifier: std::option::Option<std::string::String>,
        pub(crate) product_code: std::option::Option<std::string::String>,
        pub(crate) customer_aws_account_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The <code>CustomerIdentifier</code> is used to identify an individual customer in your application. Calls to <code>BatchMeterUsage</code> require <code>CustomerIdentifiers</code> for each <code>UsageRecord</code>.</p>
        pub fn customer_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_identifier = Some(input.into());
            self
        }
        /// <p>The <code>CustomerIdentifier</code> is used to identify an individual customer in your application. Calls to <code>BatchMeterUsage</code> require <code>CustomerIdentifiers</code> for each <code>UsageRecord</code>.</p>
        pub fn set_customer_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.customer_identifier = input;
            self
        }
        /// <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent <code>BatchMeterUsage</code> calls should be made using this product code.</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.product_code = Some(input.into());
            self
        }
        /// <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent <code>BatchMeterUsage</code> calls should be made using this product code.</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.product_code = input;
            self
        }
        /// <p>The <code>CustomerAWSAccountId</code> provides the AWS account ID associated with the <code>CustomerIdentifier</code> for the individual customer.</p>
        pub fn customer_aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_aws_account_id = Some(input.into());
            self
        }
        /// <p>The <code>CustomerAWSAccountId</code> provides the AWS account ID associated with the <code>CustomerIdentifier</code> for the individual customer.</p>
        pub fn set_customer_aws_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.customer_aws_account_id = input;
            self
        }
        /// Consumes the builder and constructs a [`ResolveCustomerOutput`](crate::output::ResolveCustomerOutput).
        pub fn build(self) -> crate::output::ResolveCustomerOutput {
            crate::output::ResolveCustomerOutput {
                customer_identifier: self.customer_identifier,
                product_code: self.product_code,
                customer_aws_account_id: self.customer_aws_account_id,
            }
        }
    }
}
impl ResolveCustomerOutput {
    /// Creates a new builder-style object to manufacture [`ResolveCustomerOutput`](crate::output::ResolveCustomerOutput).
    pub fn builder() -> crate::output::resolve_customer_output::Builder {
        crate::output::resolve_customer_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct RegisterUsageOutput {
    /// <p>(Optional) Only included when public key version has expired</p>
    #[doc(hidden)]
    pub public_key_rotation_timestamp: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>JWT Token</p>
    #[doc(hidden)]
    pub signature: std::option::Option<std::string::String>,
}
impl RegisterUsageOutput {
    /// <p>(Optional) Only included when public key version has expired</p>
    pub fn public_key_rotation_timestamp(
        &self,
    ) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.public_key_rotation_timestamp.as_ref()
    }
    /// <p>JWT Token</p>
    pub fn signature(&self) -> std::option::Option<&str> {
        self.signature.as_deref()
    }
}
/// See [`RegisterUsageOutput`](crate::output::RegisterUsageOutput).
pub mod register_usage_output {

    /// A builder for [`RegisterUsageOutput`](crate::output::RegisterUsageOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) public_key_rotation_timestamp: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) signature: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>(Optional) Only included when public key version has expired</p>
        pub fn public_key_rotation_timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.public_key_rotation_timestamp = Some(input);
            self
        }
        /// <p>(Optional) Only included when public key version has expired</p>
        pub fn set_public_key_rotation_timestamp(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.public_key_rotation_timestamp = input;
            self
        }
        /// <p>JWT Token</p>
        pub fn signature(mut self, input: impl Into<std::string::String>) -> Self {
            self.signature = Some(input.into());
            self
        }
        /// <p>JWT Token</p>
        pub fn set_signature(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.signature = input;
            self
        }
        /// Consumes the builder and constructs a [`RegisterUsageOutput`](crate::output::RegisterUsageOutput).
        pub fn build(self) -> crate::output::RegisterUsageOutput {
            crate::output::RegisterUsageOutput {
                public_key_rotation_timestamp: self.public_key_rotation_timestamp,
                signature: self.signature,
            }
        }
    }
}
impl RegisterUsageOutput {
    /// Creates a new builder-style object to manufacture [`RegisterUsageOutput`](crate::output::RegisterUsageOutput).
    pub fn builder() -> crate::output::register_usage_output::Builder {
        crate::output::register_usage_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct MeterUsageOutput {
    /// <p>Metering record id.</p>
    #[doc(hidden)]
    pub metering_record_id: std::option::Option<std::string::String>,
}
impl MeterUsageOutput {
    /// <p>Metering record id.</p>
    pub fn metering_record_id(&self) -> std::option::Option<&str> {
        self.metering_record_id.as_deref()
    }
}
/// See [`MeterUsageOutput`](crate::output::MeterUsageOutput).
pub mod meter_usage_output {

    /// A builder for [`MeterUsageOutput`](crate::output::MeterUsageOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) metering_record_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Metering record id.</p>
        pub fn metering_record_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.metering_record_id = Some(input.into());
            self
        }
        /// <p>Metering record id.</p>
        pub fn set_metering_record_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.metering_record_id = input;
            self
        }
        /// Consumes the builder and constructs a [`MeterUsageOutput`](crate::output::MeterUsageOutput).
        pub fn build(self) -> crate::output::MeterUsageOutput {
            crate::output::MeterUsageOutput {
                metering_record_id: self.metering_record_id,
            }
        }
    }
}
impl MeterUsageOutput {
    /// Creates a new builder-style object to manufacture [`MeterUsageOutput`](crate::output::MeterUsageOutput).
    pub fn builder() -> crate::output::meter_usage_output::Builder {
        crate::output::meter_usage_output::Builder::default()
    }
}

/// <p>Contains the <code>UsageRecords</code> processed by <code>BatchMeterUsage</code> and any records that have failed due to transient error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchMeterUsageOutput {
    /// <p>Contains all <code>UsageRecords</code> processed by <code>BatchMeterUsage</code>. These records were either honored by AWS Marketplace Metering Service or were invalid. Invalid records should be fixed before being resubmitted.</p>
    #[doc(hidden)]
    pub results: std::option::Option<std::vec::Vec<crate::model::UsageRecordResult>>,
    /// <p>Contains all <code>UsageRecords</code> that were not processed by <code>BatchMeterUsage</code>. This is a list of <code>UsageRecords</code>. You can retry the failed request by making another <code>BatchMeterUsage</code> call with this list as input in the <code>BatchMeterUsageRequest</code>.</p>
    #[doc(hidden)]
    pub unprocessed_records: std::option::Option<std::vec::Vec<crate::model::UsageRecord>>,
}
impl BatchMeterUsageOutput {
    /// <p>Contains all <code>UsageRecords</code> processed by <code>BatchMeterUsage</code>. These records were either honored by AWS Marketplace Metering Service or were invalid. Invalid records should be fixed before being resubmitted.</p>
    pub fn results(&self) -> std::option::Option<&[crate::model::UsageRecordResult]> {
        self.results.as_deref()
    }
    /// <p>Contains all <code>UsageRecords</code> that were not processed by <code>BatchMeterUsage</code>. This is a list of <code>UsageRecords</code>. You can retry the failed request by making another <code>BatchMeterUsage</code> call with this list as input in the <code>BatchMeterUsageRequest</code>.</p>
    pub fn unprocessed_records(&self) -> std::option::Option<&[crate::model::UsageRecord]> {
        self.unprocessed_records.as_deref()
    }
}
/// See [`BatchMeterUsageOutput`](crate::output::BatchMeterUsageOutput).
pub mod batch_meter_usage_output {

    /// A builder for [`BatchMeterUsageOutput`](crate::output::BatchMeterUsageOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) results: std::option::Option<std::vec::Vec<crate::model::UsageRecordResult>>,
        pub(crate) unprocessed_records:
            std::option::Option<std::vec::Vec<crate::model::UsageRecord>>,
    }
    impl Builder {
        /// Appends an item to `results`.
        ///
        /// To override the contents of this collection use [`set_results`](Self::set_results).
        ///
        /// <p>Contains all <code>UsageRecords</code> processed by <code>BatchMeterUsage</code>. These records were either honored by AWS Marketplace Metering Service or were invalid. Invalid records should be fixed before being resubmitted.</p>
        pub fn results(mut self, input: crate::model::UsageRecordResult) -> Self {
            let mut v = self.results.unwrap_or_default();
            v.push(input);
            self.results = Some(v);
            self
        }
        /// <p>Contains all <code>UsageRecords</code> processed by <code>BatchMeterUsage</code>. These records were either honored by AWS Marketplace Metering Service or were invalid. Invalid records should be fixed before being resubmitted.</p>
        pub fn set_results(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UsageRecordResult>>,
        ) -> Self {
            self.results = input;
            self
        }
        /// Appends an item to `unprocessed_records`.
        ///
        /// To override the contents of this collection use [`set_unprocessed_records`](Self::set_unprocessed_records).
        ///
        /// <p>Contains all <code>UsageRecords</code> that were not processed by <code>BatchMeterUsage</code>. This is a list of <code>UsageRecords</code>. You can retry the failed request by making another <code>BatchMeterUsage</code> call with this list as input in the <code>BatchMeterUsageRequest</code>.</p>
        pub fn unprocessed_records(mut self, input: crate::model::UsageRecord) -> Self {
            let mut v = self.unprocessed_records.unwrap_or_default();
            v.push(input);
            self.unprocessed_records = Some(v);
            self
        }
        /// <p>Contains all <code>UsageRecords</code> that were not processed by <code>BatchMeterUsage</code>. This is a list of <code>UsageRecords</code>. You can retry the failed request by making another <code>BatchMeterUsage</code> call with this list as input in the <code>BatchMeterUsageRequest</code>.</p>
        pub fn set_unprocessed_records(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UsageRecord>>,
        ) -> Self {
            self.unprocessed_records = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchMeterUsageOutput`](crate::output::BatchMeterUsageOutput).
        pub fn build(self) -> crate::output::BatchMeterUsageOutput {
            crate::output::BatchMeterUsageOutput {
                results: self.results,
                unprocessed_records: self.unprocessed_records,
            }
        }
    }
}
impl BatchMeterUsageOutput {
    /// Creates a new builder-style object to manufacture [`BatchMeterUsageOutput`](crate::output::BatchMeterUsageOutput).
    pub fn builder() -> crate::output::batch_meter_usage_output::Builder {
        crate::output::batch_meter_usage_output::Builder::default()
    }
}
