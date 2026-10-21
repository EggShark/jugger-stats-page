const ALLOWED_CHARACTERS = ['-', '_', '+', '~', '.', '<', '>', '^', '%', ';'];

async function processForm(e: Event) {
	if (e.preventDefault) {
		e.preventDefault();
	}

	let form_data = new FormData(form);
	let username = form_data.get("username")! as string;
	let password = form_data.get("password")! as string;

	sendUserRequest(username, password);
  // You must return false to prevent the default form behavior
  return false;
}

async function sendUserRequest(username: string, password: string) {
	if (username.length > 32) {
		console.log("password too long should be <= 32 characters");
		return;
	}

	if (/^[\x00-\xFF]*$/.test(username)) {
		console.log("Username must only have ASCII characters");
		return;
	}

	for (let i = 0; i < username.length; i++) {
		const code = username.charCodeAt(i);
		if (!((code >= 48 && code <= 57) || (code >= 65 && code <= 90) || (code >= 97 && code <= 122)) && !ALLOWED_CHARACTERS.includes(username.charAt(i))) {
			console.log("username contains illegal character: ", username.charAt(i));
		}
	}

	if (password.length < 8 || password.length > 128) {
		console.log("password aint long enough pal");
		return;
	}

	const response = await fetch(
		"/create_user",
		{
			method: "POST",
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ username: username, password: password, id: 123 }),
		}
	);

	const content = await response.json();
	if (content.Token != null) {
		document.cookie = "session=" + content.Token;
	}
	// handle other cases yk
}

// not real constant cause we can't have anything nice in this world
// let in rust killing everyone forever ty peter theil
const form = document.getElementById("usercreation")! as HTMLFormElement;
form.addEventListener("submit", processForm);