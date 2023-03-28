// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SynthesizeSpeechOutput {
    /// <p> Stream containing the synthesized speech. </p>
    pub audio_stream: aws_smithy_http::byte_stream::ByteStream,
    /// <p> Specifies the type audio stream. This should reflect the <code>OutputFormat</code> parameter in your request. </p>
    /// <ul>
    /// <li> <p> If you request <code>mp3</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/mpeg. </p> </li>
    /// <li> <p> If you request <code>ogg_vorbis</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/ogg. </p> </li>
    /// <li> <p> If you request <code>pcm</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/pcm in a signed 16-bit, 1 channel (mono), little-endian format. </p> </li>
    /// <li> <p>If you request <code>json</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is application/x-json-stream.</p> </li>
    /// </ul>
    /// <p> </p>
    #[doc(hidden)]
    pub content_type: std::option::Option<std::string::String>,
    /// <p>Number of characters synthesized.</p>
    #[doc(hidden)]
    pub request_characters: i32,
}
impl SynthesizeSpeechOutput {
    /// <p> Stream containing the synthesized speech. </p>
    pub fn audio_stream(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.audio_stream
    }
    /// <p> Specifies the type audio stream. This should reflect the <code>OutputFormat</code> parameter in your request. </p>
    /// <ul>
    /// <li> <p> If you request <code>mp3</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/mpeg. </p> </li>
    /// <li> <p> If you request <code>ogg_vorbis</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/ogg. </p> </li>
    /// <li> <p> If you request <code>pcm</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/pcm in a signed 16-bit, 1 channel (mono), little-endian format. </p> </li>
    /// <li> <p>If you request <code>json</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is application/x-json-stream.</p> </li>
    /// </ul>
    /// <p> </p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>Number of characters synthesized.</p>
    pub fn request_characters(&self) -> i32 {
        self.request_characters
    }
}
/// See [`SynthesizeSpeechOutput`](crate::output::SynthesizeSpeechOutput).
pub mod synthesize_speech_output {

    /// A builder for [`SynthesizeSpeechOutput`](crate::output::SynthesizeSpeechOutput).
    #[non_exhaustive]
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) audio_stream: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) request_characters: std::option::Option<i32>,
    }
    impl Builder {
        /// <p> Stream containing the synthesized speech. </p>
        pub fn audio_stream(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.audio_stream = Some(input);
            self
        }
        /// <p> Stream containing the synthesized speech. </p>
        pub fn set_audio_stream(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.audio_stream = input;
            self
        }
        /// <p> Specifies the type audio stream. This should reflect the <code>OutputFormat</code> parameter in your request. </p>
        /// <ul>
        /// <li> <p> If you request <code>mp3</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/mpeg. </p> </li>
        /// <li> <p> If you request <code>ogg_vorbis</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/ogg. </p> </li>
        /// <li> <p> If you request <code>pcm</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/pcm in a signed 16-bit, 1 channel (mono), little-endian format. </p> </li>
        /// <li> <p>If you request <code>json</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is application/x-json-stream.</p> </li>
        /// </ul>
        /// <p> </p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p> Specifies the type audio stream. This should reflect the <code>OutputFormat</code> parameter in your request. </p>
        /// <ul>
        /// <li> <p> If you request <code>mp3</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/mpeg. </p> </li>
        /// <li> <p> If you request <code>ogg_vorbis</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/ogg. </p> </li>
        /// <li> <p> If you request <code>pcm</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/pcm in a signed 16-bit, 1 channel (mono), little-endian format. </p> </li>
        /// <li> <p>If you request <code>json</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is application/x-json-stream.</p> </li>
        /// </ul>
        /// <p> </p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>Number of characters synthesized.</p>
        pub fn request_characters(mut self, input: i32) -> Self {
            self.request_characters = Some(input);
            self
        }
        /// <p>Number of characters synthesized.</p>
        pub fn set_request_characters(mut self, input: std::option::Option<i32>) -> Self {
            self.request_characters = input;
            self
        }
        /// Consumes the builder and constructs a [`SynthesizeSpeechOutput`](crate::output::SynthesizeSpeechOutput).
        pub fn build(self) -> crate::output::SynthesizeSpeechOutput {
            crate::output::SynthesizeSpeechOutput {
                audio_stream: self.audio_stream.unwrap_or_default(),
                content_type: self.content_type,
                request_characters: self.request_characters.unwrap_or_default(),
            }
        }
    }
}
impl SynthesizeSpeechOutput {
    /// Creates a new builder-style object to manufacture [`SynthesizeSpeechOutput`](crate::output::SynthesizeSpeechOutput).
    pub fn builder() -> crate::output::synthesize_speech_output::Builder {
        crate::output::synthesize_speech_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartSpeechSynthesisTaskOutput {
    /// <p>SynthesisTask object that provides information and attributes about a newly submitted speech synthesis task.</p>
    #[doc(hidden)]
    pub synthesis_task: std::option::Option<crate::model::SynthesisTask>,
}
impl StartSpeechSynthesisTaskOutput {
    /// <p>SynthesisTask object that provides information and attributes about a newly submitted speech synthesis task.</p>
    pub fn synthesis_task(&self) -> std::option::Option<&crate::model::SynthesisTask> {
        self.synthesis_task.as_ref()
    }
}
/// See [`StartSpeechSynthesisTaskOutput`](crate::output::StartSpeechSynthesisTaskOutput).
pub mod start_speech_synthesis_task_output {

    /// A builder for [`StartSpeechSynthesisTaskOutput`](crate::output::StartSpeechSynthesisTaskOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) synthesis_task: std::option::Option<crate::model::SynthesisTask>,
    }
    impl Builder {
        /// <p>SynthesisTask object that provides information and attributes about a newly submitted speech synthesis task.</p>
        pub fn synthesis_task(mut self, input: crate::model::SynthesisTask) -> Self {
            self.synthesis_task = Some(input);
            self
        }
        /// <p>SynthesisTask object that provides information and attributes about a newly submitted speech synthesis task.</p>
        pub fn set_synthesis_task(
            mut self,
            input: std::option::Option<crate::model::SynthesisTask>,
        ) -> Self {
            self.synthesis_task = input;
            self
        }
        /// Consumes the builder and constructs a [`StartSpeechSynthesisTaskOutput`](crate::output::StartSpeechSynthesisTaskOutput).
        pub fn build(self) -> crate::output::StartSpeechSynthesisTaskOutput {
            crate::output::StartSpeechSynthesisTaskOutput {
                synthesis_task: self.synthesis_task,
            }
        }
    }
}
impl StartSpeechSynthesisTaskOutput {
    /// Creates a new builder-style object to manufacture [`StartSpeechSynthesisTaskOutput`](crate::output::StartSpeechSynthesisTaskOutput).
    pub fn builder() -> crate::output::start_speech_synthesis_task_output::Builder {
        crate::output::start_speech_synthesis_task_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutLexiconOutput {}
/// See [`PutLexiconOutput`](crate::output::PutLexiconOutput).
pub mod put_lexicon_output {

    /// A builder for [`PutLexiconOutput`](crate::output::PutLexiconOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutLexiconOutput`](crate::output::PutLexiconOutput).
        pub fn build(self) -> crate::output::PutLexiconOutput {
            crate::output::PutLexiconOutput {}
        }
    }
}
impl PutLexiconOutput {
    /// Creates a new builder-style object to manufacture [`PutLexiconOutput`](crate::output::PutLexiconOutput).
    pub fn builder() -> crate::output::put_lexicon_output::Builder {
        crate::output::put_lexicon_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListSpeechSynthesisTasksOutput {
    /// <p>An opaque pagination token returned from the previous List operation in this request. If present, this indicates where to continue the listing.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>List of SynthesisTask objects that provides information from the specified task in the list request, including output format, creation time, task status, and so on.</p>
    #[doc(hidden)]
    pub synthesis_tasks: std::option::Option<std::vec::Vec<crate::model::SynthesisTask>>,
}
impl ListSpeechSynthesisTasksOutput {
    /// <p>An opaque pagination token returned from the previous List operation in this request. If present, this indicates where to continue the listing.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>List of SynthesisTask objects that provides information from the specified task in the list request, including output format, creation time, task status, and so on.</p>
    pub fn synthesis_tasks(&self) -> std::option::Option<&[crate::model::SynthesisTask]> {
        self.synthesis_tasks.as_deref()
    }
}
/// See [`ListSpeechSynthesisTasksOutput`](crate::output::ListSpeechSynthesisTasksOutput).
pub mod list_speech_synthesis_tasks_output {

    /// A builder for [`ListSpeechSynthesisTasksOutput`](crate::output::ListSpeechSynthesisTasksOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) synthesis_tasks: std::option::Option<std::vec::Vec<crate::model::SynthesisTask>>,
    }
    impl Builder {
        /// <p>An opaque pagination token returned from the previous List operation in this request. If present, this indicates where to continue the listing.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>An opaque pagination token returned from the previous List operation in this request. If present, this indicates where to continue the listing.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `synthesis_tasks`.
        ///
        /// To override the contents of this collection use [`set_synthesis_tasks`](Self::set_synthesis_tasks).
        ///
        /// <p>List of SynthesisTask objects that provides information from the specified task in the list request, including output format, creation time, task status, and so on.</p>
        pub fn synthesis_tasks(mut self, input: crate::model::SynthesisTask) -> Self {
            let mut v = self.synthesis_tasks.unwrap_or_default();
            v.push(input);
            self.synthesis_tasks = Some(v);
            self
        }
        /// <p>List of SynthesisTask objects that provides information from the specified task in the list request, including output format, creation time, task status, and so on.</p>
        pub fn set_synthesis_tasks(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SynthesisTask>>,
        ) -> Self {
            self.synthesis_tasks = input;
            self
        }
        /// Consumes the builder and constructs a [`ListSpeechSynthesisTasksOutput`](crate::output::ListSpeechSynthesisTasksOutput).
        pub fn build(self) -> crate::output::ListSpeechSynthesisTasksOutput {
            crate::output::ListSpeechSynthesisTasksOutput {
                next_token: self.next_token,
                synthesis_tasks: self.synthesis_tasks,
            }
        }
    }
}
impl ListSpeechSynthesisTasksOutput {
    /// Creates a new builder-style object to manufacture [`ListSpeechSynthesisTasksOutput`](crate::output::ListSpeechSynthesisTasksOutput).
    pub fn builder() -> crate::output::list_speech_synthesis_tasks_output::Builder {
        crate::output::list_speech_synthesis_tasks_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListLexiconsOutput {
    /// <p>A list of lexicon names and attributes.</p>
    #[doc(hidden)]
    pub lexicons: std::option::Option<std::vec::Vec<crate::model::LexiconDescription>>,
    /// <p>The pagination token to use in the next request to continue the listing of lexicons. <code>NextToken</code> is returned only if the response is truncated.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListLexiconsOutput {
    /// <p>A list of lexicon names and attributes.</p>
    pub fn lexicons(&self) -> std::option::Option<&[crate::model::LexiconDescription]> {
        self.lexicons.as_deref()
    }
    /// <p>The pagination token to use in the next request to continue the listing of lexicons. <code>NextToken</code> is returned only if the response is truncated.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListLexiconsOutput`](crate::output::ListLexiconsOutput).
pub mod list_lexicons_output {

    /// A builder for [`ListLexiconsOutput`](crate::output::ListLexiconsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) lexicons: std::option::Option<std::vec::Vec<crate::model::LexiconDescription>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `lexicons`.
        ///
        /// To override the contents of this collection use [`set_lexicons`](Self::set_lexicons).
        ///
        /// <p>A list of lexicon names and attributes.</p>
        pub fn lexicons(mut self, input: crate::model::LexiconDescription) -> Self {
            let mut v = self.lexicons.unwrap_or_default();
            v.push(input);
            self.lexicons = Some(v);
            self
        }
        /// <p>A list of lexicon names and attributes.</p>
        pub fn set_lexicons(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::LexiconDescription>>,
        ) -> Self {
            self.lexicons = input;
            self
        }
        /// <p>The pagination token to use in the next request to continue the listing of lexicons. <code>NextToken</code> is returned only if the response is truncated.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token to use in the next request to continue the listing of lexicons. <code>NextToken</code> is returned only if the response is truncated.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListLexiconsOutput`](crate::output::ListLexiconsOutput).
        pub fn build(self) -> crate::output::ListLexiconsOutput {
            crate::output::ListLexiconsOutput {
                lexicons: self.lexicons,
                next_token: self.next_token,
            }
        }
    }
}
impl ListLexiconsOutput {
    /// Creates a new builder-style object to manufacture [`ListLexiconsOutput`](crate::output::ListLexiconsOutput).
    pub fn builder() -> crate::output::list_lexicons_output::Builder {
        crate::output::list_lexicons_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetSpeechSynthesisTaskOutput {
    /// <p>SynthesisTask object that provides information from the requested task, including output format, creation time, task status, and so on.</p>
    #[doc(hidden)]
    pub synthesis_task: std::option::Option<crate::model::SynthesisTask>,
}
impl GetSpeechSynthesisTaskOutput {
    /// <p>SynthesisTask object that provides information from the requested task, including output format, creation time, task status, and so on.</p>
    pub fn synthesis_task(&self) -> std::option::Option<&crate::model::SynthesisTask> {
        self.synthesis_task.as_ref()
    }
}
/// See [`GetSpeechSynthesisTaskOutput`](crate::output::GetSpeechSynthesisTaskOutput).
pub mod get_speech_synthesis_task_output {

    /// A builder for [`GetSpeechSynthesisTaskOutput`](crate::output::GetSpeechSynthesisTaskOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) synthesis_task: std::option::Option<crate::model::SynthesisTask>,
    }
    impl Builder {
        /// <p>SynthesisTask object that provides information from the requested task, including output format, creation time, task status, and so on.</p>
        pub fn synthesis_task(mut self, input: crate::model::SynthesisTask) -> Self {
            self.synthesis_task = Some(input);
            self
        }
        /// <p>SynthesisTask object that provides information from the requested task, including output format, creation time, task status, and so on.</p>
        pub fn set_synthesis_task(
            mut self,
            input: std::option::Option<crate::model::SynthesisTask>,
        ) -> Self {
            self.synthesis_task = input;
            self
        }
        /// Consumes the builder and constructs a [`GetSpeechSynthesisTaskOutput`](crate::output::GetSpeechSynthesisTaskOutput).
        pub fn build(self) -> crate::output::GetSpeechSynthesisTaskOutput {
            crate::output::GetSpeechSynthesisTaskOutput {
                synthesis_task: self.synthesis_task,
            }
        }
    }
}
impl GetSpeechSynthesisTaskOutput {
    /// Creates a new builder-style object to manufacture [`GetSpeechSynthesisTaskOutput`](crate::output::GetSpeechSynthesisTaskOutput).
    pub fn builder() -> crate::output::get_speech_synthesis_task_output::Builder {
        crate::output::get_speech_synthesis_task_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetLexiconOutput {
    /// <p>Lexicon object that provides name and the string content of the lexicon. </p>
    #[doc(hidden)]
    pub lexicon: std::option::Option<crate::model::Lexicon>,
    /// <p>Metadata of the lexicon, including phonetic alphabetic used, language code, lexicon ARN, number of lexemes defined in the lexicon, and size of lexicon in bytes.</p>
    #[doc(hidden)]
    pub lexicon_attributes: std::option::Option<crate::model::LexiconAttributes>,
}
impl GetLexiconOutput {
    /// <p>Lexicon object that provides name and the string content of the lexicon. </p>
    pub fn lexicon(&self) -> std::option::Option<&crate::model::Lexicon> {
        self.lexicon.as_ref()
    }
    /// <p>Metadata of the lexicon, including phonetic alphabetic used, language code, lexicon ARN, number of lexemes defined in the lexicon, and size of lexicon in bytes.</p>
    pub fn lexicon_attributes(&self) -> std::option::Option<&crate::model::LexiconAttributes> {
        self.lexicon_attributes.as_ref()
    }
}
/// See [`GetLexiconOutput`](crate::output::GetLexiconOutput).
pub mod get_lexicon_output {

    /// A builder for [`GetLexiconOutput`](crate::output::GetLexiconOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) lexicon: std::option::Option<crate::model::Lexicon>,
        pub(crate) lexicon_attributes: std::option::Option<crate::model::LexiconAttributes>,
    }
    impl Builder {
        /// <p>Lexicon object that provides name and the string content of the lexicon. </p>
        pub fn lexicon(mut self, input: crate::model::Lexicon) -> Self {
            self.lexicon = Some(input);
            self
        }
        /// <p>Lexicon object that provides name and the string content of the lexicon. </p>
        pub fn set_lexicon(mut self, input: std::option::Option<crate::model::Lexicon>) -> Self {
            self.lexicon = input;
            self
        }
        /// <p>Metadata of the lexicon, including phonetic alphabetic used, language code, lexicon ARN, number of lexemes defined in the lexicon, and size of lexicon in bytes.</p>
        pub fn lexicon_attributes(mut self, input: crate::model::LexiconAttributes) -> Self {
            self.lexicon_attributes = Some(input);
            self
        }
        /// <p>Metadata of the lexicon, including phonetic alphabetic used, language code, lexicon ARN, number of lexemes defined in the lexicon, and size of lexicon in bytes.</p>
        pub fn set_lexicon_attributes(
            mut self,
            input: std::option::Option<crate::model::LexiconAttributes>,
        ) -> Self {
            self.lexicon_attributes = input;
            self
        }
        /// Consumes the builder and constructs a [`GetLexiconOutput`](crate::output::GetLexiconOutput).
        pub fn build(self) -> crate::output::GetLexiconOutput {
            crate::output::GetLexiconOutput {
                lexicon: self.lexicon,
                lexicon_attributes: self.lexicon_attributes,
            }
        }
    }
}
impl GetLexiconOutput {
    /// Creates a new builder-style object to manufacture [`GetLexiconOutput`](crate::output::GetLexiconOutput).
    pub fn builder() -> crate::output::get_lexicon_output::Builder {
        crate::output::get_lexicon_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeVoicesOutput {
    /// <p>A list of voices with their properties.</p>
    #[doc(hidden)]
    pub voices: std::option::Option<std::vec::Vec<crate::model::Voice>>,
    /// <p>The pagination token to use in the next request to continue the listing of voices. <code>NextToken</code> is returned only if the response is truncated.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeVoicesOutput {
    /// <p>A list of voices with their properties.</p>
    pub fn voices(&self) -> std::option::Option<&[crate::model::Voice]> {
        self.voices.as_deref()
    }
    /// <p>The pagination token to use in the next request to continue the listing of voices. <code>NextToken</code> is returned only if the response is truncated.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeVoicesOutput`](crate::output::DescribeVoicesOutput).
pub mod describe_voices_output {

    /// A builder for [`DescribeVoicesOutput`](crate::output::DescribeVoicesOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) voices: std::option::Option<std::vec::Vec<crate::model::Voice>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `voices`.
        ///
        /// To override the contents of this collection use [`set_voices`](Self::set_voices).
        ///
        /// <p>A list of voices with their properties.</p>
        pub fn voices(mut self, input: crate::model::Voice) -> Self {
            let mut v = self.voices.unwrap_or_default();
            v.push(input);
            self.voices = Some(v);
            self
        }
        /// <p>A list of voices with their properties.</p>
        pub fn set_voices(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Voice>>,
        ) -> Self {
            self.voices = input;
            self
        }
        /// <p>The pagination token to use in the next request to continue the listing of voices. <code>NextToken</code> is returned only if the response is truncated.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The pagination token to use in the next request to continue the listing of voices. <code>NextToken</code> is returned only if the response is truncated.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeVoicesOutput`](crate::output::DescribeVoicesOutput).
        pub fn build(self) -> crate::output::DescribeVoicesOutput {
            crate::output::DescribeVoicesOutput {
                voices: self.voices,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeVoicesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVoicesOutput`](crate::output::DescribeVoicesOutput).
    pub fn builder() -> crate::output::describe_voices_output::Builder {
        crate::output::describe_voices_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteLexiconOutput {}
/// See [`DeleteLexiconOutput`](crate::output::DeleteLexiconOutput).
pub mod delete_lexicon_output {

    /// A builder for [`DeleteLexiconOutput`](crate::output::DeleteLexiconOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteLexiconOutput`](crate::output::DeleteLexiconOutput).
        pub fn build(self) -> crate::output::DeleteLexiconOutput {
            crate::output::DeleteLexiconOutput {}
        }
    }
}
impl DeleteLexiconOutput {
    /// Creates a new builder-style object to manufacture [`DeleteLexiconOutput`](crate::output::DeleteLexiconOutput).
    pub fn builder() -> crate::output::delete_lexicon_output::Builder {
        crate::output::delete_lexicon_output::Builder::default()
    }
}
