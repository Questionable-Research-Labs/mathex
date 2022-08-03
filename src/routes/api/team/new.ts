import { passwordCleared } from '$lib/checks';
import type { RequestHandler } from '@sveltejs/kit';
import { PrismaClient } from '@prisma/client';

export const POST: RequestHandler = async ({ request }) => {
	if (passwordCleared(request)) {
		return {
			status: 401
		};
	}

	const { name } = await request.json();

	if (typeof name === 'undefined') {
		return {
			status: 403,
			body: {
				message: 'Missing field',
				missing_fields: ['username']
			}
		};
	}

	const client = new PrismaClient();
	client.team.create({
		data: {
			name
		}
	});

	return {
		status: 200
	};
};
