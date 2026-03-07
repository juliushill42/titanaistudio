// assets.js
// PIONEER ARCHITECTURE: Asset Management

// Using base64-encoded images to keep the project self-contained.
// These are simple, colored squares serving as placeholders.

const placeholder_player = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAADdJREFUOE9jZKAQMFKon2Fw4P9/oyxCRiE1DCgNo8hIqD7qg8qj/qg+6oPKo/6oPoqg5qYGALD3Ax4uKz3dAAAAAElFTkSuQmCC'; // Green
const placeholder_npc = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAADdJREFUOE9jZKASMFKon2Fw4P9/RyRCRiE1DCgNo8hIqD7qg8qj/qg+6oPKo/6oPoqg5qYGAKiXAx72o10gAAAAAElFTkSuQmCC'; // Orange
const placeholder_npc_dayday = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAADdJREFUOE9jZKAQMFTa/2dg8P//j6wERiE1DCgNo8hIqD7qg8qj/qg+6oPKo/6oPoqg5qYGADzDBR9FNdJzAAAAAElFTkSuQmCC'; // Blue
const placeholder_item = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAADdJREFUOE9jZKASYFSq/8/A4P9/RyRCRiE1DCgNo8hIqD7qg8qj/qg+6oPKo/6oPoqg5qYGAHw3Ax7j2VcrAAAAAElFTkSuQmCC'; // Yellow

const assets = {
    'player': { src: placeholder_player },
    'npc_pops': { src: placeholder_npc },
    'npc_dayday': { src: placeholder_npc_dayday },
    'item_bag': { src: placeholder_item },
};

export const assetLoader = {
    images: {},
    loadAll: async function() {
        const promises = [];
        for (const key in assets) {
            const asset = assets[key];
            const promise = new Promise((resolve) => {
                const img = new Image();
                img.src = asset.src;
                this.images[key] = img;
                img.onload = () => resolve();
            });
            promises.push(promise);
        }
        await Promise.all(promises);
        console.log("PIONEER: All assets loaded.");
    },
    get: function(key) {
        return this.images[key];
    }
};
