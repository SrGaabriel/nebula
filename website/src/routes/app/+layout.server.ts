import {redirect} from "@sveltejs/kit";
import {fetchStatus} from "$lib/api/user";

export async function load({ cookies, url }) {
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

    return {status, token};
}