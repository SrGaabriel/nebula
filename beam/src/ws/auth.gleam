import gleam/io
import gleam/string
import glats
import gleam/option.{type Option, None, Some}
import gleam/json
import gleam/dynamic/decode

pub fn authenticate_user(
    cableway: glats.Connection,
    token: String
) -> Option(Int) {
    case glats.request(
        cableway,
        "internal.validate_token",
        token,
        [],
        5000
    ) {
        Ok(response) -> {
            io.println("Auth response: " <> response.body)
            case decode_response(response) {
                Ok(AuthResponse(True, user_id)) -> Some(user_id)
                _ -> None
            }
        }
        Error(err) -> {
            io.println("Error during authentication: " <> string.inspect(err))
            None
        }
    }
}

type AuthResponse {
    AuthResponse(success: Bool, user_id: Int)
}

fn decode_response(response: glats.Message) -> Result(AuthResponse, json.DecodeError) {
    let decoder = {
        use success <- decode.field("success", decode.bool)
        use user_id <- decode.field("user_id", decode.int)
        decode.success(AuthResponse(success, user_id))
    }
    json.parse(response.body, decoder)
}