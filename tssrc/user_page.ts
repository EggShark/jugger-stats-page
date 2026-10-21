const pageURL = new URL(window.location.href);
const username = pageURL.pathname.slice(pageURL.pathname.lastIndexOf("/"), pageURL.pathname.length);

async function getBasicData() {
	// get username and bio
	const response = await fetch("/basic_data" + username);
	const data = await response.json();

	const descripton = data.descripton as String;
	const is_owner = data.owner_of_page as boolean;

	console.log(descripton);
	console.log(is_owner);

}

async function getAdvancedData() {
	// more complicated queries from multiple parts
}

console.log("user pages!!!");
getBasicData();