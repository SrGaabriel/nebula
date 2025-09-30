import glats
import gleam/dict
import gleam/dynamic/decode
import gleam/erlang/process
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import ws/app
import ws/manager

pub fn realm_subscription_decoder() -> decode.Decoder(Int) {
  use realm_id <- decode.field("realm_id", decode.int)
  decode.success(realm_id)
}

pub fn subscribe(
  state: app.NebulaState,
  realm_id: Int,
) -> List(app.WsSubscription) {
  case dict.get(state.allowed_topics, int.to_string(realm_id)) {
    Ok(topics) -> {
      topics
      |> list.map(fn (topic) {
        app.RealmSubscription(
          realm_id,
          manager.quick_subscribe(state.cableway, topic, handle_realm_event(
            state,
            _,
          )),
        )
      })
    }
    Error(_) -> {
      []
    }
  }
}

pub fn unsubscribe(state: app.NebulaState, realm_id: Int) -> Nil {
  state.subscriptions
  |> list.filter(fn(sub) { sub.realm_id == realm_id })
  |> list.each(fn(sub) { manager.close_subscription(sub.handle) })
}

pub fn handle_realm_event(state: app.NebulaState, message: glats.Message) -> Nil {
  io.println(
    "User: "
    <> int.to_string(state.user_id |> option.unwrap(-1))
    <> " got a realm event: "
    <> message.body,
  )

  process.send(state.socket_pid, app.SendEvent(message.body))
  Nil
}
