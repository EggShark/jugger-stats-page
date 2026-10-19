const pageURL = new URL(window.location.href);
const username = pageURL.pathname.slice(pageURL.pathname.lastIndexOf("/"), pageURL.pathname.length);

async function getBasicData() {
	// get username and bio
}

async function getAdvancedData() {
	// more complicated queries from multiple parts
}