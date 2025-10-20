use std::sync::Arc;

use log::{LevelFilter, error, info};
use tokio::sync::mpsc;
use vivian::types::common::MessageScene;
use vivian::types::message::get_plain_text_from_segments;
use vivian::types::message::out_going::{OutgoingSegment, TextData};
use vivian::{Communication, Event, EventKind, MilkyClient, Result};
use vivian::{WebSocketConfig, logger};

// 辅助函数，用于创建文本消息段
fn text_segment(text: &str) -> OutgoingSegment {
    OutgoingSegment::Text(TextData {
        text: text.to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger(Some(LevelFilter::Info)); // 初始化日志

    // 创建事件通道
    let (event_tx, mut event_rx) = mpsc::channel::<Event>(100);

    // 初始化 MilkyClient
    // 示例中使用的是WebSocket的通信方式，如果你想通过WebHook方式与服务端通信，可以参考下面的代码
    // let wh_config = WebHookConfig::new(None, 8080, "http://127.0.0.1:3000".to_string(), None);
    // let client = MilkyClient::new(Communication::WebHook(wh_config), event_tx)?;
    let ws_config = WebSocketConfig::new("ws://127.0.0.1:3002".to_string(), None);
    let client = MilkyClient::new(Communication::WebSocket(ws_config), event_tx)?;
    let client = Arc::new(client);

    // 连接到件流
    if let Err(e) = client.connect_events().await {
        error!("未能连接到事件流: {e:?}");
        return Err(e);
    }
    info!("成功连接到 Milky 服务器事件流。");

    // 启动一个异步任务来处理接收到的事件
    let client_for_task = Arc::clone(&client);
    let _event_handle = tokio::spawn(async move {
        info!("事件监听器已启动。");
        while let Some(event) = event_rx.recv().await {
            info!("收到事件: {event:?}",); // 打印原始事件

            match event.kind {
                EventKind::MessageReceive(incoming_msg) => {
                    let plain_text = get_plain_text_from_segments(&incoming_msg.segments);
                    info!(
                        "收到来自 {} 的消息 ({}): {}",
                        incoming_msg.sender_id,
                        serde_json::to_string(&incoming_msg.message_scene).unwrap(),
                        plain_text
                    );

                    // 示例：复读
                    if incoming_msg.message_scene == MessageScene::Friend
                        && plain_text.starts_with("/echo")
                    {
                        let reply_segments =
                            vec![text_segment(plain_text.replace("/echo", "").trim())];
                        match client_for_task
                            .send_private_message(incoming_msg.sender_id, reply_segments)
                            .await
                        {
                            Ok(resp) => info!("自动回复成功: seq={}", resp.message_seq),
                            Err(e) => error!("自动回复失败: {e:?}",),
                        }
                    }
                }
                EventKind::GroupMemberIncrease(data) => {
                    info!("群 {} 新成员加入: {}", data.group_id, data.user_id);
                }
                // ... 处理其他事件类型
                _ => {}
            }
        }
        info!("事件监听器已停止。");
    });

    // 等待连接稳定和事件监听器启动
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // 调用 API 示例
    // 获取登录信息
    match client.get_login_info().await {
        Ok(login_info) => {
            info!(
                "登录信息: QQ={}, 昵称='{}'",
                login_info.uin, login_info.nickname
            );
        }
        Err(e) => {
            error!("未能获取登录信息: {e:?}",);
        }
    }

    // 发送私聊消息 (请替换为有效的 user_id)
    let user_id_to_send: i64 = 123456789; // 示例QQ号
    let message_to_send = vec![text_segment("你好，这是一个来自 Vivian SDK 的测试消息！")];
    match client
        .send_private_message(user_id_to_send, message_to_send)
        .await
    {
        Ok(response) => {
            info!(
                "私聊消息成功发送至 {}: message_seq={}",
                user_id_to_send, response.message_seq
            );
        }
        Err(e) => {
            error!("未能发送私聊消息至 {user_id_to_send}: {e:?}");
        }
    }

    // 保持主程序运行以处理事件。
    info!("示例正在运行。按 Ctrl-C 退出。");
    tokio::signal::ctrl_c().await?; // 等待 Ctrl-C信号
    info!("收到 Ctrl-C，正在关闭...");
    client.shutdown().await;
    // 可以选择性地等待一小段时间，以确保关闭消息被处理
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    Ok(())
}
