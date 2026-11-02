function addPlayer() {
	const formData = new FormData(form);

	const addToTeam = document.getElementById("add-to-team-check") as HTMLInputElement;
	const name = formData.get("form-player-name") as string;
	const addingToTeam = formData.get("form-add-to-team") as string;
	// do more stuff
}


function addPlayerEvent(e: Event) {
	if (e.preventDefault) {
		e.preventDefault();
	}

	addPlayer();

	return false;
}

const form = document.getElementById("add-player-form") as HTMLFormElement;
form.addEventListener("submit", addPlayerEvent);