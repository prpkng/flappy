mod main_menu;
mod preparing;
mod game_over;

use bevy::prelude::*;

use crate::{gameplay::GameState};

pub fn setup_screens(app: &mut App)
{
    app.add_plugins((main_menu::setup, preparing::setup, game_over::setup));
}