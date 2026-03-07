import init, { TatianaMunchieverseEngine, TatianaInputState } from '../engine/pkg/tatiana_munchieverse_engine.js';
import TatianaRenderer from './renderer.js';
import { assetLoader } from './assets.js';

async function run() {
    await assetLoader.loadAll();
    await init();

    const canvas = document.getElementById('munchieverse-canvas');
    const ctx = canvas.getContext('2d');
    const questLogElement = document.getElementById('quest-log');
    const dialogueBoxElement = document.getElementById('dialogue-box');
    const inventoryUiElement = document.getElementById('inventory-ui');
    const notificationArea = document.getElementById('notification-area');

    // ...

    function showNotification(text) {
        notificationArea.innerText = text;
        notificationArea.style.opacity = 1;
        setTimeout(() => {
            notificationArea.style.opacity = 0;
        }, 2000); // Notification lasts 2 seconds
    }

    function drawDialogue() {
    // ...
    }

    let lastTime = 0;
    function gameLoop(currentTime) {
        // ...

        // --- Check for events ---
        const completed_quests = engine.drain_just_completed_quest_ids();
        if (completed_quests.length > 0) {
            const playerX = engine.get_player_pos_x();
            const playerY = engine.get_player_pos_y();
            renderer.createBurst(playerX, playerY);
            showNotification("QUEST COMPLETED");
        }

        // --- Drawing ---
        renderer.clear();
        renderer.drawEnvironment(engine);
        renderer.drawEffects();
        renderer.drawZones(engine);
        renderer.drawWorldItems(engine);
        renderer.drawNpcs(engine);
        
        const playerX = engine.get_player_pos_x();
        const playerY = engine.get_player_pos_y();
        renderer.drawPlayer(playerX, playerY);

        // ...
    }

    requestAnimationFrame(gameLoop);
}

run();
