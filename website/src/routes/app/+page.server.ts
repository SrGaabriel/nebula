import {redirect} from "@sveltejs/kit";
import {fetchSelf, fetchStatus} from "$lib/api/user";

export async function load({ cookies, url, params }) {
    if (!cookies.get('token')) {
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
    const token = cookies.get('token');
    if (!token) {
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
    const self = await fetchStatus(token);
    const status = self.fold(
        () => {
            throw redirect(303, `/login?redirect=${url.pathname}`);
        }
    , (data) => data);
		const activeRealmId = url.searchParams.get('realm');
		let activeRealm = null;
		if (activeRealmId) {
			activeRealm = status.realms.find(r => r.id == BigInt(activeRealmId));
			console.log("Realms:", status.realms);
			console.log("Active realm from URL:", activeRealmId, activeRealm);
			if (!activeRealm) {
				throw redirect(303, `/app`);
			}
		}

    return {status, token, activeRealm};
}