const pageURL = new URL(window.location.href);
const username = pageURL.pathname.slice(pageURL.pathname.lastIndexOf("/") + 1, pageURL.pathname.length);
let editing = false;

async function getBasicData() {
	// get username and bio
	const response = await fetch("/basic_data/" + username);

	if (response.status != 200) {
		console.log("Error: ", response.status);
		// make error page filling
		return;
	}

	const data = await response.json();

	const descripton = data.description as string;
	const is_owner = data.owner_of_page as boolean;

	console.log(descripton);
	console.log(is_owner);
	const name_area = document.getElementById("username") as HTMLTitleElement;
	const descripton_area = document.getElementById("description") as HTMLParagraphElement;

	name_area.innerText = username;
	if (descripton.length == 0) {
		descripton_area.innerText = "Nothing here yet!"
	} else {
		descripton_area.innerText = descripton;
	}

	if (is_owner) {
		const container = document.getElementById("user-box") as HTMLDivElement;
		const button = document.createElement("button") as HTMLButtonElement;
		button.id = "edit";
		button.innerText = "Click to Edit"
		button.onclick = editDescription;

		container.appendChild(button);
	}
}

async function getAdvancedData() {
	// more complicated queries from multiple parts
}

function editDescription() {

	const button = document.getElementById("edit") as HTMLButtonElement;
	const container = document.getElementById("user-box") as HTMLDivElement;
	const descripton_area = document.getElementById("description") as HTMLParagraphElement;

	button.innerText = "Submit";
	const current_text = descripton_area.innerText;
	const textBox = document.createElement("textarea") as HTMLTextAreaElement;
	textBox.innerText = current_text;
	textBox.id = descripton_area.id;

	container.replaceChild(textBox, descripton_area);
	button.onclick = submitEditRequest;


	console.log("edit");
}

function backToView() {
	const button = document.getElementById("edit") as HTMLButtonElement;
	const container = document.getElementById("user-box") as HTMLDivElement;
	const descripton_area = document.getElementById("description") as HTMLTextAreaElement;

	button.innerText = "Edit";
	button.onclick = editDescription;

	const new_desciption_area = document.createElement("p") as HTMLParagraphElement;
	new_desciption_area.innerText = descripton_area.value;
	new_desciption_area.id = descripton_area.id;

	container.replaceChild(new_desciption_area, descripton_area);
}

async function submitEditRequest() {
	console.log("lmao lol even");
	const textBox = document.getElementById("description") as HTMLTextAreaElement;
	const text = textBox.value;

	const res = await fetch("/change_user_description", {
		method: "POST",
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ text: text, username: username })
	});

	if (res.status != 200) {
		console.log("cope and seeth ", res.status);
		return;
	}
	backToView();
	// change elements back or refresh page idk :p
}



console.log("user pages!!!");
getBasicData();