// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) mod shape_delete_connection;

pub(crate) mod shape_get_connection;

pub(crate) mod shape_post_to_connection;

pub(crate) mod shape_post_to_connection_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_gone_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_payload_too_large_exception;

pub(crate) mod shape_identity;
