import type { RequestHandler } from '@sveltejs/kit';
import { passwordCleared } from '$lib/checks';

export const POST: RequestHandler = async (event) => {
	if (!passwordCleared(event.request)) {
		return {
			status: 401
		};
	}

	if (event.request.bodyUsed) {
		try {
			const body = await event.request.json();
		} catch (ex) {
			return {
				status: 400,
				body: {
					message: 'Invalid request, please try again'
				}
			};
		}
	}

	return {
		status: 500,
		body: {
			message: 'Internal Server Error'
		}
	};
};
