async function processForm(e: Event) {
	if (e.preventDefault) {
		e.preventDefault();
	}

	let form_data = new FormData(form);
	let username = form_data.get("username")! as string;
	let password = form_data.get("password")! as string;

	console.log(username);
	console.log(password);

	sendUserRequest(username, password);


    // You must return false to prevent the default form behavior
    return false;
}

async function sendUserRequest(username: string, password: string) {
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
	// dont worry about it kitten :)
	console.log(content);
}

// not real constant cause we can't have anything nice in this world
// let in rust killing everyone forever ty peter theil
const form = document.getElementById("usercreation")! as HTMLFormElement;
form.addEventListener("submit", processForm);