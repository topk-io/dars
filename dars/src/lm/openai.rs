use async_openai::{
    Client,
    config::Config,
    types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart,
        CreateChatCompletionRequest, ImageDetail, ImageUrl, ResponseFormat,
        ResponseFormatJsonSchema,
    },
};
use async_trait::async_trait;
use schemars::Schema;

use crate::{
    Error,
    lm::{LM, Message, MessageContent},
};

#[derive(Debug, Default)]
pub struct ModelConfig {
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
}

impl ModelConfig {
    pub fn model(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            ..Default::default()
        }
    }
}

/// LM client for providers that support OpenAI API.
pub struct OpenAILM<C: Config> {
    client: Client<C>,
    model_config: ModelConfig,
}

impl<C: Config> OpenAILM<C> {
    pub fn new(client_config: C, model_config: ModelConfig) -> Self {
        Self {
            client: Client::<C>::with_config(client_config),
            model_config,
        }
    }
}

#[async_trait]
impl<C: Config + 'static> LM for OpenAILM<C> {
    async fn call(&self, messages: Vec<Message>, schema: Option<Schema>) -> Result<String, Error> {
        let mut req = CreateChatCompletionRequest {
            messages: vec![],
            model: self.model_config.model.clone(),
            temperature: self.model_config.temperature,
            max_completion_tokens: self.model_config.max_tokens,
            top_p: self.model_config.top_p,
            response_format: schema.map(convert_schema_to_response_format),
            ..Default::default()
        };
        for m in messages {
            req.messages.push(m.try_into()?);
        }
        let response = self.client.chat().create(req).await?;
        let content = response.choices[0].message.content.clone();
        Ok(content.unwrap_or_default())
    }
}

impl TryFrom<Message> for ChatCompletionRequestMessage {
    type Error = Error;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        let msg = match msg {
            Message::System { instruction } => {
                ChatCompletionRequestMessage::System(instruction.into())
            }
            Message::User { content } => {
                let mut parts = Vec::with_capacity(content.len());
                for msg in content {
                    let part = match msg {
                        MessageContent::Text { text } => {
                            ChatCompletionRequestUserMessageContentPart::Text(text.into())
                        }
                        MessageContent::Image { url } => {
                            ChatCompletionRequestUserMessageContentPart::ImageUrl(
                                ImageUrl {
                                    url,
                                    detail: Some(ImageDetail::Auto),
                                }
                                .into(),
                            )
                        }
                    };

                    parts.push(part);
                }
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    name: None,
                    content: ChatCompletionRequestUserMessageContent::Array(parts),
                })
            }
            Message::Assistant { content } => match content {
                MessageContent::Text { text } => {
                    ChatCompletionRequestMessage::Assistant(text.into())
                }
                MessageContent::Image { url } => image_message(url),
            },
        };

        Ok(msg)
    }
}

fn image_message(url: String) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        name: None,
        content: ChatCompletionRequestUserMessageContent::Array(vec![
            ChatCompletionRequestUserMessageContentPart::ImageUrl(
                ImageUrl { url, detail: None }.into(),
            ),
        ]),
    })
}

fn convert_schema_to_response_format(schema: Schema) -> ResponseFormat {
    ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            name: "schema".into(),
            schema: Some(schema.to_value()),
            description: None,
            strict: None,
        },
    }
}
