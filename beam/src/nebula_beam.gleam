import envoy
import glats
import glats/handler
import gleam/int
import gleam/io
import gleam/result
import handlers/ping.{ping_handler}
import ws/manager as subscription_manager
import ws/server as ws

@external(erlang, "Elixir.Dotenv", "load")
pub fn load_dotenv() -> Nil

pub fn main() {
  load_dotenv()

  let cableway_host = "localhost"
  let cableway_port =
    envoy.get("CABLEWAY_PORT")
    |> result.unwrap("4222")
    |> int.parse()
    |> result.unwrap(4222)

  use conn <- result.try(glats.connect(cableway_host, cableway_port, []))
  io.println(
    "Connected to Cableway at "
    <> cableway_host
    <> ":"
    <> int.to_string(cableway_port),
  )

  let assert Ok(_actor) =
    handler.handle_request(conn.data, [], "internal.ping", [], ping_handler)

  subscription_manager.quick_subscribe(conn.data, "internal.status", handle_status_message)

  ws.start(conn.data)
  Ok(Nil)
}

fn handle_status_message(message: glats.Message) -> Nil {
  io.println("Got status: " <> message.body)
}