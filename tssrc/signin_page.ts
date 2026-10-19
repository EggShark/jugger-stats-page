import { signin, type SignInResponse } from "./signin.js";

async function processForm(e: Event) {
	if (e.preventDefault) {
		e.preventDefault();
	}

	let form_data = new FormData(form);
	let username = form_data.get("username")! as string;
	let password = form_data.get("password")! as string;

	console.log(username);
	console.log(password);

	const response = await signin(username, password);

	if (response.code != 200) {
		console.log("FREAK OUT CRAZY STYLE", response.code);
	} else {
		// should never be null when code = 200
		document.cookie = "session=" + response.token!;
	}

    // You must return false to prevent the default form behavior
    return false;
}

const form = document.getElementById("signin-form")! as HTMLFormElement;
form.addEventListener("submit", processForm);