use chzzk::{chat::chat_client::ChatClient, ChzzkClient};

// Find these values in your browser cookies
const AUT: &str = "QQnjgWC/jZUauV38IXvOZI1MUlU76LuMgOzINPqQQMCKKhdDtmnnQK/48pXVyHyK";
const SES: &str = "AAABfs0ukOXRFtJUVD7Xgu0WDKILF4U19NOOV1DhguI5noS1xE7zlQCRtIpB/xuwHf02G8Jywfp1MDG7A8sWxMXmkQzT8uDv5x7r0whU3BqawVG8rS4a1O778e7w5gM3yraxBhhFet+bO2gd4Nk9z0CqZWLBc6/d6jrlpvHtqs3xbyAV/w2Nc3wUwjP1Qnvu2vBAZy+irRwIh+WvVHcGb3g5KheXHLhRYlL9R6KpHhNvLDijHki22BTnoEnPFyS2zfg/0fDE5lZzPIs/IZPF0+E1gskleZgY74l80xDg4xxg/8uSwDwZGkv2V9d2Hf/ppumlLux/il34T4FnLxZNtMpfqCQPAMmJFx1vxYrqlWDnifl32JbXhJHt/W+xo+xSlJmgXpTHerTjK4jNPU0kk6YrkYrlhvwyXVKjM3ojDnCBmZy8u624296YlZZQpja4erdwspa4d3XsKtSnTEg7Td63SFCOdB5Hm6FFP1dPGI8ixI1BktWprDZgpx04gGcowCCakQ==";

#[tokio::main]
async fn main() {
    let mut client = ChzzkClient::new();
    client.sign_in(AUT, SES);
    let mut chat_client = ChatClient::new(
        client, "1dac6492f81d89e261f692bb6b79ff57"); // feel free to send chat on this channel
    chat_client.connect().await.unwrap();
    chat_client.send_chat("hello chat").await.unwrap();
}