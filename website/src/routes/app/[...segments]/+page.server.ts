import { redirect } from '@sveltejs/kit';

export const load = ({ params }) => {
	const parts = params.segments.split('/');
	const query = new URLSearchParams();
	for (let i = 0; i < parts.length; i++) {
		switch (parts[i]) {
			case 'realms': {
				if (i + 1 < parts.length)
					query.set('realm', parts[i+1]);
			}
		}
	}

	throw redirect(302, `/app?${query}`);
};