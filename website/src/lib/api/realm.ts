import {apiRequest, type ApiResult, type RealmDto} from "$lib/api/api";

export interface RealmCreationForm {
		name: string,
		description: string
}

interface RealmObject {
		realm: RealmDto
}

export async function createRealm(token: string, form: RealmCreationForm): Promise<ApiResult<RealmDto>> {
	return apiRequest<RealmObject>("/realms", token, {
		method: "POST",
		body: JSON.stringify(form),
		headers: {
			"Content-Type": "application/json"
		}
	}).then(res => res.map(obj => obj.realm));
}

export async function fetchRealm(token: string, realmId: string): Promise<ApiResult<RealmDto>> {
	return apiRequest<RealmObject>(`/realms/${realmId}`, token).
		then(res => res.map(obj => obj.realm));
}