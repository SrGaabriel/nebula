import ws/realm
import glats
import gleam/bytes_tree
import gleam/dict
import gleam/dynamic/decode
import gleam/erlang/process
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import gleam/int
import gleam/io
import gleam/json
import gleam/list
import gleam/option
import gleam/string
import logging
import mist.{
  type Connection, type ResponseData, type WebsocketConnection,
  type WebsocketMessage,
}
import ws/app.{type NebulaState, NebulaState}
import ws/auth
import ws/manager

pub fn start(cableway: glats.Connection) -> Nil {
  let not_found =
    response.new(404)
    |> response.set_body(mist.Bytes(bytes_tree.new()))

  let assert Ok(_) =
    fn(req: Request(Connection)) -> Response(ResponseData) {
      let client_info = mist.get_client_info(req.body)
      logging.log(
        logging.Info,
        "Got a request from: " <> string.inspect(client_info),
      )
      case request.path_segments(req) {
        ["ws"] -> {
          mist.websocket(
            request: req,
            on_init: on_init(cableway, _),
            on_close: on_close,
            handler: on_message,
          )
        }
        _ -> not_found
      }
    }
    |> mist.new
    |> mist.bind("localhost")
    |> mist.with_ipv6
    |> mist.port(3030)
    |> mist.start

  process.sleep_forever()
}

pub fn on_init(
  cableway: glats.Connection,
  ws_conn: WebsocketConnection,
) -> #(NebulaState, option.Option(process.Selector(app.WsActorMessage))) {
  logging.log(logging.Info, "WebSocket connected!" <> string.inspect(ws_conn))

  let self_subject = process.new_subject()

  let selector =
    process.new_selector()
    |> process.select(self_subject)

  #(
    NebulaState(ws_conn, self_subject, cableway, option.None, [], dict.new()),
    option.Some(selector),
  )
}

pub fn on_close(state: NebulaState) -> Nil {
  list.each(state.subscriptions, fn(handle) {
    manager.close_subscription(handle)
  })
  logging.log(logging.Info, "WebSocket closed: " <> string.inspect(state))
}

pub fn on_message(
  state: NebulaState,
  message: WebsocketMessage(app.WsActorMessage),
  conn: WebsocketConnection,
) -> mist.Next(NebulaState, a) {
  logging.log(logging.Info, "Got WS message: " <> string.inspect(message))
  case message {
    mist.Text(text) -> {
      // todo: use binary instead (compression)
      let decoded = decode_message(text)
      case state.user_id {
        option.Some(_user_id) ->
          case decoded {
            Ok(Heartbeat) -> {
              mist.continue(state)
            }

            Ok(RealmSubscription(realm_id)) -> {
              let new_subscriptions = realm.subscribe(state, realm_id)
              let all_subscriptions =
                list.append(state.subscriptions, new_subscriptions)
              let new_state =
                NebulaState(
                  conn,
                  state.socket_pid,
                  state.cableway,
                  state.user_id,
                  all_subscriptions,
                  state.realm_perms,
                )
              mist.continue(new_state)
            }
            Error(err) -> {
              logging.log(
                logging.Warning,
                "Failed to decode WS message: " <> string.inspect(err),
              )
              mist.continue(state)
            }
            _ -> {
              logging.log(
                logging.Warning,
                "Received unknown WS message: " <> string.inspect(decoded),
              )
              mist.continue(state)
            }
          }
        option.None ->
          case decoded {
            Ok(AuthRequest(token)) -> {
              case auth.authenticate_user(state.cableway, token) {
                option.Some(auth.AuthResponse(_, user_id, realm_perms)) -> {
                  logging.log(
                    logging.Info,
                    "User authenticated: " <> int.to_string(user_id),
                  )
                  let new_state =
                    NebulaState(
                      conn,
                      state.socket_pid,
                      state.cableway,
                      option.Some(user_id),
                      [],
                      realm_perms,
                    )
                  mist.continue(new_state)
                }
                option.None -> {
                  logging.log(
                    logging.Warning,
                    "Authentication failed for token: " <> token,
                  )
                  mist.stop()
                }
              }
            }
            _ -> {
              logging.log(
                logging.Warning,
                "Received non-auth message before authentication",
              )
              mist.stop()
            }
          }
      }
    }
    mist.Closed -> {
      mist.stop()
    }
    mist.Shutdown -> {
      mist.stop()
    }
    mist.Custom(app.SendEvent(text)) -> {
        let _ = mist.send_text_frame(conn, text)
        mist.continue(state)
    }
    _ -> {
      io.println("Ignoring non-text WS message: " <> string.inspect(message))
      mist.continue(state)
    }
  }
}

pub type DecodedMessage {
  AuthRequest(token: String)
  RealmSubscription(realm_id: Int)
  UnsubscribeRealm(realm_id: Int)
  Heartbeat
}

fn decode_message(text: String) -> Result(DecodedMessage, json.DecodeError) {
  let decoder = {
    use code <- decode.field("code", decode.int)
    case code {
      3 -> {
        use auth_req <- decode.field("data", auth.auth_request_decoder())
        decode.success(AuthRequest(auth_req))
      }
      5 -> {
        use subscription <- decode.field(
          "data",
          realm.realm_subscription_decoder(),
        )
        decode.success(RealmSubscription(subscription))
      }
      _ -> decode.failure(Heartbeat, "Unknown message code")
    }
  }
  json.parse(text, decoder)
}
