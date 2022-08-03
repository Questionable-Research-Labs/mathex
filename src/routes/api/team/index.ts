import type { RequestHandler } from '@sveltejs/kit';
import { PrismaClient } from '@prisma/client';

export const GET: RequestHandler = async () => {
	const teams = await new PrismaClient().team.findMany();

	return {
		status: 200,
		teams
	};
};
