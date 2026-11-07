interface AddPlayerReq {
	player_name: string,
	player_team_id?: number,
	player_jersey_num?: string,
	player_jersey_name?: string,
}

async function addPlayer() {
	const formData = new FormData(form);

	const addToTeam = document.getElementById("add-to-team-check") as HTMLInputElement;
	const name = formData.get("form-player-name") as string;
	const addingToTeam = formData.get("form-add-to-team") as string;

	if (name.length == 0) {
		addPlayerFormError(document.getElementById("form-player-name")!, "player name cannot be empty")
		return;
	}

	const reqBody: AddPlayerReq = {
		player_name: name,
	};
	// do more stuff
	if (addToTeam.checked) {
		const teamId = formData.get("form-team-name") as string;
		const jerseyNum = formData.get("form-jersey-number") as string;
		const jerseyName = formData.get("form-jersey-name") as string;

		if (!/^[0-9]+$/.test(jerseyNum) || jerseyNum.length > 4 || jerseyNum.length == 0) {
			console.log("DIE");
			addPlayerFormError(document.getElementById("form-jersey-number")!, "jersey number must but an 4 digit integer");
			return;
		}

		if (!/^[0-9]+$/.test(teamId)) {
			addPlayerFormError(document.getElementById("form-team-name")!, "team id must be a number");
			return;
		}

		if (jerseyName.length == 0) {
			addPlayerFormError(document.getElementById("form-jersey-name")!, "jersey name cannot be empty");
			return;
		}

		if (teamId.length == 0) {
			addPlayerFormError(document.getElementById("form-team-name")!, "team name cannot be empty");
			return;
		}

		reqBody.player_team_id = +teamId;
		reqBody.player_jersey_num = jerseyNum;
		reqBody.player_jersey_name = jerseyName;
	}

	const res = await fetch("/admin/add_player", {
		method: "POST",
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(reqBody),
	});

	if (res.status != 200) {
		console.log("FREAK THE OUT");
	}
}

function addPlayerFormError(element: HTMLElement, message: string) {
	// modify the things to display error
	element.classList.add("invalid-input");
	console.log(message);
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