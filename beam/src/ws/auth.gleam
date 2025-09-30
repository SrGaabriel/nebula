import ws/app
import glats
import gleam/dynamic/decode
import gleam/io
import gleam/json
import gleam/option.{type Option, None, Some}
import gleam/string

pub fn authenticate_user(
  cableway: glats.Connection,
  token: String,
) -> Option(AuthResponse) {
  case glats.request(cableway, "internal.validate_token", token, [], 5000) {
    Ok(response) -> {
      case decode_response(response) {
        Ok(resp) ->
          Some(resp)
        Error(err) -> {
          io.println("Error decoding auth response: " <> string.inspect(err))
          None
        }
      }
    }
    Error(err) -> {
      io.println("Error during authentication: " <> string.inspect(err))
      None
    }
  }
}

pub type AuthResponse {
  AuthSuccess(user_id: Int, allowed_topics: app.AllowedTopicsDict)
  AuthFailure
}

fn decode_response(
  response: glats.Message,
) -> Result(AuthResponse, json.DecodeError) {
  let decoder = {
    use success <- decode.field("success", decode.bool)
    case success {
        True -> {
            use user_id <- decode.field("user_id", decode.int)
            use allowed_topics <- decode.field(
                "allowed_topics",
                decode.dict(decode.string, decode.list(decode.string)),
            )
            decode.success(AuthSuccess(user_id, allowed_topics))
        }
        False -> {
            decode.success(AuthFailure)
        }
    }
  }
  json.parse(response.body, decoder)
}

pub fn auth_request_decoder() -> decode.Decoder(String) {
  use token <- decode.field("token", decode.string)
  decode.success(token)
}
