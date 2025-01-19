use crate::data::*;
use bevnet::server::{ClientConnection, FromClient, Synced};
use bevy::prelude::*;
use rand::Rng;

fn gestion_new_client(
    conexions: Query<(&ClientConnection, Entity), Added<ClientConnection>>,
    mut count: Local<u8>,
    mut commands: Commands,
) {
    for (conexion, entity) in conexions.iter() {
        let list = [
            Race::Arabe,
            Race::Chevalier,
            Race::Chauve,
            Race::Noir,
            Race::Voleur,
            Race::Chinoi,
        ];
        let mut rng = rand::thread_rng();
        let current_race = &list[rng.gen_range(0..6)];
        *count += 1;
        let packet = match *count {
            1 => {
                println!("Player 1 is connected");
                PacketIdentification::Joueur1
            }
            2 => {
                println!("Player 2 is connected");
                PacketIdentification::Joueur2
            }
            _ => {
                println!("New Spectator connected");
                PacketIdentification::Spectateur
            }
        };
        conexion.send(&packet);
        commands.entity(entity).insert(packet);
        if packet != PacketIdentification::Spectateur {
            commands
                .entity(entity)
                .insert(Moula {
                    stone: 5000,
                    wood: 5000,
                    food: 0,
                })
                .insert(*current_race);
            conexion.send(current_race);
        }
    }
}

pub struct PluginGestionServeur;

impl Plugin for PluginGestionServeur {
    fn build(&self, app: &mut App) {
        app.add_system(gestion_build)
            .add_system(gestion_new_client)
            .add_system(gestion_use_resource)
            .add_system(destroy_server_side)
            .add_system(new_troup)
            .add_system(deplacement_troup)
            .add_system(combat)
            .add_system(gestion_reponse)
            .add_system(get_ressource);
    }
}

fn gestion_build(
    mut event: EventReader<FromClient<EventBuild>>,
    mut query: Query<(&mut BlockPosition, &mut BlockApartenance, &mut BlockType)>,
    mut moula: Query<(&mut Moula, &PacketIdentification)>,
    mut tour: ResMut<Tour>,
) {
    for data in event.iter() {
        let EventBuild {
            pos,
            type_b,
            who_are_u,
        } = data.event;

        if let Ok((mut current_moula, id)) = moula.get_mut(data.entity) {
            if tour.current_player == *id {
                for (block_pos, mut apartenance, mut type_block) in query.iter_mut() {
                    if *block_pos == pos {
                        if *type_block == BlockType::Grass {
                            if who_are_u.get_block_apartenance() == *apartenance {
                                if can_build(type_b, *current_moula) {
                                    current_moula.stone -= type_b.get_price()[0];
                                    current_moula.wood -= type_b.get_price()[1];
                                    *apartenance = who_are_u.get_block_apartenance();
                                    *type_block = type_b;
                                }
                            }
                        }
                    }
                }
                tour.change_tour()
            }
        }
    }
}

fn gestion_use_resource(
    mut event: EventReader<FromClient<RecoltEvent>>,
    mut connexions: Query<(&mut Moula, &PacketIdentification)>,
    mut query: Query<(&BlockApartenance, &mut BlockType, &BlockPosition)>,
    mut tour: ResMut<Tour>,
) {
    for data in event.iter() {
        if let Ok((mut moula, id)) = connexions.get_mut(data.entity) {
            if tour.current_player == *id {
                for (apa, mut type_b, pos) in query.iter_mut() {
                    if *pos == data.event.pos {
                        if id.get_block_apartenance() == *apa {
                            match *type_b {
                                BlockType::GrassForest => moula.wood += 500,
                                BlockType::GrassHill => moula.stone += 500,
                                _ => break,
                            };
                            *type_b = BlockType::Grass;
                        }
                    }
                }
                tour.change_tour();
            }
        }
    }
}

fn destroy_server_side(
    mut event: EventReader<FromClient<DestroyEvent>>,
    mut connexions: Query<&PacketIdentification>,
    mut query: Query<(&BlockApartenance, &mut BlockType, &BlockPosition)>,
    mut tour: ResMut<Tour>,
) {
    for data in event.iter() {
        if let Ok(id) = connexions.get_mut(data.entity) {
            if tour.current_player == *id {
                for (apa, mut type_b, pos) in query.iter_mut() {
                    if *pos == data.event.pos {
                        if id.get_block_apartenance() == *apa {
                            if *type_b != BlockType::Grass
                                || *type_b != BlockType::GrassHill
                                || *type_b != BlockType::GrassForest
                                || *type_b != BlockType::Wall
                            {
                                *type_b = BlockType::Grass;
                            }
                        }
                    }
                }
                tour.change_tour()
            }
        }
    }
}

fn new_troup(
    mut event: EventReader<FromClient<TrainEvent>>,
    mut connexions: Query<(&PacketIdentification, &Race)>,
    blocks: Query<(&BlockType, &BlockApartenance, &BlockPosition)>,
    mut commands: Commands,
    mut tour: ResMut<Tour>,
    troups: Query<&TroupPosition>,
) {
    for data in event.iter() {
        if let Ok((id, race)) = connexions.get_mut(data.entity) {
            let mut my_grass_block: Vec<BlockPosition> = blocks
                .iter()
                .filter_map(|(&type_b, &appa, &pos)| {
                    if id.get_block_apartenance() == appa && type_b == BlockType::Grass {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .collect();

            for index in 0..my_grass_block.len() {
                for pos_t in troups.iter() {
                    if pos_t.to_troup_position() == my_grass_block[index] {
                        my_grass_block[index] = BlockPosition { x: 100, y: 100 };
                    }
                }
            }

            if my_grass_block != vec![] && il_peu(&my_grass_block) {
                let mut rng = rand::thread_rng();
                let mut current_case = &my_grass_block[rng.gen_range(0..my_grass_block.len())];
                while current_case == &(BlockPosition { x: 100, y: 100 }) {
                    let mut rng = rand::thread_rng();
                    current_case = &my_grass_block[rng.gen_range(0..my_grass_block.len())];
                }
                commands.spawn((
                    Troup {
                        race: *race,
                        niveau: data.event.niv,
                    },
                    id.get_troup_apartenance(),
                    TroupPosition {
                        x: current_case.x,
                        y: current_case.y,
                    },
                    Synced,
                ));
            }
            tour.change_tour();
        }
    }
}

fn deplacement_troup(
    mut event: EventReader<FromClient<DeplacementEvent>>,
    blocks: Query<&BlockPosition>,
    mut troups: Query<&mut TroupPosition>,
    mut tour: ResMut<Tour>,
) {
    for data in event.iter() {
        for block in blocks.iter() {
            if *block == data.event.pos_start {
                for mut troup in troups.iter_mut() {
                    if *troup == data.event.pos_start.to_troup_position() {
                        *troup = data.event.pos_end.to_troup_position();
                        tour.change_tour();
                    }
                }
            }
        }
    }
}
fn combat(
    positions: Query<(&BlockPosition, Entity, &mut BlockApartenance)>,
    mut event: EventReader<FromClient<CombatEvent>>,
    connexions: Query<(&PacketIdentification, &Race)>,
    troup: Query<(&Troup, &TroupApartenance, &TroupPosition)>,
    mut event_rep: EventWriter<ReponceCombatEvent>,
    mut tour: ResMut<Tour>,
) {
    for data in event.iter() {
        if let Ok((id, _)) = connexions.get(data.entity) {
            let mut list1: Vec<Troup> = vec![];
            let mut list2: Vec<Troup> = vec![];
            let mut list: Vec<BlockPosition> = vec![];
            for (block_pos, _, _) in positions.iter() {
                list.push(*block_pos)
            }

            for (troup, troup_apa, pos_t) in troup.iter() {
                for pos_b in
                    cases_adjacentes_rayon(&list, pos_t.to_troup_position(), troup.get_range())
                {
                    if pos_b == data.event.pos {
                        match troup_apa {
                            TroupApartenance::Joueur1 => list1.push(*troup),
                            TroupApartenance::Joueur2 => list2.push(*troup),
                        }
                    }
                }
            }

            let mut hp1: i32 = 0;
            for ellement in &list1 {
                hp1 += ellement.get_hp() as i32
            }
            let mut hp2: i32 = 0;
            for ellement in &list2 {
                hp2 += ellement.get_hp() as i32
            }

            let mut force1: i32 = 0;
            for ellement in &list1 {
                force1 += ellement.get_attaque() as i32
            }
            let mut force2: i32 = 0;
            for ellement in &list2 {
                force2 += ellement.get_attaque() as i32
            }

            let mut sheild1: i32 = 1;
            for ellement in &list1 {
                sheild1 += ellement.get_sheild() as i32
            }
            let mut sheild2: i32 = 1;
            for ellement in &list2 {
                sheild2 += ellement.get_sheild() as i32
            }

            let res: i32 = hp1 - (force2 * 1 / sheild1) - ( hp2 - (force1 * 1 / sheild2 ));
            let resu: PacketIdentification = if res < 0 {
                PacketIdentification::Joueur1
            } else if res > 0 {
                PacketIdentification::Joueur2
            } else {
                id.get_enemie()
            };
            event_rep.send(ReponceCombatEvent {
                id: resu.get_enemie(),
                pos: data.pos,
                entity: data.entity,
            })
        }
        tour.change_tour();
    }
    
}

fn gestion_reponse(
    mut positions: Query<(&BlockPosition, &mut BlockApartenance)>,
    mut event: EventReader<ReponceCombatEvent>,
    troup: Query<(Entity, &Troup, &TroupApartenance, &TroupPosition)>,
    mut commands: Commands,
) {
    // suprimer les enemie qui peuvent y avoir accez
    // modifier l'appartenance du block
    for data in event.iter() {
        let mut list: Vec<BlockPosition> = vec![];

        for (block_pos, _) in positions.iter() {
            list.push(*block_pos);
        }
        for (entity, troup, troup_apa, pos_t) in troup.iter() {
            for pos_b in cases_adjacentes_rayon(&list, pos_t.to_troup_position(), troup.get_range())
            {
                if pos_b == data.pos {
                    if *troup_apa == data.id.get_enemie().get_troup_apartenance() {
                        println!("il y a un enemie");
                        commands.entity(entity).despawn()
                    }
                }
            }
        }
        for (pos, mut apa) in positions.iter_mut() {
            if *pos == data.pos {
                println!("on change le block ");
                *apa = data.id.get_block_apartenance()
            }
        }
    }
}

fn get_ressource(
    tour: Res<Tour>,
    mut conexions: Query<(&mut Moula, &PacketIdentification)>,
    blocks: Query<(&BlockType, &BlockApartenance)>,
) {
    if tour.is_changed() {
        for (mut moula, identity) in conexions.iter_mut() {
            for (type_b, apa) in blocks.iter() {
                if identity.get_block_apartenance() == *apa {
                    match type_b {
                        BlockType::Farm => moula.wood += 100,
                        BlockType::Mine => moula.stone += 100,
                        BlockType::Sheep => moula.food += 25,
                        _ => (),
                    }
                }
            }
        }
    }
}
