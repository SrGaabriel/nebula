import JSONbig from // @ts-ignore
"json-bigint";

// todo: move to env
export const SECURE = false;
export const PROTOCOL = SECURE ? "s" : "";
export const HOST = 'localhost';
export const API_PORT = 3000;
export const WS_PORT = 3030;
export const API = `http${PROTOCOL}://${HOST}:${API_PORT}/api`;
export const WEBSOCKET = `ws${PROTOCOL}://${WS_PORT}`;

export type Snowflake = bigint;

export interface UserDto {
    id: Snowflake,
    name: string
}

export interface RealmDto {
    id: Snowflake,
    name: string,
    description?: string,
    owner_id: Snowflake,
}

export class ApiResult<T> {
    data?: T;
    error?: number;

    constructor(data?: T, error?: number) {
        this.data = data;
        this.error = error;
    }

    isSuccess(): boolean {
        return this.error === undefined;
    }

    isError(): boolean {
        return this.error !== undefined;
    }

    fold<U>(onError: (error: number) => U, onSuccess: (data: T) => U): U {
        if (this.isSuccess() && this.data !== undefined) {
            return onSuccess(this.data);
        }
        if (this.isError() && this.error !== undefined) {
            return onError(this.error);
        }
        throw new Error('ApiResult is in an invalid state');
    }

    map<U>(fn: (data: T) => U): ApiResult<U> {
        if (this.isSuccess() && this.data !== undefined) {
            return new ApiResult<U>(fn(this.data), undefined);
        }
        return new ApiResult<U>(undefined, this.error);
    }

    unwrap(): T {
        if (this.isError() || this.data === undefined) {
            throw new Error(`Tried to unwrap an error: ${this.error}`);
        }
        return this.data;
    }
}

export function apiError<T>(error: number): ApiResult<T> {
    return new ApiResult<T>(undefined, error);
}

export function apiSuccess<T>(data: T): ApiResult<T> {
    return new ApiResult<T>(data, undefined);
}

export async function unauthorizedApiRequest<T>(
    endpoint: string,
    options?: RequestInit
): Promise<ApiResult<T>> {
    const response = await fetch(`${API}${endpoint}`, {
        headers: {
            'Content-Type': 'application/json',
        },
        ...options,
    })
    if (response.status > 299) {
        return apiError(response.status);
    }
    let text = await response.text();
    let parsed = JSONbig.parse(text);
    return apiSuccess(parsed);
}

export async function apiRequest<T>(
    endpoint: string,
    token: string,
    options?: RequestInit
): Promise<ApiResult<T>> {
    return unauthorizedApiRequest<T>(endpoint, {
        ...options,
        headers: {
            'Authorization': `Bearer ${token}`,
            ...options?.headers
        }
    });
}