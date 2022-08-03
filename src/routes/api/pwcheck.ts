import { passwordCleared } from '$lib/checks';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = ({ request }) => {
	return {
		status: passwordCleared(request) ? 200 : 401
	};
};
