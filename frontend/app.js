let images = {};

async function load() {
	let i = ["cornelius", "meatfull", "meatempty", "barbelfull", "barbelempty", "grave"];
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
let w = null;
let c = null;
let laf = null;

function roll() {
	let r = Math.floor(Math.random() * 100);
	return r;
}

function flip() {
		pet.right = !pet.right;
}

function drawBg() {
	// Sky
	let g = c.createRadialGradient(160, 212, 16, 160, 212, 140);
	g.addColorStop(0, "orange");
	g.addColorStop(0.3, "darkorange");
	g.addColorStop(1, "purple");
	c.fillStyle = g;
	c.fillRect(0, 0, 240, 212);
	// Ground
	g = c.createLinearGradient(120, 212, 120, 340);
	g.addColorStop(0, "darkgreen");
	g.addColorStop(1, "lightgreen");
	c.fillStyle = g;
	c.fillRect(0, 212, 240, 108);
}

function drawStatus() {
	let xbase = 3;
	for (let i = 0; i < 5; i++) {
		c.drawImage(i < pet.food ? images.meatfull : images.meatempty, xbase + i * 25, 3);
		c.drawImage(i < pet.train ? images.barbelfull : images.barbelempty, xbase + i * 25, 30);
	}
}

function animTick(ts) {
	c.clearRect(0, 0, w.width, w.height);
	let df = pet.right ? 1 : -1;
	if (!pet.dead && ts - laf > 100) {
		laf = ts;
		
		if (!pet.right && pet.x <= 0 || pet.right && pet.x >= w.clientWidth - 51 || roll() < 3) {
			flip();
			df = pet.right ? 1 : -1;
		}
		if (roll() < 30) {
			pet.x += 5 * df;
		}
	}
	drawBg();
	drawStatus();
	c.save();
	c.scale(df, 1);
	c.drawImage(pet.i, 0, 0, 48, 49, df * pet.x, w.height - 48, 48 * df, 49);
	c.restore();
	window.requestAnimationFrame(animTick);
}

function simTick() {
	if (pet.dead || pet.food <= -5 || pet.train <= -5) {
		pet.dead = true;
		pet.i = images.grave;
		return;
	}
	if (roll() < 5) {
		pet.food--;
	}

	if (roll() < 8) {
		pet.train--;
	}
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
	w = document.getElementById("game");
	c = w.getContext("2d");
	let save = stor.getItem("pet");
	if (save !== null) {
		pet = save;
	}
	setInterval(simTick, 60 * 10);
	load().then(_ => {
		pet.i = images.cornelius;
		laf = document.timeline.currentTime;
		document.getElementById("softkey-left").onclick = () => { pet.food = clamp(1, pet.food + 1, 5)};
		document.getElementById("softkey-right").onclick = () => { pet.train = clamp(1, pet.train + 1, 5)};
		document.addEventListener('keydown', keyDown);
		window.requestAnimationFrame(animTick);
	});
});
