import mist
import gleam/option
import ws/app
import glats
import gleam/int
import gleam/io
    
pub fn handle_event_created(state: app.NebulaState, message: glats.Message) -> Nil {
  io.println(
    "User: "
    <> int.to_string(state.user_id |> option.unwrap(-1))
    <> " got event created message: "
    <> message.body
  )
  let _ = mist.send_text_frame(state.socket, "Event created: " <> message.body)
  Nil
}