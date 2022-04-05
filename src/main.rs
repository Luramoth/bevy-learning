use bevy::{prelude::{App, Commands, Query, With, Component}, DefaultPlugins};

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);


fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Kayla Mizuki".to_string()));
    commands.spawn().insert(Person).insert(Name("Jacob Garrak".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}


fn main() {
    App::new()
		.add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
