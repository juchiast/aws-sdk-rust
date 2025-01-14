// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tag_resource_input(
    input: &crate::operation::tag_resource::TagResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_tag_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::tag_resource::TagResourceOutput,
    crate::operation::tag_resource::TagResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::tag_resource::TagResourceError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ArgumentException" => {
            crate::operation::tag_resource::TagResourceError::ArgumentException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ArgumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_argument_exception::de_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => {
            crate::operation::tag_resource::TagResourceError::NotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TagOperationException" => {
            crate::operation::tag_resource::TagResourceError::TagOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TagOperationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_operation_exception::de_tag_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TagPolicyException" => {
            crate::operation::tag_resource::TagResourceError::TagPolicyException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TagPolicyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_policy_exception::de_tag_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyTagsException" => {
            crate::operation::tag_resource::TagResourceError::TooManyTagsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TooManyTagsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_tags_exception::de_too_many_tags_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::tag_resource::TagResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_tag_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::tag_resource::TagResourceOutput,
    crate::operation::tag_resource::TagResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::tag_resource::builders::TagResourceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
