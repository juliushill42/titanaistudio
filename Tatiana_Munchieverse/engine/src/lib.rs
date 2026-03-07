use wasm_bindgen::prelude::*;
use glam::{Vec2, Rect};
use mlua::{Lua, Result as LuaResult};

// --- Core Structs ---
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct TatianaItem {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct WorldItem {
    item: TatianaItem,
    position: Vec2,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct TatianaInputState {
    pub up: bool, pub down: bool, pub left: bool, pub right: bool, pub interact: bool,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub enum MissionState { Pending, InProgress, Completed, }

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TatianaQuest {
    id: String,
    title: String,
    state: MissionState,
    required_item_id: Option<String>,
}

#[wasm_bindgen]
pub struct TatianaZone {
    rect: Rect,
    quest_id: String,
}

#[wasm_bindgen]
pub struct TatianaNpc {
    name: String,
    position: Vec2,
    dialogue: Vec<String>,
}

struct ActiveDialogue {
    npc_index: usize,
    line_index: usize,
}

#[wasm_bindgen]
pub struct TatianaPlayer {
    position: Vec2,
    velocity: Vec2,
    size: Vec2,
    inventory: Vec<TatianaItem>,
}

#[wasm_bindgen]
pub struct TatianaMunchieverseEngine {
    player: TatianaPlayer,
    world_bounds: Vec2,
    quests: Vec<TatianaQuest>,
    zones: Vec<TatianaZone>,
    npcs: Vec<TatianaNpc>,
    walls: Vec<Rect>,
    world_items: Vec<WorldItem>,
    active_dialogue: Option<ActiveDialogue>,
            just_completed_quests: Vec<String>,
            lua: Lua,}

// --- Implementations ---
#[wasm_bindgen]
impl TatianaInputState {
    pub fn new() -> TatianaInputState {
        TatianaInputState { up: false, down: false, left: false, right: false, interact: false }
    }
}

#[wasm_bindgen]
impl TatianaMunchieverseEngine {
    pub fn new(width: f32, height: f32) -> TatianaMunchieverseEngine {
        let mut engine = TatianaMunchieverseEngine {
            player: TatianaPlayer {
                position: Vec2::new(width / 2.0 + 50.0, height / 2.0),
                velocity: Vec2::ZERO,
                size: Vec2::new(20.0, 20.0),
                inventory: Vec::new(),
            },
            world_bounds: Vec2::new(width, height),
            quests: Vec::new(),
            zones: Vec::new(),
            npcs: Vec::new(),
            walls: Vec::new(),
            world_items: Vec::new(),
            active_dialogue: None,
            lua: Lua::new(),
        };
        engine.load_initial_quests();
        engine.load_initial_zones();
        engine.load_initial_npcs();
        engine.load_level();
        engine.load_initial_items();
        engine
    }

    fn load_initial_quests(&mut self) {
        self.quests.push(TatianaQuest {
            id: "MQ01".to_string(),
            title: "The Corner Store Run".to_string(),
            state: MissionState::InProgress,
            required_item_id: None,
        });
        self.quests.push(TatianaQuest {
            id: "SQ01".to_string(),
            title: "Find Day-Day's Lost Bag".to_string(),
            state: MissionState::Pending,
            required_item_id: Some("BAG_01".to_string()),
        });
    }
    
    fn load_initial_items(&mut self) {
        self.world_items.push(WorldItem {
            item: TatianaItem { id: "BAG_01".to_string(), name: "Day-Day's Bag".to_string() },
            position: Vec2::new(150.0, 300.0),
        });
    }
    
    fn load_initial_zones(&mut self) {
        self.zones.push(TatianaZone {
            rect: Rect::from_corners(Vec2::new(self.world_bounds.x - 150.0, 50.0), Vec2::new(self.world_bounds.x-10.0, 150.0)),
            quest_id: "MQ01".to_string(),
        });
    }

    fn load_initial_npcs(&mut self) {
        self.npcs.push(TatianaNpc {
            name: "Pops".to_string(),
            position: Vec2::new(self.world_bounds.x / 2.0, self.world_bounds.y / 2.0 - 50.0),
            dialogue: vec![
                "Hey son, I need you to run to the store for me.".to_string(),
                "We're out of milk for the cereal. Don't take all day!".to_string(),
            ],
        });
        self.npcs.push(TatianaNpc {
            name: "Day-Day".to_string(),
            position: Vec2::new(180.0, 250.0),
            dialogue: vec![
                "Man, I can't believe it... I lost my bag!".to_string(),
                "You find it for me, I'll make it worth your while.".to_string(),
                "Yo, you found it! You're a real one.".to_string(),
            ],
        });
    }

    fn load_level(&mut self) {
        // Outer boundary walls
        self.walls.push(Rect::new(0.0, 0.0, self.world_bounds.x, 10.0));
        self.walls.push(Rect::new(0.0, self.world_bounds.y - 10.0, self.world_bounds.x, 10.0));
        self.walls.push(Rect::new(0.0, 0.0, 10.0, self.world_bounds.y));
        self.walls.push(Rect::new(self.world_bounds.x - 10.0, 0.0, 10.0, self.world_bounds.y));

        // Pops' House area
        self.walls.push(Rect::new(self.world_bounds.x / 2.0 - 200.0, self.world_bounds.y / 2.0 - 150.0, 400.0, 10.0));
        self.walls.push(Rect::new(self.world_bounds.x / 2.0 - 200.0, self.world_bounds.y / 2.0 + 150.0, 400.0, 10.0));
        self.walls.push(Rect::new(self.world_bounds.x / 2.0 - 200.0, self.world_bounds.y / 2.0 - 150.0, 10.0, 310.0));

        // Day-Day's area
        self.walls.push(Rect::new(80.0, 150.0, 10.0, 200.0));
        self.walls.push(Rect::new(220.0, 150.0, 10.0, 200.0));
    }

    pub fn tick(&mut self, input: &TatianaInputState, dt: f32) {
        if self.active_dialogue.is_some() {
            if input.interact {
                let mut end_dialogue = false;
                if let Some(dialogue) = &mut self.active_dialogue {
                    let npc = &self.npcs[dialogue.npc_index];
                    let quest_complete = self.quests.iter().any(|q| q.id == "SQ01" && q.state == MissionState::Completed);

                    let line_index = if quest_complete && npc.name == "Day-Day" {
                        2
                    } else {
                        dialogue.line_index
                    };

                    if line_index + 1 < npc.dialogue.len() && !(quest_complete && npc.name == "Day-Day") {
                        dialogue.line_index += 1;
                    } else {
                        end_dialogue = true;
                    }
                }
                if end_dialogue { self.active_dialogue = None; }
            }
            return;
        }

        const ACCELERATION: f32 = 1200.0;
        const DAMPING: f32 = 0.90;
        let mut direction = Vec2::ZERO;
        if input.up { direction.y -= 1.0; }
        if input.down { direction.y += 1.0; }
        if input.left { direction.x -= 1.0; }
        if input.right { direction.x += 1.0; }

        if direction.length_squared() > 0.0 {
            self.player.velocity += direction.normalize() * ACCELERATION * dt;
        }

        // --- Player Movement & Collision ---
        let proposed_delta = self.player.velocity * dt;

        // Move on X axis
        self.player.position.x += proposed_delta.x;
        let mut player_rect = Rect::from_center_size(self.player.position, self.player.size);

        // Check for X-axis collisions
        for wall in &self.walls {
            if player_rect.intersect(*wall).is_some() {
                self.player.position.x -= proposed_delta.x; // Revert X movement
                self.player.velocity.x = 0.0;
                break;
            }
        }

        // Move on Y axis
        self.player.position.y += proposed_delta.y;
        player_rect = Rect::from_center_size(self.player.position, self.player.size);

        // Check for Y-axis collisions
        for wall in &self.walls {
            if player_rect.intersect(*wall).is_some() {
                self.player.position.y -= proposed_delta.y; // Revert Y movement
                self.player.velocity.y = 0.0;
                break;
            }
        }

        self.player.velocity *= DAMPING;
        
        // --- Interaction Logic ---

        if input.interact {
            const INTERACTION_RADIUS: f32 = 50.0;
            let mut interacted = false;

            for (i, npc) in self.npcs.iter().enumerate() {
                if self.player.position.distance(npc.position) < INTERACTION_RADIUS {
                    if npc.name == "Day-Day" {
                        if let Some(quest) = self.quests.iter_mut().find(|q| q.id == "SQ01") {
                            if quest.state != MissionState::Completed && self.player.inventory.iter().any(|item| item.id == "BAG_01") {
                                quest.state = MissionState::Completed;
                            }
                        }
                    }
                    self.active_dialogue = Some(ActiveDialogue { npc_index: i, line_index: 0 });
                    interacted = true;
                    break;
                }
            }

            if !interacted {
                let mut picked_up_index: Option<usize> = None;
                for (i, world_item) in self.world_items.iter().enumerate() {
                    if self.player.position.distance(world_item.position) < INTERACTION_RADIUS {
                        picked_up_index = Some(i);
                        break;
                    }
                }
                if let Some(index) = picked_up_index {
                    let item = self.world_items.remove(index);
                    self.player.inventory.push(item.item);
                }
            }
        }

        for zone in &self.zones {
            if zone.rect.contains(self.player.position) {
                if let Some(quest) = self.quests.iter_mut().find(|q| q.id == zone.quest_id) {
                    if quest.state != MissionState::Completed {
                        quest.state = MissionState::Completed;
                    }
                }
            }
        }
    }

    // --- Accessors ---
    pub fn get_player_pos_x(&self) -> f32 { self.player.position.x }
    pub fn get_player_pos_y(&self) -> f32 { self.player.position.y }
    pub fn get_quest_count(&self) -> usize { self.quests.len() }
    pub fn get_quest_title(&self, index: usize) -> Option<String> { self.quests.get(index).map(|q| q.title.clone()) }
    pub fn get_quest_state(&self, index: usize) -> Option<String> {
        self.quests.get(index).map(|q| match q.state {
            MissionState::Pending => "Pending".to_string(),
            MissionState::InProgress => "In Progress".to_string(),
            MissionState::Completed => "Completed".to_string(),
        })
    }
    pub fn get_zone_count(&self) -> usize { self.zones.len() }
    pub fn get_zone_data_x(&self, index: usize) -> Option<f32> { self.zones.get(index).map(|z| z.rect.min.x) }
    pub fn get_zone_data_y(&self, index: usize) -> Option<f32> { self.zones.get(index).map(|z| z.rect.min.y) }
    pub fn get_zone_data_w(&self, index: usize) -> Option<f32> { self.zones.get(index).map(|z| z.rect.width()) }
    pub fn get_zone_data_h(&self, index: usize) -> Option<f32> { self.zones.get(index).map(|z| z.rect.height()) }
    pub fn get_npc_count(&self) -> usize { self.npcs.len() }
    pub fn get_npc_name(&self, index: usize) -> Option<String> { self.npcs.get(index).map(|n| n.name.clone()) }
    pub fn get_npc_pos_x(&self, index: usize) -> Option<f32> { self.npcs.get(index).map(|n| n.position.x) }
    pub fn get_npc_pos_y(&self, index: usize) -> Option<f32> { self.npcs.get(index).map(|n| n.position.y) }
    pub fn get_active_dialogue(&self) -> Option<String> {
        self.active_dialogue.as_ref().map(|d| {
            let npc = &self.npcs[d.npc_index];
            let line = &npc.dialogue[d.line_index];
            format!("{}: {}", npc.name, line)
        })
    }
    pub fn get_wall_count(&self) -> usize { self.walls.len() }
    pub fn get_wall_data_x(&self, index: usize) -> Option<f32> { self.walls.get(index).map(|w| w.min.x) }
    pub fn get_wall_data_y(&self, index: usize) -> Option<f32> { self.walls.get(index).map(|w| w.min.y) }
    pub fn get_wall_data_w(&self, index: usize) -> Option<f32> { self.walls.get(index).map(|w| w.width()) }
    pub fn get_wall_data_h(&self, index: usize) -> Option<f32> { self.walls.get(index).map(|w| w.height()) }
    pub fn get_world_item_count(&self) -> usize { self.world_items.len() }
    pub fn get_world_item_pos_x(&self, index: usize) -> Option<f32> { self.world_items.get(index).map(|wi| wi.position.x) }
    pub fn get_world_item_pos_y(&self, index: usize) -> Option<f32> { self.world_items.get(index).map(|wi| wi.position.y) }
    pub fn get_world_item_name(&self, index: usize) -> Option<String> { self.world_items.get(index).map(|wi| wi.item.name.clone()) }
    pub fn get_inventory_item_count(&self) -> usize { self.player.inventory.len() }
    pub fn get_inventory_item_name(&self, index: usize) -> Option<String> { self.player.inventory.get(index).map(|i| i.name.clone()) }
    pub fn run_lua_script(&self, script: &str) -> String {
        let result: LuaResult<String> = self.lua.load(script).eval();
        match result { Ok(res) => res, Err(e) => format!("LUA ERROR: {}", e.to_string()) }
    }
}
