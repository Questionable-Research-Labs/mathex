export function passwordCleared(request: Request) {
	console.log(import.meta.env.VITE_PASSWORD);
	return import.meta.env.VITE_PASSWORD === request.headers.get('Authorization');
}
