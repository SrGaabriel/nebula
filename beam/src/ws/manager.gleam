import glats
import gleam/erlang/process
import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}

pub type SubscriptionConfig {
  SubscriptionConfig(
    subject: String,
    handler: fn(glats.Message) -> Nil,
    timeout_ms: Option(Int),
    on_error: Option(fn(String) -> Nil),
    max_retries: Int,
  )
}

pub type SubscriptionHandle {
  SubscriptionHandle(subject: String, pid: process.Pid)
}

pub fn new_subscription(
  subject: String,
  handler: fn(glats.Message) -> Nil,
) -> SubscriptionConfig {
  SubscriptionConfig(
    subject: subject,
    handler: handler,
    timeout_ms: None,
    on_error: None,
    max_retries: 3,
  )
}

pub fn with_timeout(
  config: SubscriptionConfig,
  timeout_ms: Int,
) -> SubscriptionConfig {
  SubscriptionConfig(..config, timeout_ms: Some(timeout_ms))
}

pub fn with_error_handler(
  config: SubscriptionConfig,
  error_handler: fn(String) -> Nil,
) -> SubscriptionConfig {
  SubscriptionConfig(..config, on_error: Some(error_handler))
}

pub fn with_max_retries(
  config: SubscriptionConfig,
  max_retries: Int,
) -> SubscriptionConfig {
  SubscriptionConfig(..config, max_retries: max_retries)
}

pub fn start_managed_subscriptions(
  conn: glats.Connection,
  subscriptions: List(SubscriptionConfig),
) -> List(SubscriptionHandle) {
  subscriptions
  |> list.map(fn(config) {
    let pid =
      process.spawn_unlinked(fn() {
        start_subscription_with_retry(conn, config, 0)
      })
    SubscriptionHandle(subject: config.subject, pid: pid)
  })
}

fn start_subscription_with_retry(
  conn: glats.Connection,
  config: SubscriptionConfig,
  retry_count: Int,
) -> Nil {
  let subject_ref = process.new_subject()
  case glats.subscribe(conn, subject_ref, config.subject, []) {
    Ok(_sid) -> {
      io.println("✅ Subscribed to: " <> config.subject)
      enhanced_listen_loop(subject_ref, config, 0)
    }
    Error(_) -> {
      let error_msg = "Failed to subscribe to: " <> config.subject

      case config.on_error {
        Some(error_handler) -> error_handler(error_msg)
        None -> io.println("❌ " <> error_msg)
      }

      case retry_count < config.max_retries {
        True -> {
          io.println("🔄 Retrying subscription to: " <> config.subject)
          process.sleep(1000)
          start_subscription_with_retry(conn, config, retry_count + 1)
        }
        False -> {
          io.println("💀 Max retries exceeded for: " <> config.subject)
        }
      }
    }
  }
}

fn enhanced_listen_loop(
  subject: process.Subject(glats.SubscriptionMessage),
  config: SubscriptionConfig,
  error_count: Int,
) -> Nil {
  case config.timeout_ms {
    Some(timeout) -> {
      case process.receive(subject, timeout) {
        Ok(glats.ReceivedMessage(_conn, _sid, _subject, message)) -> {
          handle_message(subject, config, message)
        }
        Error(_) -> {
          let new_error_count = error_count + 1

          case new_error_count > 10 {
            True -> {
              let error_msg =
                "Too many consecutive timeouts for: " <> config.subject
              case config.on_error {
                Some(error_handler) -> error_handler(error_msg)
                None -> io.println("⚠️ " <> error_msg)
              }
              enhanced_listen_loop(subject, config, 0)
            }
            False -> {
              enhanced_listen_loop(subject, config, new_error_count)
            }
          }
        }
      }
    }
    None -> {
      let glats.ReceivedMessage(_conn, _sid, _subject, message) =
        process.receive_forever(subject)
      handle_message(subject, config, message)
    }
  }
}

fn handle_message(
  subject: process.Subject(glats.SubscriptionMessage),
  config: SubscriptionConfig,
  message: glats.Message,
) -> Nil {
  config.handler(message)
  enhanced_listen_loop(subject, config, 0)
}

pub fn quick_subscribe(
  conn: glats.Connection,
  subject: String,
  handler: fn(glats.Message) -> Nil,
) -> SubscriptionHandle {
  let config = new_subscription(subject, handler)
  let handles = start_managed_subscriptions(conn, [config])
  let assert [handle] = handles
  handle
}

pub fn close_subscription(handle: SubscriptionHandle) -> Nil {
  process.kill(handle.pid)
}
