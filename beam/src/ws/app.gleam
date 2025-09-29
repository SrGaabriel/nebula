import mist
import ws/manager
import glats
import gleam/option
import gleam/dict

pub type NebulaState {
  NebulaState(
    socket: mist.WebsocketConnection,
    cableway: glats.Connection,
    user_id: option.Option(Int),
    subscriptions: List(manager.SubscriptionHandle),
    realm_perms: dict.Dict(String, Int),
  )
}