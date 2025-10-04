import { redirect } from '@sveltejs/kit';

export async function load({ params }) {
	throw redirect(302, '/app/realm/' + params.realm_id + '/overview');
}