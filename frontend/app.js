let images = {};
let apibase = "http://petite-expedition.com/";

async function load() {
	let i = ["squib", "squibright", "brrkah", "brrkahright", "schmeat", "schmeatempty", "trainfull", "trainempty", "grave", "graveright", "egg"];
	let p = [];
	i.forEach(function(s) {
			p.push(new Promise(resolve => {
				images[s] = new Image();
				images[s].onload = resolve;
				images[s].src = "images/" + s + ".png";
			}));
	});
	if (id === null || !(await fetch(apibase + "/api/v1/pet/" + id)).ok) {
		id = (await (await fetch(apibase + "/api/v1/pet", {method: "POST"})).json()).id;
	}
	if (id !== null) {
		stor.setItem("pet", id)
	} else {
		console.log("FAILED TO RETRIEVE PET LOCALLY OR FROM SERVER");
	}
	return Promise.all(p);
};

let pet = {
	"name": "Cornelius",
	"food": 5,
	"training": 5,
	"age": 0,
	"right": false,
	"x": 0,
	dead: false
};

let stor = null;
let laf = null;

function roll() {
	let r = Math.floor(Math.random() * 100);
	return r;
}

function flip() {
		pet.right = !pet.right;
		document.getElementById("pet").src = pet.right ? images.brrkahright.src : images.brrkah.src;
}

function drawStatus() {
	let i=0;
	document.querySelectorAll(".meat").forEach((e) =>
		e.src = i++ < pet.food ? images.schmeat.src : images.schmeatempty.src
	);
	i = 0;
	document.querySelectorAll(".train").forEach((e) =>
		e.src = i++ < pet.training ? images.trainfull.src : images.trainempty.src
	);
}

function animTick(ts) {
	let df = pet.right ? 1 : -1;
	if (!pet.dead) {
		if (ts - laf > 100) {
			laf = ts;
			let w = document.getElementById("field");
			let p = document.getElementById("pet");
			if (!pet.egg) {
				if (!pet.right && pet.x <= 0 || pet.right && pet.x >= field.clientWidth - 51 || roll() < 3) {
					flip();
					df = pet.right ? 1 : -1;
				}
				if (roll() < 30) {
					pet.x += 5 * df;
				}
			}
			p.style.left = pet.x + "px";
		}
		window.requestAnimationFrame(animTick);
	}
}

async function update_overlap(o1, o2) {
	for (const [k, v] of Object.entries(o1)) {
		o2[k] = v;
	}
}

async function simTick() {
	let r = await fetch(apibase + "/api/v1/pet/" + id);
	update_overlap(await r.json(), pet);

	/*if (pet.dead || pet.food <= -5 || pet.train <= -5) {
		pet.dead = true;
		document.getElementById("pet").src = pet.left ? images.grave.src : images.graveright.src;
		return;
	}
	if (roll() < 5) {
		pet.food--;
	}

	if (roll() < 8) {
		pet.train--;
	}*/
	drawStatus();
}

function clamp(min, mid, max) {
	return Math.max(min, Math.min(mid, max));
}

function keyDown(e) {
	switch (e.key) {
		case 'SoftLeft':
		if (!pet.dead) { pet.food = clamp(1, pet.food + 1, 5); }
		break;

		case 'SoftRight':
		if (!pet.dead) { pet.training = clamp(1, pet.training + 1, 5);}
		break;
	}
}

async function sendFeed() {
	return fetch(apibase + "/api/v1/pet/" + id + "/feed", {method: "POST"});
}

async function sendTrain() {
	return fetch(apibase + "/api/v1/pet/" + id + "/train", {method: "POST"});
}

window.addEventListener("load", function() {
	stor = window.localStorage;
	id = stor.getItem("pet");
	load().then(async _ => {
		laf = document.timeline.currentTime;
		simTick();
		pet.x = Math.random() * (document.getElementById("field").offsetWidth - 50); 
		document.getElementById("softkey-left").onclick = async () => {
			let n = await sendFeed();
			update_overlap(await n.json(), pet);
			drawStatus();
		};
		document.getElementById("softkey-right").onclick = async () => {
			let n = await sendTrain();
			update_overlap(await n.json(), pet);
			drawStatus();
		};
		document.addEventListener('keydown', keyDown);
		document.getElementById('pet').src = pet.egg ? images.egg.src : images.brrkah.src;

		window.requestAnimationFrame(animTick);
		setInterval(simTick, 60 * 1000);
	});
});
