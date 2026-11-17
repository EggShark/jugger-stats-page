const playerButton = document.getElementById("player-nav-button") as HTMLButtonElement;
const teamButton = document.getElementById("team-nav-button") as HTMLButtonElement;
const eventButton = document.getElementById("event-nav-button") as HTMLButtonElement;
const registrationButton = document.getElementById("registration-nav-button") as HTMLButtonElement;

const playerDiv = document.getElementById("player-data-div")! as HTMLDivElement;
const teamDiv = document.getElementById("team-data-div")! as HTMLDivElement;
const eventDiv = document.getElementById("event-data-div")! as HTMLDivElement;
const registrationDiv = document.getElementById("registration-div-data")! as HTMLDivElement;

// in order based on the page
const buttons = [playerButton, teamButton, eventButton, registrationButton] as const;
const divs = [playerDiv, teamDiv, eventDiv, registrationDiv];

for (const [i, button] of buttons.entries()) {
	button.onclick = (event: MouseEvent) => { toggleArea(i); }
}

function toggleArea(button_id: number) {
	console.log("you clicked: {button_id}")
	for (let i = 0; i < buttons.length; i++) {
		if (i == button_id) {
			buttons[i]?.classList.add("data-bar-button-selected");
			divs[i]?.classList.remove("hidden");
			continue;
		}

		divs[i]?.classList.add("hidden");
		buttons[i]?.classList.remove("data-bar-button-selected");
	}
}