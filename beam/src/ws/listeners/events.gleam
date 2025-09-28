import ws/manager
import glats
import gleam/int
import gleam/io
    
pub fn subscribe_events(
    cableway: glats.Connection,
    user_id: Int,
) -> List(manager.SubscriptionHandle) {
  io.println("Subscribing to events for user: " <> int.to_string(user_id))

  [
    manager.quick_subscribe(
        cableway,
        "realm.*.calendar.event_created",
        handle_event_created(user_id, _)
    )
  ]
}

pub fn handle_event_created(user_id: Int, message: glats.Message) -> Nil {
  io.println(
    "User: "
    <> int.to_string(user_id)
    <> " got event created message: "
    <> message.body
  )
}