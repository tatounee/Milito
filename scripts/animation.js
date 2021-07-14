
const ANIMATION_SPEED_LASER = 40;
const ANIMATION_SPEED_PROJECTILE = 20;

let lasers = document.querySelectorAll(".laser-img");
let projectiles = document.querySelectorAll(".projectile");

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

(async () => {
    let i = 0;
    while (true) {
        i++;
        await sleep(ANIMATION_SPEED_LASER);
        let shift = i % 16 * 32;
        for (sprite of lasers) {
            sprite.style.backgroundPosition = `0 -${shift}px`;
        }
    }
})();

(
    async () => {
        let rotation = 0;
        while (true) {
            rotation += 10;
            rotation %= 360;
            await sleep(ANIMATION_SPEED_PROJECTILE);
            for (proj of projectiles) {
                proj.style.transform = `rotate(${rotation}deg)`
            }
        }
    }
)();