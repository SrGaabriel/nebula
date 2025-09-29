import glats
import gleam/dict
import gleam/dynamic/decode
import gleam/erlang/process
import gleam/int
import gleam/io
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
) -> List(manager.SubscriptionHandle) {
  case dict.get(state.realm_perms, int.to_string(realm_id)) {
    Ok(_perm) -> {
      // todo: check if perm is enough for each subscription type
      let r = "realm." <> int.to_string(realm_id) <> "."
      [
        manager.quick_subscribe(state.cableway, r <> "*", handle_realm_event(
          state,
          _,
        )),
      ]
    }
    Error(_) -> {
      []
    }
  }
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
