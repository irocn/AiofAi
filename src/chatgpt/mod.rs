use log::error;
use serde_json::Value;
use reqwest::{header::{HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE}, Body, Client, Response};
use tungstenite::{connect, Message};
use url::Url;

#[derive(Clone)]
struct ChatGPT{
    url: String,
    header: reqwest::header::HeaderMap,
    http: Client,
}

impl ChatGPT {
    fn new(access_token:&str, _url: &str) -> Self{

        let mut _headers = reqwest::header::HeaderMap::new();
        _headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        _headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        _headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Ik1UaEVOVUpHTkVNMVFURTRNMEZCTWpkQ05UZzVNRFUxUlRVd1FVSkRNRU13UmtGRVFrRXpSZyJ9.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL3Byb2ZpbGUiOnsiZW1haWwiOiJoYW8uY2hlbkBpcm9jbi5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZX0sImh0dHBzOi8vYXBpLm9wZW5haS5jb20vYXV0aCI6eyJwb2lkIjoib3JnLU0xSVl0NlNISDhnNVQwbFpmdDRGTWRKSyIsInVzZXJfaWQiOiJ1c2VyLUtwM0U0d0tDellmMEhUZEc0NEh2QVdRcCJ9LCJpc3MiOiJodHRwczovL2F1dGgwLm9wZW5haS5jb20vIiwic3ViIjoiYXV0aDB8NjNkZWIwNWM2NWRhNDAyZjE1NmJkYTRlIiwiYXVkIjpbImh0dHBzOi8vYXBpLm9wZW5haS5jb20vdjEiLCJodHRwczovL29wZW5haS5vcGVuYWkuYXV0aDBhcHAuY29tL3VzZXJpbmZvIl0sImlhdCI6MTcxMTA4MTczNiwiZXhwIjoxNzExOTQ1NzM2LCJhenAiOiJUZEpJY2JlMTZXb1RIdE45NW55eXdoNUU0eU9vNkl0RyIsInNjb3BlIjoib3BlbmlkIHByb2ZpbGUgZW1haWwgbW9kZWwucmVhZCBtb2RlbC5yZXF1ZXN0IG9yZ2FuaXphdGlvbi5yZWFkIG9yZ2FuaXphdGlvbi53cml0ZSBvZmZsaW5lX2FjY2VzcyJ9.scWzPGQq8HDlA70Z2EHnPcn_ky8OYgiYVXCHLVQSBo8m45XQ__tCa6Hk_IsGPLUTsplhdUxSfehF__DGKQ33J3S8aWSYBMoYDvQxcq2jPycogcD-46_HvkcVUvztEZT1E8Bbjvc7IvPjNKXALbG2eRE_6GYs_s4R9BFKV-O2XJkcCYVWoQ53OxJNW2Lq-GW-Rg_KkZm6Jxn2NPA8omR7KwWJWUxsS03yd7hl75jyt3bNzbB61e7II2bV1nZeZLcDWuX8gw3UE5TuWEkKT9tDVhJkpu448k8orCGJ2Lv-UOZsuTqjbkpwfiC-rG0x-DfErx6zOSz6I6O0f2b8ZQCD9w")
        );
        _headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
        _headers.insert("sec-ch-ua-platform", HeaderValue::from_static("Linux"));
        _headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
        _headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        _headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
        _headers.insert("oai-device-id", HeaderValue::from_static("12eb9b7c-08e6-46aa-a808-03cbb3f05ddd"));
        _headers.insert("sec-ch-ua", HeaderValue::from_static("\"Chromium\";v=\"121\", \"Not A(Brand\";v=\"99\""));
        _headers.insert("content-type", HeaderValue::from_static("application/json"));
        _headers.insert("referrerPolicy",HeaderValue::from_static("strict-origin-when-cross-origin"));
        _headers.insert("mode",HeaderValue::from_static("cors"));
        _headers.insert("credentials",HeaderValue::from_static("include"));
        _headers.insert("method",HeaderValue::from_static("POST"));
        _headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36"));
        
        ChatGPT{
            url: _url.to_owned(),
            header: _headers,
            http: Client::new(),
        }
    }

    async fn register_websocket(self:&Self, _url: &str, _token :&str){

    }

    async fn send_requirements(self:&Self, reqbody:Value) -> String {
        
        let payload = serde_json::json!(
            {
                "conversation_mode_kind": "primary_assistant"
            }
        );

        let _resp = self.http.post( &self.url )
                .headers( self.header.clone() )
                .body( payload.to_string() )
                .send()
                .await.unwrap();
        
        if _resp.status().as_str() != "200" {
            error!("requirements failed: {}", _resp.status().as_str());
        }
        
        let _json:Value = serde_json::from_str(&_resp.text().await.unwrap()).unwrap();
        _json["token"].to_string()
    }

    async fn send_chat_request(self:&Self, _token:&str, _body:Value) -> Value {

        let _resp = self.http.post(&self.url)
                    .headers( self.header.clone() )
                    .body(_body.to_string() )
                    .send()
                    .await.unwrap();

        let _json:Value = serde_json::from_str(&_resp.text().await.unwrap()).unwrap();
        _json
    }

    // get response from chatgpt
    async fn connect_chatgpt_ws (self:&Self, url:&str){
        match connect(Url::parse(url).unwrap()){
            Ok(ret) => {
                let mut socket = ret.0;
                let response = ret.1;

                println!("Connected to the server");
                println!("Response HTTP code: {}", response.status());
                println!("Response contains the following headers:");
                for (ref header, _value) in response.headers() {
                    println!("* {}", header);
                }
            
                socket.send(Message::Text("Hello WebSocket".into())).unwrap();
                loop {
                    let msg = socket.read().expect("Error reading message");
                    println!("Received: {}", msg);
                }

            },
            Err(err) => {
                let out = err.to_string();
                println!("this error is {:?}", out);
            },
        };
    }

}


#[cfg(test)]
mod tests {
    use super::ChatGPT;


    #[tokio::test]
    async fn test_send_requirements(){
        let _url = "https://chat.openai.com/backend-api/sentinel/chat-requirements";
        let _access_token = "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Ik1UaEVOVUpHTkVNMVFURTRNMEZCTWpkQ05UZzVNRFUxUlRVd1FVSkRNRU13UmtGRVFrRXpSZyJ9.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL3Byb2ZpbGUiOnsiZW1haWwiOiJoYW8uY2hlbkBpcm9jbi5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZX0sImh0dHBzOi8vYXBpLm9wZW5haS5jb20vYXV0aCI6eyJwb2lkIjoib3JnLU0xSVl0NlNISDhnNVQwbFpmdDRGTWRKSyIsInVzZXJfaWQiOiJ1c2VyLUtwM0U0d0tDellmMEhUZEc0NEh2QVdRcCJ9LCJpc3MiOiJodHRwczovL2F1dGgwLm9wZW5haS5jb20vIiwic3ViIjoiYXV0aDB8NjNkZWIwNWM2NWRhNDAyZjE1NmJkYTRlIiwiYXVkIjpbImh0dHBzOi8vYXBpLm9wZW5haS5jb20vdjEiLCJodHRwczovL29wZW5haS5vcGVuYWkuYXV0aDBhcHAuY29tL3VzZXJpbmZvIl0sImlhdCI6MTcxMTA4MTczNiwiZXhwIjoxNzExOTQ1NzM2LCJhenAiOiJUZEpJY2JlMTZXb1RIdE45NW55eXdoNUU0eU9vNkl0RyIsInNjb3BlIjoib3BlbmlkIHByb2ZpbGUgZW1haWwgbW9kZWwucmVhZCBtb2RlbC5yZXF1ZXN0IG9yZ2FuaXphdGlvbi5yZWFkIG9yZ2FuaXphdGlvbi53cml0ZSBvZmZsaW5lX2FjY2VzcyJ9.scWzPGQq8HDlA70Z2EHnPcn_ky8OYgiYVXCHLVQSBo8m45XQ__tCa6Hk_IsGPLUTsplhdUxSfehF__DGKQ33J3S8aWSYBMoYDvQxcq2jPycogcD-46_HvkcVUvztEZT1E8Bbjvc7IvPjNKXALbG2eRE_6GYs_s4R9BFKV-O2XJkcCYVWoQ53OxJNW2Lq-GW-Rg_KkZm6Jxn2NPA8omR7KwWJWUxsS03yd7hl75jyt3bNzbB61e7II2bV1nZeZLcDWuX8gw3UE5TuWEkKT9tDVhJkpu448k8orCGJ2Lv-UOZsuTqjbkpwfiC-rG0x-DfErx6zOSz6I6O0f2b8ZQCD9w";

        let _body = "{\"conversation_mode_kind\":\"primary_assistant\"}";
        let _gpt_client = ChatGPT::new(_access_token, _url);
        
        _gpt_client.send_requirements(_body.into()).await;

    }

    #[tokio::test]
    async fn test_connect_chatgpt_ws(){
        let _gpt_client = ChatGPT::new("_access_token", "_url");
        _gpt_client.connect_chatgpt_ws("wss://chatgpt-async-webps-prod-southcentralus-19.webpubsub.azure.com/client/hubs/conversations?access_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJodHRwczovL2NoYXRncHQtYXN5bmMtd2VicHMtcHJvZC1zb3V0aGNlbnRyYWx1cy0xOS53ZWJwdWJzdWIuYXp1cmUuY29tL2NsaWVudC9odWJzL2NvbnZlcnNhdGlvbnMiLCJpYXQiOjE3MTEyMDQyMjEsImV4cCI6MTcxMTIwNzgyMSwic3ViIjoidXNlci1LcDNFNHdLQ3pZZjBIVGRHNDRIdkFXUXAiLCJyb2xlIjpbIndlYnB1YnN1Yi5qb2luTGVhdmVHcm91cC51c2VyLUtwM0U0d0tDellmMEhUZEc0NEh2QVdRcCJdLCJ3ZWJwdWJzdWIuZ3JvdXAiOlsidXNlci1LcDNFNHdLQ3pZZjBIVGRHNDRIdkFXUXAiXX0.GVA6cHWU_ftvKbll8il6fjPEz_B3zWvSMssg9Uo5STc").await;
    }
}

/*

fetch("https://chat.openai.com/backend-api/conversation", {
  "headers": {
    "accept": "text/event-stream",
    "accept-language": "en-US,en;q=0.9",
    "authorization": "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Ik1UaEVOVUpHTkVNMVFURTRNMEZCTWpkQ05UZzVNRFUxUlRVd1FVSkRNRU13UmtGRVFrRXpSZyJ9.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL3Byb2ZpbGUiOnsiZW1haWwiOiJoYW8uY2hlbkBpcm9jbi5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZX0sImh0dHBzOi8vYXBpLm9wZW5haS5jb20vYXV0aCI6eyJwb2lkIjoib3JnLU0xSVl0NlNISDhnNVQwbFpmdDRGTWRKSyIsInVzZXJfaWQiOiJ1c2VyLUtwM0U0d0tDellmMEhUZEc0NEh2QVdRcCJ9LCJpc3MiOiJodHRwczovL2F1dGgwLm9wZW5haS5jb20vIiwic3ViIjoiYXV0aDB8NjNkZWIwNWM2NWRhNDAyZjE1NmJkYTRlIiwiYXVkIjpbImh0dHBzOi8vYXBpLm9wZW5haS5jb20vdjEiLCJodHRwczovL29wZW5haS5vcGVuYWkuYXV0aDBhcHAuY29tL3VzZXJpbmZvIl0sImlhdCI6MTcxMTA4MTczNiwiZXhwIjoxNzExOTQ1NzM2LCJhenAiOiJUZEpJY2JlMTZXb1RIdE45NW55eXdoNUU0eU9vNkl0RyIsInNjb3BlIjoib3BlbmlkIHByb2ZpbGUgZW1haWwgbW9kZWwucmVhZCBtb2RlbC5yZXF1ZXN0IG9yZ2FuaXphdGlvbi5yZWFkIG9yZ2FuaXphdGlvbi53cml0ZSBvZmZsaW5lX2FjY2VzcyJ9.scWzPGQq8HDlA70Z2EHnPcn_ky8OYgiYVXCHLVQSBo8m45XQ__tCa6Hk_IsGPLUTsplhdUxSfehF__DGKQ33J3S8aWSYBMoYDvQxcq2jPycogcD-46_HvkcVUvztEZT1E8Bbjvc7IvPjNKXALbG2eRE_6GYs_s4R9BFKV-O2XJkcCYVWoQ53OxJNW2Lq-GW-Rg_KkZm6Jxn2NPA8omR7KwWJWUxsS03yd7hl75jyt3bNzbB61e7II2bV1nZeZLcDWuX8gw3UE5TuWEkKT9tDVhJkpu448k8orCGJ2Lv-UOZsuTqjbkpwfiC-rG0x-DfErx6zOSz6I6O0f2b8ZQCD9w",
    "content-type": "application/json",
    "oai-device-id": "12eb9b7c-08e6-46aa-a808-03cbb3f05ddd",
    "oai-language": "en-US",
    "openai-sentinel-chat-requirements-token": "gAAAAABl_pcvmJpCbh17FMQTbIfSAhkObU2AO09B8GfggL2uQIMUeO_gzuj-xZhqWJ-LvXZc61_6bvibjhFiqC_F8hozMI_ri6LnTqsy-Tk6qlI-UlhK7l4IfhfEUH7yAzK5l_MhSRZbRRFqPVht8pDG8N3lz6odFLEsAdNYzrFZyA3NofhfxFIXSOPTWwIA7lxGG5KP96vazmqTSc1gQXAo56vS8YA_qbSNd7wzFTj1MzCuIEidRpKZg_TOAqREQHNflBhvUEswHCbQdPXMgWXnedaO6xxXSw==",
    "sec-ch-ua": "\"Chromium\";v=\"121\", \"Not A(Brand\";v=\"99\"",
    "sec-ch-ua-mobile": "?0",
    "sec-ch-ua-platform": "\"Linux\"",
    "sec-fetch-dest": "empty",
    "sec-fetch-mode": "cors",
    "sec-fetch-site": "same-origin"
  },
  "referrer": "https://chat.openai.com/c/9cae68f0-5e68-464f-b295-3a767b907dc6",
  "referrerPolicy": "strict-origin-when-cross-origin",
  "body": "{\"action\":\"next\",\"messages\":[{\"id\":\"aaa2de75-d7c8-4735-b79b-3cb8596dfd1b\",\"author\":{\"role\":\"user\"},\"content\":{\"content_type\":\"text\",\"parts\":[\"This comparison appears to be unintentional because the types have no overlap.\"]},\"metadata\":{}}],\"conversation_id\":\"9cae68f0-5e68-464f-b295-3a767b907dc6\",\"parent_message_id\":\"5027ad20-4d8d-4366-b50f-25c143e4830e\",\"model\":\"text-davinci-002-render-sha\",\"timezone_offset_min\":0,\"suggestions\":[],\"history_and_training_disabled\":false,\"conversation_mode\":{\"kind\":\"primary_assistant\",\"plugin_ids\":null},\"force_paragen\":false,\"force_rate_limit\":false,\"websocket_request_id\":\"68ecc5e2-1b5f-4232-b3fa-36c8d5986fc7\"}",
  "method": "POST",
  "mode": "cors",
  "credentials": "include"
});

*/

/*
    var socket = new WebSocket("wss://chatgpt-async-webps-prod-southcentralus-19.webpubsub.azure.com/client/hubs/conversations?access_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJodHRwczovL2NoYXRncHQtYXN5bmMtd2VicHMtcHJvZC1zb3V0aGNlbnRyYWx1cy0xOS53ZWJwdWJzdWIuYXp1cmUuY29tL2NsaWVudC9odWJzL2NvbnZlcnNhdGlvbnMiLCJpYXQiOjE3MTExOTUxODAsImV4cCI6MTcxMTE5ODc4MCwic3ViIjoidXNlci1LcDNFNHdLQ3pZZjBIVGRHNDRIdkFXUXAiLCJyb2xlIjpbIndlYnB1YnN1Yi5qb2luTGVhdmVHcm91cC51c2VyLUtwM0U0d0tDellmMEhUZEc0NEh2QVdRcCJdLCJ3ZWJwdWJzdWIuZ3JvdXAiOlsidXNlci1LcDNFNHdLQ3pZZjBIVGRHNDRIdkFXUXAiXX0.ZG32M4I1zKenBf76E0OxuVeytRe9R8G0-yzqkPLWHZ8");
    socket.onopen = function(event) {
      console.log(`Open wss_url`);
    };
  
    socket.onmessage = function(event) {
      console.log('Message received:', event.data);
    };
*/