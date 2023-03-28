// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_get_reference(
    input: &crate::input::GetReferenceInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.range {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "range",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("Range", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_payload_get_read_set_get_read_set_output_payload(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::GetReadSetError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_payload_get_reference_get_reference_output_payload(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::GetReferenceError>
{
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}
