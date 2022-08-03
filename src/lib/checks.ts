export function passwordCleared(request: Request) {
	console.log('password', request.headers);
	return import.meta.env.VITE_PUBLIC_PASSWORD === request.headers.get('Authorization');
}
