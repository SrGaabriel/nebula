import { fetchRealm } from '$lib/api/realm';

export async function load({ parent, params }) {
		const { token }	= await parent();
		const realmId = params.realm_id;
		const realmResult = await fetchRealm(token, realmId);
		const realm = realmResult.fold(
			() => {
				throw new Error("Failed to fetch realm");
			},
			(realm) => {
				if (!realm) {
					throw new Error("Realm not found");
				}
				return realm;
			}
		);
		return { realm };
}