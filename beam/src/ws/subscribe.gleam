import gleam/int
import ws/realm/events
import ws/manager
import ws/app.{type NebulaState}
import gleam/dynamic/decode
import gleam/dict

pub fn realm_subscription_decoder() -> decode.Decoder(Int) {
    use realm_id <- decode.field("realm_id", decode.int)
    decode.success(realm_id)
}

pub fn resubscribe(state: NebulaState, realm_id: Int) -> List(manager.SubscriptionHandle) {
    case dict.get(state.realm_perms, int.to_string(realm_id)) {
        Ok(_perm) -> {
            // todo: check if perm is enough for each subscription type
            let r = "realms." <> int.to_string(realm_id)
            [
                manager.quick_subscribe(
                    state.cableway,
                    r <> ".calendar.event_created", 
                    events.handle_event_created(state, _),
                )
            ]
        }
        Error(_) -> {
            []
        }
    }
}