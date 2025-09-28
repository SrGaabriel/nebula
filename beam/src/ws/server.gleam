import gleam/io
import ws/auth
import ws/manager
import ws/listeners/events
import glats
import gleam/option
import gleam/bytes_tree
import logging
import gleam/erlang/process
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import mist.{type Connection, type ResponseData, type WebsocketMessage, type WebsocketConnection}
import gleam/string
import gleam/list

pub fn start(cableway: glats.Connection) -> Nil {
    let not_found =
        response.new(404)
        |> response.set_body(mist.Bytes(bytes_tree.new()))
    let unauthorized =
        response.new(401)
        |> response.set_body(mist.Bytes(bytes_tree.from_string("Unauthorized")))

    let assert Ok(_) =
        fn(req: Request(Connection)) -> Response(ResponseData) {
            let client_info = mist.get_client_info(req.body)
            logging.log(
                logging.Info,
                "Got a request from: " <> string.inspect(client_info),
            )
            case extract_token(req) {
                option.None -> unauthorized
                option.Some(token) -> {
                    io.println("Extracted token: " <> token)
                    let user_id = auth.authenticate_user(cableway, token)
                    io.println("Authenticated user ID: " <> string.inspect(user_id))
                    case user_id {
                        option.None -> unauthorized
                        option.Some(user_id) -> 
                            case request.path_segments(req) {
                                ["ws"] -> {
                                    mist.websocket(
                                        request: req,
                                        on_init: on_init(cableway, user_id, _),
                                        on_close: on_close,
                                        handler: on_message,
                                    )
                                }
                                _ -> not_found
                            }
                    }
                }
            }
        }
        |> mist.new
        |> mist.bind("localhost")
        |> mist.with_ipv6
        |> mist.port(3030)
        |> mist.start

    process.sleep_forever()
}

fn extract_token(req: Request(Connection)) -> option.Option(String) {
    case list.key_find(req.headers, "sec-websocket-protocol") {
        Ok(value) -> {
            io.println("Found protocol header: " <> value)
            case list.last(string.split(value, ",")) {
                Ok(token) -> {
                    option.Some(string.trim(token))
                }
                Error(_) -> option.None
            }
        }
        Error(_) -> option.None
    }
}

pub type NebulaState {
    NebulaState(cableway: glats.Connection, user_id: Int, subscriptions: List(manager.SubscriptionHandle))
}

pub fn on_init(
    cableway: glats.Connection,
    user_id: Int,
    ws_conn: WebsocketConnection,
) -> #(NebulaState, option.Option(process.Selector(message))) {
    logging.log(
        logging.Info,
        "WebSocket connected!" <> string.inspect(ws_conn)
    )
    let handles =
        [] // todo: add another one to the top
        |> list.append([events.subscribe_events(cableway, user_id)])
        |> list.flatten

    #(NebulaState(cableway, user_id, handles), option.None)
}

pub fn on_close(state: NebulaState) -> Nil {
    list.each(state.subscriptions, fn(handle) {
        manager.close_subscription(handle)
    })
    logging.log(
        logging.Info,
        "WebSocket closed: " <> string.inspect(state)
    )
}

pub fn on_message(
    state: NebulaState,
    message: WebsocketMessage(message),
    _conn: WebsocketConnection
) -> mist.Next(NebulaState, a) {
    logging.log(
        logging.Info,
        "Got WS message: " <> string.inspect(message)
    )
    mist.continue(state)
}