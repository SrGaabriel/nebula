import {apiRequest, type ApiResult, type RealmDto, type UserDto} from "$lib/api/api";

export interface UserStatus {
    realms: RealmDto[],
    self: UserDto
}

export async function fetchSelf(token: string): Promise<ApiResult<UserDto>> {
    return fetchStatus(token).then(res => res.map(status => status.self));
}

export async function fetchStatus(token: string): Promise<ApiResult<UserStatus>> {
    return apiRequest<UserStatus>("/users/@me/status", token)
}