import { passwordCleared } from '$lib/checks';
import Kv from '$lib/kv';
import type { RequestHandler } from '@sveltejs/kit';
import { PrismaClient } from '@prisma/client';

export const GET: RequestHandler = async () => {
	try {
		const val = await Kv.get('round');
		return {
			status: 200,
			body: val
		};
	} catch (ex) {
		if (ex instanceof Error) {
			return {
				status: 500,
				body: {
					message: ex.message
				}
			};
		}
	}

	return {
		status: 200
	};
};

export const POST: RequestHandler = async ({ request }) => {
	if (!passwordCleared(request)) {
		return {
			status: 401
		};
	}

	if (!request.bodyUsed) {
		return {
			status: 403,
			message: 'Please set the value for the body'
		};
	}

	const id = Number(await request.text());

	if (isNaN(id)) {
		return {
			status: 403,
			message: 'Body should be a number'
		};
	}

	const rounds = await new PrismaClient().round.count({ where: { id } });

	if (rounds == 0) {
		return {
			status: 403,
			message: 'The specified round does not exist'
		};
	}

	try {
		await Kv.set('round', '' + id);
	} catch (ex) {
		if (ex instanceof Error) {
			return {
				status: 500,
				body: {
					message: ex.message
				}
			};
		}
	}

	return {
		status: 200
	};
};
