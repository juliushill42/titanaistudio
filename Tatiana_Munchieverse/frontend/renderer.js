import { assetLoader } from './assets.js';

class TatianaParticle {
    constructor(x, y, isBurst = false) {
        this.position = { x, y };
        const angle = Math.random() * Math.PI * 2;
        const speed = isBurst ? (1 + Math.random() * 2) : (Math.random() * 1.5);
        this.velocity = { x: Math.cos(angle) * speed, y: Math.sin(angle) * speed };
        this.lifetime = isBurst ? (40 + Math.random() * 40) : (80 + Math.random() * 50);
        this.size = isBurst ? (2 + Math.random() * 2) : (1 + Math.random() * 2);
        this.color = isBurst ? `rgba(255, 215, 0, ${0.5 + Math.random() * 0.5})` : `rgba(0, 255, 65, ${Math.random() * 0.5})`; // Gold for burst
        this.drag = 0.96;
    }

    update() {
        this.position.x += this.velocity.x;
        this.position.y += this.velocity.y;
        this.velocity.x *= this.drag;
        this.velocity.y *= this.drag;
        this.lifetime--;
    }

    draw(ctx) {
        ctx.fillStyle = this.color;
        ctx.beginPath();
        ctx.arc(this.position.x, this.position.y, this.size, 0, Math.PI * 2);
        ctx.fill();
    }
}

class TatianaParticleEmitter {
    constructor() {
        this.particles = [];
    }

    emit(x, y) {
        for (let i = 0; i < 3; i++) {
            this.particles.push(new TatianaParticle(x, y, false));
        }
    }

    createBurst(x, y) {
        for (let i = 0; i < 60; i++) {
            this.particles.push(new TatianaParticle(x, y, true));
        }
    }

    updateAndDraw(ctx) {
        for (let i = this.particles.length - 1; i >= 0; i--) {
            const p = this.particles[i];
            p.update();
            p.draw(ctx);
            if (p.lifetime <= 0) {
                this.particles.splice(i, 1);
            }
        }
    }
}

export default class TatianaRenderer {
    constructor(ctx, canvas) {
        this.ctx = ctx;
        this.canvas = canvas;
        this.particleEmitter = new TatianaParticleEmitter();
    }

    clear() {
        this.ctx.fillStyle = '#0a0a0a';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }
    
    drawEffects() {
        this.particleEmitter.updateAndDraw(this.ctx);
    }

    createBurst(x, y) {
        this.particleEmitter.createBurst(x, y);
    }

    drawPlayer(x, y) {
        this.particleEmitter.emit(x, y); // Player trail
        const sprite = assetLoader.get('player');
        if (sprite) {
            this.ctx.drawImage(sprite, x - sprite.width / 2, y - sprite.height / 2);
        }
    }

    drawZones(engine) {
        const zoneCount = engine.get_zone_count();
        this.ctx.font = "16px 'Courier New'";
        for (let i = 0; i < zoneCount; i++) {
            const zx = engine.get_zone_data_x(i);
            const zy = engine.get_zone_data_y(i);
            const zw = engine.get_zone_data_w(i);
            const zh = engine.get_zone_data_h(i);
            
            this.ctx.fillStyle = 'rgba(0, 100, 255, 0.3)';
            this.ctx.fillRect(zx, zy, zw, zh);
            
            this.ctx.fillStyle = '#fff';
            this.ctx.textAlign = 'center';
            this.ctx.fillText('Corner Store', zx + zw / 2, zy + zh / 2);
        }
    }

    drawNpcs(engine) {
        const npcCount = engine.get_npc_count();
        this.ctx.font = "16px 'Courier New'";
        for (let i = 0; i < npcCount; i++) {
            const nx = engine.get_npc_pos_x(i);
            const ny = engine.get_npc_pos_y(i);
            const name = engine.get_npc_name(i);

            let spriteKey = 'npc_pops';
            if (name === 'Day-Day') {
                spriteKey = 'npc_dayday';
            }
            
            const sprite = assetLoader.get(spriteKey);
            if (sprite) {
                this.ctx.drawImage(sprite, nx - sprite.width / 2, ny - sprite.height / 2);
            }

            this.ctx.fillStyle = '#fff';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(name, nx, ny - 25);
        }
    }

    drawEnvironment(engine) {
        const wallCount = engine.get_wall_count();
        this.ctx.fillStyle = '#4a4a4a';
        for (let i = 0; i < wallCount; i++) {
            const wx = engine.get_wall_data_x(i);
            const wy = engine.get_wall_data_y(i);
            const ww = engine.get_wall_data_w(i);
            const wh = engine.get_wall_data_h(i);
            this.ctx.fillRect(wx, wy, ww, wh);
        }
    }

    drawWorldItems(engine) {
        const itemCount = engine.get_world_item_count();
        this.ctx.font = "12px 'Courier New'";
        for (let i = 0; i < itemCount; i++) {
            const ix = engine.get_world_item_pos_x(i);
            const iy = engine.get_world_item_pos_y(i);
            const name = engine.get_world_item_name(i);

            const sprite = assetLoader.get('item_bag');
             if (sprite) {
                this.ctx.drawImage(sprite, ix - sprite.width / 2, iy - sprite.height / 2);
            }

            this.ctx.fillStyle = '#fff';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(name, ix, iy + 20);
        }
    }
}
