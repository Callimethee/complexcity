use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct AssetHandles {
    pub street: Handle<Image>,
    pub person: Handle<Image>,
    pub selector: Handle<Image>,
    pub house: Handle<Image>,
    pub forum: Handle<Image>,
    pub cinema: Handle<Image>,
    pub hospital: Handle<Image>,
    pub pool: Handle<Image>,
    pub restaurant: Handle<Image>,
    pub creative: Handle<Image>,
    pub tree: Handle<Image>,
    pub lamp: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandles>()
            .add_systems(PreStartup, load_assets);
    }
}

/// Load assets once, provide handles for the rest of the game
fn load_assets(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.person = asset_server.load("Adam_idle_front.png");
    asset_handles.street = asset_server.load("street.png");
    asset_handles.selector = asset_server.load("selector.png");
    asset_handles.house = asset_server.load("house.png");
    asset_handles.forum = asset_server.load("forum.png");
    asset_handles.cinema = asset_server.load("cinema.png");
    asset_handles.hospital = asset_server.load("hospital.png");
    asset_handles.pool = asset_server.load("pool.png");
    asset_handles.restaurant = asset_server.load("restaurant.png");
    asset_handles.creative = asset_server.load("creative_mart.png");
    asset_handles.tree = asset_server.load("tree.png");
    asset_handles.lamp = asset_server.load("lamp.png");
}
