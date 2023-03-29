use eventsource_stream::{EventStreamError, Eventsource};
use futures::TryStreamExt;
use log::{error, info};
use reqwest;
use serde::{ser::Serializer, Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env::consts::OS, time::Duration};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Stream(#[from] EventStreamError<reqwest::Error>),
    #[error("Custom Error: (code: {code:?}, message: {msg:?})")]
    Custom { code: u16, msg: String },
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressPayload {
    pub id: u64,
    pub detail: String,
    pub role: String,
    pub finish_reason: String,
    pub conversation_id: Option<String>,
    pub parent_message_id: String,
}

impl ProgressPayload {
    pub fn emit_progress(&self, handle: &AppHandle) {
        handle.emit_all("CHAT_FETCHEING_PROGRESS", &self).ok();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct FetchOption {
    pub proxy: Option<String>,
    pub host: String,
    pub apiKey: String,
    pub model: String,
    pub temperature: f32,

    pub conversationId: Option<String>,
    pub parentMessageId: Option<String>,
    pub messageId: Option<String>,
    pub action: Option<String>,
    pub timeoutMs: Option<String>,
    // pub onProgress?: (partialResponse: ChatMessage) => void
    // pub abortSignal?: AbortSignal
}

#[tauri::command]
pub async fn fetch_chat_api_by_access_token(
    handle: AppHandle,
    id: u64,
    messages: Vec<Message>,
    option: FetchOption,
) -> Result<u64> {
    // https://platform.openai.com/docs/guides/chat/introduction
    // let url = "https://api.openai.com/v1/chat/completions";
    log::info!(
        "> send message: length: {}, option: {:?}",
        messages.len(),
        option,
    );

    let _messages_id = option.messageId.unwrap_or(Uuid::new_v4().to_string());
    let _parent_message_id = option.parentMessageId.unwrap_or(Uuid::new_v4().to_string());
    let conversation_id = option.conversationId;
    let action = option.action.unwrap_or("next".to_string());

    let last_message = messages.last().unwrap();

    let mut body = json!({
            "action": action,
            "messages": [{
                "id": _messages_id,
                "role": "user".to_string(),
                "content": {
                    "content_type": "text".to_string(),
                    "parts": [last_message.content]
                }
            }],
            "model": option.model,
            "parent_message_id": _parent_message_id,
    });

    info!("> conversation_id: {:?}", conversation_id);
    if let Some(conversation_id) = conversation_id {
        body["conversation_id"] = conversation_id.into();
    }
    log::info!("> send message: body {}", body);

    let proxy_str = option.proxy.unwrap_or(String::from(""));

    let client: reqwest::Client = {
        log::info!("proxy is: {}", proxy_str);
        let mut client_builder = reqwest::Client::builder();
        if proxy_str.len() > 0 {
            let proxy = reqwest::Proxy::all(proxy_str).unwrap();
            client_builder = client_builder.proxy(proxy);
        }
        client_builder.build().unwrap()
    };
    info!("> body body: {}", body);

    let res = client
        .post(option.host)
        .header("Accept", "text/event-stream")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", option.apiKey))
        .header(
            reqwest::header::USER_AGENT,
            format!("ChatGPT-Tauri ({})", OS),
        )
        .timeout(Duration::from_secs(600))
        .body(body.to_string())
        .send()
        .await?;
    info!("> receive message: {}", id);

    let status_code = res.status().as_u16();
    info!("> receive message status code: {}", status_code);
    if status_code != 200 {
        let error_msg = res.text().await?;
        log::error!("{}", error_msg);
        return Err(Error::Custom {
            code: status_code,
            msg: String::from(error_msg),
        });
    }

    let mut stream = res.bytes_stream().eventsource();
    while let Some(chunk) = stream.try_next().await? {
        let chunk = chunk.data;
        if chunk == "[DONE]" {
            return Ok(id);
        } else {
            match serde_json::from_str::<Value>(&chunk) {
                Ok(object) => {
                    // info!("> object: {:?}", object);
                    let _message = &object["message"];
                    let _conversation_id =
                        String::from(object["conversation_id"].as_str().unwrap_or("")); // 从 JSON 对象获取 conversationId
                    let content =
                        String::from(_message["content"]["parts"][0].as_str().unwrap_or(""));
                    let role = String::from(_message["author"]["role"].as_str().unwrap_or(""));
                    let finish_reason = String::from(
                        _message["metadata"]["finish_details"]["type"]
                            .as_str()
                            .unwrap_or(""),
                    );
                    let progress = ProgressPayload {
                        id,
                        detail: content,
                        role,
                        finish_reason,
                        conversation_id: Some(_conversation_id),
                        parent_message_id: _messages_id.clone(),
                    };
                    // info!("> progress: {:?}", progress);
                    progress.emit_progress(&handle);
                }
                Err(err) => {
                    // 处理 JSON 转换错误
                    info!("Failed to parse JSON object: {:?}", err); // 中途会打印一个时间戳，导致无法转换为 JSON
                    continue; // 跳过当前循环，继续下一个循环
                }
            }
        }
    }

    Ok(id)
}
