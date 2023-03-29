use bevy::prelude::*;

pub struct MenuMainPlugin;

impl Plugin for MenuMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu);
    }
}

pub fn main_menu() {
    println!("Main Menu!");
}
