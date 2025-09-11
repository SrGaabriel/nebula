import gleam/io
import glats/handler
import gleam/erlang/process
import envoy
import glats
import gleam/result
import gleam/int
import handlers/ping.{ping_handler}

@external(erlang, "Elixir.Dotenv", "load")
pub fn load_dotenv() -> Nil

pub fn main() {
  load_dotenv()

  let cableway_host = "localhost"
  let cableway_port = envoy.get("CABLEWAY_PORT")
    |> result.unwrap("4222")
    |> int.parse()
    |> result.unwrap(4222)

  use conn <- result.try(glats.connect(cableway_host, cableway_port, []))
  io.println("Connected to Cableway at " <> cableway_host <> ":" <> int.to_string(cableway_port))
  let assert Ok(_actor) =
    handler.handle_request(conn.data, [], "internal.ping", [], ping_handler)

  process.sleep_forever()

  Ok(Nil)
}