import type { RequestHandler } from '@sveltejs/kit';
import { PrismaClient } from '';

export const GET: RequestHandler = async (_) => {
	return {
		status: 200
	};
};
