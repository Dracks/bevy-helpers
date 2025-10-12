use std::marker::PhantomData;

use bevy::{prelude::*, state::state::FreelyMutableState};


pub struct LoadingPlugin<T>{
    states: PhantomData<T>
}

impl<T> LoadingPlugin<T> {
    pub fn new() -> Self {
        Self {
            states: PhantomData::<T>{}
        }
    }
}

impl<T: States+ FreelyMutableState+Copy> Plugin for LoadingPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_loading::<T>)
            .add_systems(Update, count_timer)
            .insert_resource(LoadFiles::default());
    }
}

#[derive(Resource, Debug, Default)]
pub struct LoadFiles {
    wait_time: Timer,
    assets: Vec<UntypedHandle>,
}

impl LoadFiles {
    pub fn from_duration(wait_duration: f32) -> Self {
        Self {
            wait_time: Timer::from_seconds(wait_duration, TimerMode::Once),
            ..Default::default()
        }
    }
    pub fn from_time(wait_time: Timer) -> Self {
        Self {
            wait_time,
            ..Default::default()
        }
    }

    pub fn with_assets(self, assets: Vec<UntypedHandle>) -> Self {
        Self {
            assets,
            ..self
        }
    }

    pub fn push_asset<T: Into<UntypedHandle>>(&mut self, asset: T){
        self.assets.push(asset.into());
    }

    pub fn is_complete(&self, asset_server: &AssetServer) -> bool{
        self.assets.iter().all(|handler| {
            !asset_server.get_recursive_dependency_load_state(handler)
                .is_none_or(|state| !state.is_loaded())
        }) && self.wait_time.is_finished()
    }

    pub fn percent(&self, asset_server: &AssetServer) -> f32 {
        let mut complete_count = self.assets.iter().filter(|handler| {
            !asset_server.get_recursive_dependency_load_state(*handler)
                .is_none_or(|state| !state.is_loaded())
        }).count();
        if self.wait_time.is_finished() {
            complete_count += 1;
        }
        (complete_count as f32 *100.0) / (self.assets.iter().count()+1)as f32

    }
}

#[derive(Component, Debug)]
pub struct Loading<T: States> {
    next: T
}

impl<T: States> Loading<T> {
    pub fn new(next: T) -> Self {
        Self {
            next,
        }
    }
}

fn count_timer(mut status: ResMut<LoadFiles>, time: Res<Time>) {
    if status.wait_time.is_finished() {
        return
    }
    status.wait_time.tick(time.delta());
}

fn handle_loading<T: States+ FreelyMutableState+Copy>(
    next_step: Single<&Loading<T>>,
    mut next_state: ResMut<NextState<T>>,
    status: Res<LoadFiles>,
    asset_server: Res<AssetServer>
){

    if status.is_complete(&asset_server) {
        next_state.set(next_step.next);
    }
}
