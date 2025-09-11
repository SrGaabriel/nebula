import gleam/option
import gleam/io
import glats/handler

pub fn ping_handler(req: handler.Request, state) {
  io.println("Got pinged: " <> req.body)

  handler.Reply(
    handler.Response(
      headers: req.headers,
      reply_to: option.None,
      body: "Pong!",
    ),
    state,
  )
}