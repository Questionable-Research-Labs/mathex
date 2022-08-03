const Kv = {
	get: async (key: string): Promise<string> => {
		const request = await fetch(import.meta.env.VITE_KV_URL + key, {
			headers: {
				Authorization: import.meta.env.VITE_CLOUDFLARE_KV_KEY
			}
		});
		if (request.status == 200) {
			return await request.text();
		} else if (request.status == 404) {
			throw new Error('No such key found');
		} else {
			throw new Error('Could no make a request to cloudflare API', await request.json());
		}
	},
	set: async (key: string, value: string): Promise<void> => {
		const request = await fetch(import.meta.env.VITE_KV_URL + key, {
			headers: {
				Authorization: import.meta.env.VITE_CLOUDFLARE_KV_KEY
			},
			body: value
		});
		if (request.status == 200) {
			return;
		} else {
			throw new Error('Could no make a request to cloudflare API', await request.json());
		}
	}
};

export default Kv;
