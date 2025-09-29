import gleam/dict
import gleam/io
import gleam/string
import glats
import gleam/option.{type Option, None, Some}
import gleam/json
import gleam/dynamic/decode

pub fn authenticate_user(
    cableway: glats.Connection,
    token: String
) -> Option(AuthResponse) {
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
                    Ok(AuthResponse(_, user_id, dict)) -> Some(AuthResponse(True, user_id, dict))
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
    AuthResponse(success: Bool, user_id: Int, realm_perms: dict.Dict(String, Int))
}

fn decode_response(response: glats.Message) -> Result(AuthResponse, json.DecodeError) {
    let decoder = {
        use success <- decode.field("success", decode.bool)
        use user_id <- decode.field("user_id", decode.int)
        use realm_perms <- decode.field("realm_perms", decode.dict(decode.string, decode.int))
        decode.success(AuthResponse(success, user_id, realm_perms))
    }
    json.parse(response.body, decoder)
}

pub fn auth_request_decoder() -> decode.Decoder(String) {
    use token <- decode.field("token", decode.string)
    decode.success(token)
}