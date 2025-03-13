use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_health_bar3d::prelude::{BarSettings, HealthBarPlugin, Percentage};


use crate::asset_loader::AssetLoadingState;

pub fn plugin(app: &mut App) {
    app
        .add_plugins(HealthBarPlugin::<Health>::default())
        .add_event::<HealthModifyEvent>()
        .add_event::<DeathEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (health_modify, die, display_health).run_if(in_state(AssetLoadingState::Loaded)));
}

// Health Implementation

#[derive(Component, Reflect)]
#[require(BarSettings::<Health>(health_bar_default))]
pub struct Health {
    current_health: i32,
    max_health: i32
}

fn health_bar_default() -> BarSettings<Health> {
    BarSettings::<Health> {
        width: 5.,
        offset: 5.5,
        height: bevy_health_bar3d::prelude::BarHeight::Relative(0.075),
        ..default()
    }
}

impl Health {
    pub fn new(max_health: i32) -> Self {
        Self {
            current_health: max_health,
            max_health
        }
    }
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }
}

// Health-based events

#[derive(Event)]
pub struct HealthModifyEvent {
    pub amount: i32,
    pub damaged_entity: Entity
}

#[derive(Event)]
pub struct DeathEvent(Entity);

pub fn setup() {
    //Not sure
}

pub fn health_modify(
    mut health_modify_event: EventReader<HealthModifyEvent>,
    mut death_event_writer: EventWriter<DeathEvent>,
    mut health_query: Query<&mut Health>
) {
    //receive event and do things based on event.
    for event in health_modify_event.read() {
        println!("Health Modify Event: {:?}", event.amount);
        let Ok(mut health) = health_query.get_mut(event.damaged_entity) else {
            println!("No health component found for entity: {}", event.damaged_entity);
            continue;
        };

        health.current_health = (health.current_health + event.amount).clamp(0, health.max_health);

        if health.current_health <= 0 {
            death_event_writer.send(DeathEvent(event.damaged_entity));
        }
    }
}

pub fn die(
    mut death_event: EventReader<DeathEvent>,
    mut commands: Commands,
) {
    for event in death_event.read() {

        let Some(entity) = commands.get_entity(event.0) else {
            continue;
        };
        
        entity.despawn_recursive();
    }
}


pub fn display_health(
    mut contexts: EguiContexts,
    health_query: Query<&Health>,
) {


    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {

        health_query.iter().for_each(|health| {
            ui.label(format!("Health: {:?}", health.current_health));
        });
    });

}