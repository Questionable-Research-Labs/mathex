import type { RequestHandler } from '@sveltejs/kit';
import { PrismaClient } from '@prisma/client';

export const POST: RequestHandler = async (event) => {
	console.log(import.meta.env['VITE_PUBLIC_PASSWORD']);
	if (event.request.headers.get('Authorization') == import.meta.env['VITE_PUBLIC_PASSWORD']) {
		if (event.request.bodyUsed) {
			try {
				let body = await event.request.json();
			} catch (ex) {
				return {
					status: 400,
					body: {
						message: 'Invalid request, please try again'
					}
				};
			}
		}
	} else {
		return {
			status: 401,
			body: {
				message: 'You do not have premission to preform this task'
			}
		};
	}

	return {
		status: 500,
		body: {
			message: 'Internal Server Error'
		}
	};
};
