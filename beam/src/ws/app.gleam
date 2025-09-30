import glats
import gleam/dict
import gleam/erlang/process
import gleam/option
import mist
import ws/manager

pub type AllowedTopicsDict = dict.Dict(String, List(String))

pub type NebulaState {
  NebulaState(
    socket: mist.WebsocketConnection,
    socket_pid: process.Subject(WsActorMessage),
    cableway: glats.Connection,
    user_id: option.Option(Int),
    subscriptions: List(WsSubscription),
    allowed_topics: AllowedTopicsDict,
  )
}

pub type WsSubscription {
  RealmSubscription(realm_id: Int, handle: manager.SubscriptionHandle)
}

pub type WsActorMessage {
  SendEvent(String)
  AuthDeadline
}
