# Vivian SDK

[![Repository](https://img.shields.io/badge/repository-GitHub-blue.svg)](https://github.com/hanasa2023/vivian-rs)
[![Latest version](https://img.shields.io/crates/v/vivian.svg)](https://crates.io/crates/vivian)
[![Documentation](https://docs.rs/vivian/badge.svg)](https://docs.rs/vivian)
![License](https://img.shields.io/crates/l/vivian.svg)

Vivian 是一个使用 Rust 编写的软件开发工具包 (SDK)，用于与 [Milky](https://milky.ntqqrev.org/)
后端服务进行交互。它提供了一套便捷的API客户端，用于执行各种操作，
如发送消息、管理群组、处理文件，并能通过 WebSocket/WebHook 接收和处理实时事件。

## 特性

- **异步客户端**: 基于 `tokio` 构建，所有API调用和事件处理都是异步的。
- **全面的API覆盖**:
  - **消息处理**: 发送和接收私聊、群聊消息，获取历史消息，撤回消息，处理消息段（文本、图片、@提及、表情等）。
  - **群组管理**: 创建、查询、修改群信息，管理群成员（邀请、踢出、禁言、设置管理员），处理群公告、群文件等。
  - **好友互动**: 发送戳一戳，资料卡点赞。
  - **文件操作**: 上传和下载私聊及群文件，管理群文件和文件夹。
  - **请求处理**: 同意或拒绝好友请求、加群请求。
  - **系统信息**: 获取登录信息、好友列表、群列表等。
- **实时事件处理**: 通过 WebSocket 接收服务器推送的各类事件，如新消息、用户加入/退出群组等。
- **强类型接口**: 所有API请求参数和响应数据都有明确的Rust结构体定义，利用 `serde`进行序列化和反序列化，确保类型安全。
- **自定义日志**: 内置可定制的日志记录器 ()，支持彩色输出和级别过滤。
- **错误处理**: 定义了详细的错误类型 `MilkyError` (`error.rs`) 和统一的 `Result<T>`，方便错误处理。
- **模块化设计**: 清晰的模块划分，包括 `client` (核心客户端)、`api` (各API端点实现)、`types` (数据结构定义)、`error` (错误处理) 和 `logger` (日志模块)。

## 快速开始

### 1. 添加依赖

将 `vivian` 添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
vivian = "0.1" # 或者使用 git/crates.io 依赖
tokio = { version = "1", features = ["full"] }
log = "0.4"
# 其他您项目可能需要的依赖
```

### 2. 初始化日志 (可选但推荐)

```rust
use vivian::logger;
use log::LevelFilter;

fn main() {
    logger::init_logger(Some(LevelFilter::Info)); // 设置日志级别为 Info
    // ... 您的代码
}
```

### 3. 创建和使用 `MilkyClient`

以下是一个基本的使用示例，展示了如何初始化客户端、连接事件流、处理事件以及调用API。

```rust
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

    // 连接到事件流
    if let Err(e) = client.connect_events().await {
        error!("未能连接到事件流: {:?}", e);
        return Err(e);
    }
    info!("成功连接到 Milky 服务器事件流。");

    // 启动一个异步任务来处理接收到的事件
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
                        incoming_msg.sender_id,
                        serde_json::to_string(&incoming_msg.message_scene).unwrap(),
                        plain_text
                    );

                    // 示例：复读
                    if incoming_msg.message_scene == MessageScene::Friend
                        && plain_text.starts_with("/echo")
                    {
                        let reply_segments = vec![text_segment(plain_text.replace("/echo", "").trim())];
                        match client_for_task
                            .send_private_message(incoming_msg.sender_id, reply_segments)
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
            error!("未能获取登录信息: {:?}", e);
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
            error!("未能发送私聊消息至 {}: {:?}", user_id_to_send, e);
        }
    }

    // 保持主程序运行以处理事件。在实际应用中，您需要更健壮的关闭逻辑。
    info!("示例正在运行。按 Ctrl-C 退出。");
    tokio::signal::ctrl_c().await?; // 等待 Ctrl-C信号
    info!("收到 Ctrl-C，正在关闭...");

    Ok(())
}

```

## 贡献

欢迎对本项目做出贡献！如果您发现任何bug或有功能建议，请随时提交 Issues 或 Pull Requests。
