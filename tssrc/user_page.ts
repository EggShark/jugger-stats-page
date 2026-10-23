const pageURL = new URL(window.location.href);
const username = pageURL.pathname.slice(pageURL.pathname.lastIndexOf("/") + 1, pageURL.pathname.length);

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
}

async function getAdvancedData() {
	// more complicated queries from multiple parts
}

console.log("user pages!!!");
getBasicData();