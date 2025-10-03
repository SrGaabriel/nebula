// @ts-ignore
import JSONbig from "json-bigint";
import {API, apiError, type ApiResult, apiSuccess, unauthorizedApiRequest, type UserDto} from "$lib/api/api";

export interface LoginResponse {
    token: string;
    user: UserDto
}

export async function login(email: string, password: string): Promise<ApiResult<LoginResponse>> {
    return unauthorizedApiRequest(`/login`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email, password })
    })
}