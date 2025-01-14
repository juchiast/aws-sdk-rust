// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_signaling_channel_input(
    input: &crate::operation::delete_signaling_channel::DeleteSignalingChannelInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_signaling_channel_input::ser_delete_signaling_channel_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_signaling_channel_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_signaling_channel::DeleteSignalingChannelOutput,
    crate::operation::delete_signaling_channel::DeleteSignalingChannelError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ClientLimitExceededException" => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::ClientLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClientLimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_limit_exceeded_exception::de_client_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgumentException" => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceInUseException" => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceInUseExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "VersionMismatchException" => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::VersionMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::VersionMismatchExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_version_mismatch_exception::de_version_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_signaling_channel::DeleteSignalingChannelError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_signaling_channel::DeleteSignalingChannelError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_signaling_channel_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_signaling_channel::DeleteSignalingChannelOutput,
    crate::operation::delete_signaling_channel::DeleteSignalingChannelError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_signaling_channel::builders::DeleteSignalingChannelOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
