// if code != 200 then token is always empty
export interface SignInResponse {
	code: number;
	token?: string;
}

export async function signin(username: string, password: string): Promise<SignInResponse> {
	const response = await fetch(
		"/signin",
		{
			method: "POST",
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ username: username, password: password}),
		}
	);
	const status = response.status;
	if (status != 200) {
		return { code: status };
	}

	return {
		code: status,
		token: await response.text(),
	};
}