use log::{error, info};
use tokio::sync::mpsc;
use vivian::types::message::out_going::OutgoingSegment;
use vivian::types::message::out_going::TextData;
use vivian::{MilkyClient, Result, types::event::Event};

fn text_segment(text: &str) -> OutgoingSegment {
    OutgoingSegment::Text(TextData {
        text: text.to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init(); // 初始化日志记录器

    let (event_tx, mut event_rx) = mpsc::channel::<Event>(100); // 事件通道

    // MilkyClient期望一个HTTP/HTTPS基础URL用于API调用，
    // 并且它将从中派生出WebSocket URL。
    let client = MilkyClient::new("http://127.0.0.1:8080", None, event_tx)?; // 请替换为您的Milky服务器HTTP地址

    // 连接到WebSocket以接收事件，并在后台开始处理它们
    if let Err(e) = client.connect_events().await {
        error!("未能连接到事件流: {:?}", e);
        return Err(e);
    }
    info!("成功连接到Milky服务器事件流。");

    // 启动一个任务来接收和处理事件
    let _event_handle = tokio::spawn(async move {
        info!("事件监听器已启动。");
        while let Some(event) = event_rx.recv().await {
            // 收到的 'event' 是 vivian::types::event::Event 类型
            // 其中包含 { time: i64, self_id: i64 }
            info!("收到事件: {:?}", event);

            // 要处理特定的事件类型（如 PrivateMessage, GroupMessage 等），
            // 您通常需要 EventKind 枚举。
            // 当前 client.rs 的实现发送的是基础的 Event 结构体。
            // 对于详细的事件处理，您可能需要：
            // 1. 修改 client.rs 以解析并发送包含 EventKind 的结构体。
            // 2. 或者，如果事件是带有类型字段的JSON字符串，则在此处解析它们。
            // 目前，我们只记录基础的事件数据。
            // 示例：
            // if let Ok(full_event_data) = serde_json::from_str::<vivian::types::event::EventKind>(&event.raw_data_if_available) {
            //     match full_event_data {
            //         vivian::types::event::EventKind::MessageReceive(msg_event) => {
            //             if msg_event.message_type == "private" {
            //                 info!("来自 {} 的私聊消息: {}", msg_event.user_id.unwrap_or_default(), msg_event.alt_message);
            //             } else if msg_event.message_type == "group" {
            //                 info!("群 {} 中来自用户 {} 的消息: {}", msg_event.group_id.unwrap_or_default(), msg_event.user_id.unwrap_or_default(), msg_event.alt_message);
            //             }
            //         },
            //         // 处理其他 EventKind 变体
            //         _ => {}
            //     }
            // }
        }
        info!("事件监听器已停止。");
    });

    // 留出一些时间让连接稳定并且事件监听器启动。
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // --- API 调用示例 ---
    // 注意：这些调用是从主任务进行的。
    // 如果您需要在事件处理任务内部调用客户端方法，
    // 您将需要使用 Arc<MilkyClient>。

    // 示例 1：获取登录信息
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

    // 示例 2：获取好友列表
    match client.get_friend_list(false).await {
        // false 表示不使用缓存
        Ok(friend_list_response) => {
            info!("获取到 {} 个好友。", friend_list_response.friends.len());
            for friend in friend_list_response.friends.iter().take(3) {
                // 打印前3个好友
                info!("  - 好友 ID: {}, 昵称: {}", friend.user_id, friend.nickname);
            }
        }
        Err(e) => {
            error!("未能获取好友列表: {:?}", e);
        }
    }

    // 示例 3：发送私聊消息
    let user_id_to_send: i64 = 123456789; // 请替换为有效的QQ号以进行测试
    let message_to_send = vec![
        text_segment("来自 vivian SDK 的问候! "),
        text_segment("这是一条测试消息。"),
    ];
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

    // 示例 4：设置群名称（需要有效的 group_id）
    let group_id_to_rename: i64 = 987654321; // 请替换为有效的群ID
    let new_group_name = "Vivian 测试群名称".to_string();
    match client
        .set_group_name(group_id_to_rename, new_group_name.clone())
        .await
    {
        Ok(_) => {
            info!(
                "成功将群 {} 名称设置为 '{}'",
                group_id_to_rename, new_group_name
            );
        }
        Err(e) => {
            error!("未能为 {} 设置群名称: {:?}", group_id_to_rename, e);
        }
    }

    // 保持主程序运行以允许事件循环处理事件。
    // 在实际应用中，您可能会有更复杂的关闭逻辑。
    info!("示例正在运行。按 Ctrl-C 退出。");
    // 您可以使用 tokio::signal::ctrl_c() 来实现优雅关闭：
    // tokio::signal::ctrl_c().await?;
    // info!("收到 Ctrl-C，正在关闭。");

    // 在此示例中，让它运行一段时间或直到手动停止
    std::thread::sleep(std::time::Duration::from_secs(600)); // 运行10分钟

    // 完成后关闭WebSocket连接。
    client.close_event_stream().await?;
    info!("客户端事件流已关闭。");

    // 等待事件处理任务完成（可选）
    // if let Err(e) = event_handle.await {
    //     eprintln!("事件处理任务发生 panic: {:?}", e);
    // }

    Ok(())
}
