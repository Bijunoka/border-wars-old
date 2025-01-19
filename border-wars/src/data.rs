use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

///--------------------------------------------------------------------------------------------------------------------------------------------
/// ALL EVENTS -------------------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EventBuild {
    pub who_are_u: PacketIdentification,
    pub pos: BlockPosition,
    pub type_b: BlockType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConqueteEvent(pub BlockPosition);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RecoltEvent {
    pub pos: BlockPosition,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DeplacementEvent {
    pub pos_start: BlockPosition,
    pub pos_end: BlockPosition,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DestroyEvent {
    pub pos: BlockPosition,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CombatEvent {
    pub pos: BlockPosition,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ReponceCombatEvent {
    pub id: PacketIdentification,
    pub pos: BlockPosition,
    pub entity: Entity,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TrainEvent {
    pub niv: NiveauTroup,
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// BLOCK TYPE --------------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockType {
    AvantPost,
    Caserne,
    Castle,
    Farm,
    Grass,
    GrassForest,
    GrassHill,
    Mine,
    Tower,
    Upgradeur,
    Wall,
    Sheep,
}

impl BlockType {
    pub fn get_draw(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match self {
            BlockType::Grass => asset_server.load("map/Grass.png"),
            BlockType::GrassForest => asset_server.load("map/GrassForest.png"),
            BlockType::Castle => asset_server.load("map/Castle.png"),
            BlockType::Farm => asset_server.load("map/Farm.png"),
            BlockType::Mine => asset_server.load("map/Mine.png"),
            BlockType::Tower => asset_server.load("map/Tower.png"),
            BlockType::Wall => asset_server.load("map/Wall.png"),
            BlockType::GrassHill => asset_server.load("map/GrassHill.png"),
            BlockType::AvantPost => asset_server.load("map/AvantPost.png"),
            BlockType::Caserne => asset_server.load("map/Caserne.png"),
            BlockType::Upgradeur => asset_server.load("map/Upgradeur.png"),
            BlockType::Sheep => asset_server.load("map/Sheep.png"),
        }
    }
}

impl BlockType {
    pub fn get_text(&self) -> &str {
        match self {
            BlockType::Caserne => "a barracks, it allows you to create troops",
            BlockType::Castle => "a castle, it's your base and you must defend it at all costs",
            BlockType::Grass => "a block of grass, it's useless",
            BlockType::GrassForest => "a forest, if you break it you can collect resources",
            BlockType::GrassHill => "a mountain, if you break it you can collect resources",
            BlockType::Mine => "a mine, it allows you to collect resources",
            BlockType::Farm => "a farm, it allows you to collect resources",
            BlockType::AvantPost => "an outpost, it allows you to teleport your troops",
            BlockType::Tower => "a defense tower, it allows you to defend your troops",
            BlockType::Upgradeur => "an upgrader",
            BlockType::Wall => "a wall",
            BlockType::Sheep => "a food reseve wich permit create an army",
        }
    }
}

impl BlockType {
    pub fn get_text_with_typeb(&self) -> String {
        format!(
            "{:?}\n\n{} Wood,\n{} Stone",
            self,
            self.get_price()[1],
            self.get_price()[0]
        )
    }
}

impl BlockType {
    pub fn get_price(&self) -> [u32; 2] {
        match self {
            BlockType::AvantPost => [2300, 3000],
            BlockType::Caserne => [600, 400],
            BlockType::Castle => todo!(),
            BlockType::Farm => [200, 300],
            BlockType::Grass => todo!(),
            BlockType::GrassForest => todo!(),
            BlockType::GrassHill => todo!(),
            BlockType::Mine => [300, 200],
            BlockType::Tower => [1500, 1500],
            BlockType::Upgradeur => [1000, 1000],
            BlockType::Wall => [4000, 4000],
            BlockType::Sheep => [400, 500],
        }
    }
}

impl BlockType {
    pub fn is_breakable(&self) -> bool {
        match &self {
            BlockType::AvantPost => true,
            BlockType::Caserne => true,
            BlockType::Castle => true,
            BlockType::Farm => true,
            BlockType::Grass => false,
            BlockType::GrassForest => false,
            BlockType::GrassHill => false,
            BlockType::Mine => true,
            BlockType::Tower => true,
            BlockType::Upgradeur => true,
            BlockType::Wall => false,
            BlockType::Sheep => true,
        }
    }
}

impl BlockType {
    pub fn is_empty(&self) -> bool {
        match &self {
            BlockType::AvantPost => false,
            BlockType::Caserne => false,
            BlockType::Castle => false,
            BlockType::Farm => false,
            BlockType::Grass => true,
            BlockType::GrassForest => true,
            BlockType::GrassHill => false,
            BlockType::Mine => false,
            BlockType::Tower => false,
            BlockType::Upgradeur => false,
            BlockType::Wall => false,
            BlockType::Sheep => false,
        }
    }
}

pub fn get_type_with_number(nbr: u8) -> Option<BlockType> {
    match nbr {
        1 => Some(BlockType::Caserne),
        2 => Some(BlockType::AvantPost),
        3 => Some(BlockType::Mine),
        4 => Some(BlockType::Farm),
        5 => Some(BlockType::Tower),
        6 => Some(BlockType::Upgradeur),
        7 => Some(BlockType::Wall),
        8 => Some(BlockType::Sheep),
        _ => None,
    }
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// BLOCK APPARTENANCE ------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockApartenance {
    Neutre,
    Joueur1,
    Joueur2,
}

impl BlockApartenance {
    pub fn get_flip(&self, identity: PacketIdentification) -> bool {
        let res = match identity {
            PacketIdentification::Joueur1 => (false, true),
            PacketIdentification::Joueur2 => (true, false),
            PacketIdentification::Spectateur => (false, true),
        };
        match self {
            BlockApartenance::Joueur1 => res.0,
            BlockApartenance::Joueur2 => res.1,
            BlockApartenance::Neutre => res.0,
        }
    }
}

impl BlockApartenance {
    pub fn get_text(&self, identity: &PacketIdentification) -> &str {
        match identity {
            PacketIdentification::Joueur1 => match &self {
                BlockApartenance::Neutre => "it doesn't belong to anyone",
                BlockApartenance::Joueur1 => "it belongs to you",
                BlockApartenance::Joueur2 => "it belongs to your enemy",
            },
            PacketIdentification::Joueur2 => match &self {
                BlockApartenance::Neutre => "it doesn't belong to anyone",
                BlockApartenance::Joueur1 => "it belongs to your enemy",
                BlockApartenance::Joueur2 => "it belongs to you",
            },
            PacketIdentification::Spectateur => match &self {
                BlockApartenance::Neutre => "it doesn't belong to anyone",
                BlockApartenance::Joueur1 => "it belongs to Joueur 1",
                BlockApartenance::Joueur2 => "it belongs to Joueur 2",
            },
        }
    }
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// BLOC POSITION -----------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: u8,
    pub y: u8,
}

impl BlockPosition {
    pub fn to_transform(&self, identity: PacketIdentification) -> Transform {
        let max_y: u8 = 9;
        let max_x = 10.;
        let new_y = max_y - 1 - self.y;
        let offset_x = new_y % 2;

        let mut new_x = self.x as f32 + (offset_x as f32 * 0.5);
        if identity == PacketIdentification::Joueur2 {
            new_x = max_x - new_x;
        };

        Transform::from_xyz(new_x, new_y as f32 * 0.42, self.y as f32)
    }
    pub fn to_troup_position(&self) -> TroupPosition {
        TroupPosition {
            x: self.x,
            y: self.y,
        }
    }
}

impl TroupPosition {
    pub fn to_transform(&self, identity: PacketIdentification) -> Transform {
        let max_y: u8 = 9;
        let max_x = 10.;
        let new_y = max_y - 1 - self.y;
        let offset_x = new_y % 2;

        let mut new_x = self.x as f32 + (offset_x as f32 * 0.5);
        if identity == PacketIdentification::Joueur2 {
            new_x = max_x - new_x;
        };

        Transform::from_xyz(new_x, new_y as f32 * 0.42 - 0.34, self.y as f32 + 1.)
    }

    pub fn to_troup_position(&self) -> BlockPosition {
        BlockPosition {
            x: self.x,
            y: self.y,
        }
    }
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// ALL TEXTS ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy)]

pub struct TextInterface {}

#[derive(Component, Debug, Clone, Copy)]
pub struct TextInfo;

#[derive(Component, Debug, Clone, Copy)]
pub struct TextRace;

#[derive(Component, Debug, Clone, Copy)]
pub struct TextRessources;

#[derive(Component, Debug, Clone, Copy)]
pub struct BlockInfoBuild;

#[derive(Component, Debug, Clone, Copy)]
pub struct TroupInfoTrain;

///--------------------------------------------------------------------------------------------------------------------------------------------
/// PACKET IDENTIFICATION ---------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Copy, Clone, Resource, Debug, PartialEq, Eq, Component)]
pub enum PacketIdentification {
    Joueur1,
    Joueur2,
    Spectateur,
}

impl PacketIdentification {
    pub fn get_block_apartenance(&self) -> BlockApartenance {
        match self {
            PacketIdentification::Joueur1 => BlockApartenance::Joueur1,
            PacketIdentification::Joueur2 => BlockApartenance::Joueur2,
            PacketIdentification::Spectateur => todo!(),
        }
    }
    pub fn get_troup_apartenance(&self) -> TroupApartenance {
        match self {
            PacketIdentification::Joueur1 => TroupApartenance::Joueur1,
            PacketIdentification::Joueur2 => TroupApartenance::Joueur2,
            PacketIdentification::Spectateur => todo!(),
        }
    }
    pub fn get_enemie(&self) -> PacketIdentification {
        match self {
            PacketIdentification::Joueur1 => PacketIdentification::Joueur2,
            PacketIdentification::Joueur2 => PacketIdentification::Joueur1,
            PacketIdentification::Spectateur => todo!(),
        }
    }
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// SELECTION ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Debug)]
pub struct SelectedCase {
    pub pos: Vec3,
    pub blockpos: BlockPosition,
    pub entity: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct SelectBar {}

impl SelectBar {
    pub fn get_draw(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        asset_server.load("map/Select.png")
    }
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// TROUP -------------------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Troup {
    pub race: Race,
    pub niveau: NiveauTroup,
}

impl Troup {
    pub fn get_image(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        let race = match self.race {
            Race::Chauve => "chauve",
            Race::Arabe => "arabe",
            Race::Noir => "noir",
            Race::Chevalier => "ch",
            Race::Chinoi => "chinois",
            Race::Voleur => "voleur",
        };
        let niv = match self.niveau {
            NiveauTroup::Niveau1 => "1",
            NiveauTroup::Niveau2 => "2",
            NiveauTroup::Niveau3 => "3",
        };
        let txt = format!("perso/{}{}.png", race, niv);
        asset_server.load(txt)
    }

    pub fn get_text(&self) -> String {
        format!(
            "Force : {}\nHp : {}\nShield : {}\nVitesse : {}\nRange : {}\nPrice : {}",
            self.get_attaque(),
            self.get_hp(),
            self.get_sheild(),
            self.get_vitesse(),
            self.get_range(),
            self.get_price(),
        )
    }

    pub fn get_attaque(&self) -> u8 {
        match self.race {
            Race::Chauve => match self.niveau {
                NiveauTroup::Niveau1 => 15,
                NiveauTroup::Niveau2 => 25,
                NiveauTroup::Niveau3 => 35,
            },
            Race::Arabe => match self.niveau {
                NiveauTroup::Niveau1 => 20,
                NiveauTroup::Niveau2 => 30,
                NiveauTroup::Niveau3 => 40,
            },
            Race::Noir => match self.niveau {
                NiveauTroup::Niveau1 => 10,
                NiveauTroup::Niveau2 => 20,
                NiveauTroup::Niveau3 => 30,
            },
            Race::Chevalier => match self.niveau {
                NiveauTroup::Niveau1 => 15,
                NiveauTroup::Niveau2 => 25,
                NiveauTroup::Niveau3 => 35,
            },
            Race::Chinoi => match self.niveau {
                NiveauTroup::Niveau1 => 5,
                NiveauTroup::Niveau2 => 10,
                NiveauTroup::Niveau3 => 20,
            },
            Race::Voleur => match self.niveau {
                NiveauTroup::Niveau1 => 25,
                NiveauTroup::Niveau2 => 35,
                NiveauTroup::Niveau3 => 45,
            },
        }
    }
    pub fn get_hp(&self) -> u8 {
        match self.race {
            Race::Chauve => match self.niveau {
                NiveauTroup::Niveau1 => 100,
                NiveauTroup::Niveau2 => 150,
                NiveauTroup::Niveau3 => 200,
            },
            Race::Arabe => match self.niveau {
                NiveauTroup::Niveau1 => 80,
                NiveauTroup::Niveau2 => 120,
                NiveauTroup::Niveau3 => 160,
            },
            Race::Noir => match self.niveau {
                NiveauTroup::Niveau1 => 100,
                NiveauTroup::Niveau2 => 150,
                NiveauTroup::Niveau3 => 200,
            },
            Race::Chevalier => match self.niveau {
                NiveauTroup::Niveau1 => 150,
                NiveauTroup::Niveau2 => 200,
                NiveauTroup::Niveau3 => 250,
            },
            Race::Chinoi => match self.niveau {
                NiveauTroup::Niveau1 => 120,
                NiveauTroup::Niveau2 => 160,
                NiveauTroup::Niveau3 => 200,
            },
            Race::Voleur => match self.niveau {
                NiveauTroup::Niveau1 => 80,
                NiveauTroup::Niveau2 => 120,
                NiveauTroup::Niveau3 => 160,
            },
        }
    }
    pub fn get_sheild(&self) -> u8 {
        match self.race {
            Race::Chauve => match self.niveau {
                NiveauTroup::Niveau1 => 10,
                NiveauTroup::Niveau2 => 20,
                NiveauTroup::Niveau3 => 30,
            },
            Race::Arabe => match self.niveau {
                NiveauTroup::Niveau1 => 5,
                NiveauTroup::Niveau2 => 10,
                NiveauTroup::Niveau3 => 20,
            },
            Race::Noir => match self.niveau {
                NiveauTroup::Niveau1 => 10,
                NiveauTroup::Niveau2 => 20,
                NiveauTroup::Niveau3 => 30,
            },
            Race::Chevalier => match self.niveau {
                NiveauTroup::Niveau1 => 15,
                NiveauTroup::Niveau2 => 25,
                NiveauTroup::Niveau3 => 35,
            },
            Race::Chinoi => match self.niveau {
                NiveauTroup::Niveau1 => 20,
                NiveauTroup::Niveau2 => 30,
                NiveauTroup::Niveau3 => 40,
            },
            Race::Voleur => match self.niveau {
                NiveauTroup::Niveau1 => 5,
                NiveauTroup::Niveau2 => 10,
                NiveauTroup::Niveau3 => 20,
            },
        }
    }
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// TROUPE APPARTENNANCE ----------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TroupApartenance {
    Joueur1,
    Joueur2,
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// TROUPE POSITION ---------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TroupPosition {
    pub x: u8,
    pub y: u8,
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// NIVEAU TROUPE -----------------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NiveauTroup {
    Niveau1,
    Niveau2,
    Niveau3,
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// RACES -------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Resource, Component, Clone, Copy, PartialEq, Eq)]
pub enum Race {
    Chauve,
    Arabe,
    Noir,
    Chevalier,
    Chinoi,
    Voleur,
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// FONCTIONS ---------------------------------------------------------------------------------------------------------------------------------

pub fn can_build(block_t: BlockType, moula: Moula) -> bool {
    if moula.stone >= block_t.get_price()[0] && moula.wood >= block_t.get_price()[1] {
        true
    } else {
        false
    }
}

pub fn can_train(troup: Troup, moula: Moula) -> bool {
    if moula.food >= troup.get_price() {
        true
    } else {
        false
    }
}

pub fn print_text(case: &mut Mut<Text>, text: &str, font: TextStyle) {
    **case = Text::from_section(text, font)
}

pub fn get_font(asset_server: &Res<AssetServer>, taile: f32) -> TextStyle {
    TextStyle {
        font_size: taile,
        color: Color::WHITE,
        font: asset_server.load("fonts/Mabook.ttf"),
    }
}

pub fn cases_adjacentes_rayon(
    positions: &[BlockPosition],
    center: BlockPosition,
    rayon: u8,
) -> HashSet<BlockPosition> {
    if rayon == 0 {
        let mut variable = HashSet::new();
        variable.insert(center);
        return variable;
    }
    let result: Vec<BlockPosition> = positions
        .iter()
        .filter_map(|position| {
            let x = position.x as i8 - center.x as i8;
            let y = position.y as i8 - center.y as i8;
            if center.y % 2 == 0 {
                match (x, y) {
                    (1, 1) | (1, -1) => None,
                    (-1..=1, -1..=1) => Some(position),
                    _ => None,
                }
            } else {
                match (x, y) {
                    (-1, 1) | (-1, -1) => None,
                    (-1..=1, -1..=1) => Some(position),
                    _ => None,
                }
            }
        })
        .copied()
        .collect();
    let mut moui = HashSet::new();
    for pos in result {
        moui.extend(cases_adjacentes_rayon(positions, pos, rayon - 1))
    }
    moui
}

///--------------------------------------------------------------------------------------------------------------------------------------------
/// LE RESTE  ----------------------------------------------------------------------------------------------------------------------
#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Debug, Clone)]
pub struct CaseAdjacente {
    pub entities: Vec<Entity>,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub enum Etat {
    Idle,
    Building,
    Train,
    Move {
        adjacentes: HashSet<BlockPosition>,
        pos_d: BlockPosition,
    },
}

#[derive(Resource, Debug, Clone)]
pub enum DataPrint {
    None,
    Text { text: String },
    AllBlock,
    Train,
}

#[derive(Resource, Component, Debug, Clone, Copy)]
pub struct Moula {
    pub stone: u32,
    pub wood: u32,
    pub food: u32,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tour {
    pub current_player: PacketIdentification,
}

impl Tour {
    pub fn change_tour(&mut self) {
        self.current_player = match self.current_player {
            PacketIdentification::Joueur1 => PacketIdentification::Joueur2,
            PacketIdentification::Joueur2 => PacketIdentification::Joueur1,
            PacketIdentification::Spectateur => unreachable!("Serveur said spectator turn..."),
        };
    }
}

pub fn il_peu(vec: &Vec<BlockPosition>) -> bool {
    for block in vec {
        if *block != (BlockPosition { x: 100, y: 100 }) {
            return true;
        }
    }
    false
}

impl Troup {
    pub fn get_vitesse(&self) -> u8 {
        match self.race {
            Race::Chauve => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Arabe => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Noir => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 2,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Chevalier => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Chinoi => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Voleur => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 2,
                NiveauTroup::Niveau3 => 2,
            },
        }
    }
    pub fn get_range(&self) -> u8 {
        let a = match self.race {
            Race::Chauve => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Arabe => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Noir => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 2,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Chevalier => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Chinoi => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 1,
                NiveauTroup::Niveau3 => 2,
            },
            Race::Voleur => match self.niveau {
                NiveauTroup::Niveau1 => 1,
                NiveauTroup::Niveau2 => 2,
                NiveauTroup::Niveau3 => 2,
            },
        };
        a + 1
    }
    pub fn get_price(&self) -> u32 {
        match self.niveau {
            NiveauTroup::Niveau1 => 50,
            NiveauTroup::Niveau2 => 75,
            NiveauTroup::Niveau3 => 100,
        }
    }
}

pub fn get_troup_niveau(x: u8) -> NiveauTroup {
    match x {
        1 => NiveauTroup::Niveau1,
        2 => NiveauTroup::Niveau2,
        3 => NiveauTroup::Niveau3,
        _ => todo!(),
    }
}
