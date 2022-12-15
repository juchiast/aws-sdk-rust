// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_synthesize_speech_synthesize_speech_output_audio_stream(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<
    aws_smithy_http::byte_stream::ByteStream,
    crate::error::SynthesizeSpeechError,
> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub(crate) fn deser_header_synthesize_speech_synthesize_speech_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_synthesize_speech_synthesize_speech_output_request_characters(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amzn-RequestCharacters").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}
