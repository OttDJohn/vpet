let images = {};

async function load() {
	let i = ["cornelius", "corneliusright", "meatfull", "meatempty", "trainfull", "trainempty", "grave", "graveright"];
	let p = [];
	i.forEach(function(s) {
			p.push(new Promise(resolve => {
				images[s] = new Image();
				images[s].onload = resolve;
				images[s].src = "images/" + s + ".png";
			}));
	});
	return Promise.all(p);
};

let pet = {
	"name": "Cornelius",
	"food": 5,
	"train": 5,
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
		document.getElementById("pet").src = pet.right ? images.corneliusright.src : images.cornelius.src;
}

function drawStatus() {
	let i=0;
	document.querySelectorAll(".meat").forEach((e) =>
		e.src = i++ < pet.food ? images.meatfull.src : images.meatempty.src
	);
	i = 0;
	document.querySelectorAll(".train").forEach((e) =>
		e.src = i++ < pet.train ? images.trainfull.src : images.trainempty.src
	);
}

function animTick(ts) {
	let df = pet.right ? 1 : -1;
	if (!pet.dead) {
		if (ts - laf > 100) {
			laf = ts;
			let w = document.getElementById("field");
			let p = document.getElementById("pet");
			if (!pet.right && pet.x <= 0 || pet.right && pet.x >= field.clientWidth - 51 || roll() < 3) {
				flip();
				df = pet.right ? 1 : -1;
			}
			if (roll() < 30) {
				pet.x += 5 * df;
				p.style.left = pet.x + "px";
			}
		}
		window.requestAnimationFrame(animTick);
	}
}

function simTick() {
	if (pet.dead || pet.food <= -5 || pet.train <= -5) {
		pet.dead = true;
		document.getElementById("pet").src = pet.left ? images.grave.src : images.graveright.src;
		return;
	}
	if (roll() < 5) {
		pet.food--;
	}

	if (roll() < 8) {
		pet.train--;
	}
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
		if (!pet.dead) { pet.train = clamp(1, pet.train + 1, 5);}
		break;
	}
}


window.addEventListener("load", function() {
	stor = window.localStorage;
	let save = stor.getItem("pet");
	if (save !== null) {
		pet = save;
	}
	load().then(_ => {
		laf = document.timeline.currentTime;
		document.getElementById("softkey-left").onclick = () => { pet.food = clamp(1, pet.food + 1, 5)};
		document.getElementById("softkey-right").onclick = () => { pet.train = clamp(1, pet.train + 1, 5)};
		document.addEventListener('keydown', keyDown);
		window.requestAnimationFrame(animTick);
		setInterval(simTick, 60 * 10);
	});
});
