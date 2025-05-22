use std::sync::Arc;

use log::{LevelFilter, error, info};
use tokio::sync::mpsc;
use vivian::logger; // 确保 logger 模块被正确引用
use vivian::types::message::get_plain_text_from_segments; // 辅助函数
use vivian::types::message::out_going::{OutgoingSegment, TextData};
use vivian::{Event, EventKind, MilkyClient, Result};

// 辅助函数，用于创建文本消息段
fn text_segment(text: &str) -> OutgoingSegment {
    OutgoingSegment::Text(TextData {
        text: text.to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger(Some(LevelFilter::Info)); // 初始化日志

    // 1. 创建事件通道
    let (event_tx, mut event_rx) = mpsc::channel::<Event>(100);

    // 2. 初始化 MilkyClient
    //    请将 "http://127.0.0.1:3000" 替换为您的 Milky 服务器的实际 HTTP 地址。
    //    第二个参数是可选的 access_token。
    let client = MilkyClient::new("http://127.0.0.1:3000", None, event_tx)?;
    let client = Arc::new(client);

    // 3. 连接到 WebSocket 事件流
    if let Err(e) = client.connect_events().await {
        error!("未能连接到事件流: {:?}", e);
        return Err(e);
    }
    info!("成功连接到 Milky 服务器事件流。");

    // 4. 启动一个异步任务来处理接收到的事件
    let client_for_task = Arc::clone(&client);
    let _event_handle = tokio::spawn(async move {
        info!("事件监听器已启动。");
        while let Some(event) = event_rx.recv().await {
            info!("收到事件: {:?}", event); // 打印原始事件

            match event.kind {
                EventKind::MessageReceive(incoming_msg) => {
                    let plain_text = get_plain_text_from_segments(&incoming_msg.segments);
                    info!(
                        "收到来自 {} 的消息 ({}): {}",
                        incoming_msg.sender_id, incoming_msg.message_scene, plain_text
                    );

                    // 示例：复读
                    if incoming_msg.message_scene == "friend" && plain_text.starts_with("/echo") {
                        let reply_segments = vec![text_segment(&(plain_text.replace("/echo", "")))];
                        match client_for_task
                            .send_private_msg(incoming_msg.sender_id, reply_segments)
                            .await
                        {
                            Ok(resp) => info!("自动回复成功: seq={}", resp.message_seq),
                            Err(e) => error!("自动回复失败: {:?}", e),
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

    // 5. 调用 API 示例
    // 获取登录信息
    match client.get_login_info().await {
        Ok(login_info) => {
            info!(
                "登录信息: QQ={}, 昵称='{}'",
                login_info.uin, login_info.nickname
            );
        }
        Err(e) => {
            error!("未能获取登录信息: {:?}", e);
        }
    }

    // 发送私聊消息 (请替换为有效的 user_id)
    let user_id_to_send: i64 = 123456789; // 示例QQ号
    let message_to_send = vec![text_segment("你好，这是一个来自 Vivian SDK 的测试消息！")];
    match client
        .send_private_msg(user_id_to_send, message_to_send)
        .await
    {
        Ok(response) => {
            info!(
                "私聊消息成功发送至 {}: message_seq={}",
                user_id_to_send, response.message_seq
            );
        }
        Err(e) => {
            error!("未能发送私聊消息至 {}: {:?}", user_id_to_send, e);
        }
    }

    // 保持主程序运行以处理事件。在实际应用中，您需要更健壮的关闭逻辑。
    info!("示例正在运行。按 Ctrl-C 退出。");
    tokio::signal::ctrl_c().await?; // 等待 Ctrl-C信号
    info!("收到 Ctrl-C，正在关闭...");

    // 6. 关闭事件流
    client.close_event_stream().await?;
    info!("客户端事件流已关闭。");

    Ok(())
}
