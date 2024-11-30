use std::rc::Rc;
use ggez::{event, ContextBuilder, GameResult};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::graphics::{Color, FontData};

use node_thing::NodeThing;
use node_thing::node::node_definition::NodeDefinition;


fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(
        "node-thing",
        "lukecreator")
        .window_mode(WindowMode::default()
            .min_dimensions(240.0, 160.0)
            .borderless(false)
            .resizable(true))
        .window_setup(WindowSetup::default()
            .title("Node Thing")
            .vsync(true)
            .samples(NumSamples::Four))
        .build()?;
    
    let mut node_thing = NodeThing::new(&mut ctx);
    
    // example node
    let node = NodeDefinition::new(
        "Example Node",
        "This node is an example.",
        "example_node",
        Color::RED).create_node(&ctx);
    node_thing.add_node(node);
    
    let mut node = NodeDefinition::default().create_node(&ctx);
    node.set_position(400.0, 650.0);
    node_thing.add_node(node);
    
    // starts application loop
    event::run(ctx, event_loop, node_thing);
}

