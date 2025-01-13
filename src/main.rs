// Define UI and converters modules
mod ui;
mod converters;

// Import necessary dependencies
use fltk::{prelude::*, *};
use std::path::Path;
use ui::components::ConverterUI;
use converters::{length, volume};

// Main application entry point
fn main() {
    // Initialize FLTK application
    let app = app::App::default();
    
    // Create main window with specific size and title
    let mut wind = window::Window::default()
        .with_size(250, 300)
        .with_label("Unit Converter");
    
    // Attempt to load and set application icon
    if let Ok(icon) = image::SvgImage::load(Path::new("src/icon.svg")) {
        wind.set_icon(Some(icon));
    } else {
        eprintln!("Failed to load icon from src/icon.svg");
    }
    
    // Initialize UI components
    let mut ui = ConverterUI::new();
    
    // Set up window layout and callbacks
    setup_layout(&mut wind, &ui);
    setup_callbacks(&mut ui);

    // Finalize window setup and run application
    wind.end();
    wind.show();
    app.run().unwrap();
}

// Function to set up the window layout
fn setup_layout(_wind: &mut window::Window, ui: &ConverterUI) {
    // Create vertical pack layout container
    let mut pack = group::Pack::default()
        .with_size(100, 300)
        .center_of_parent()
        .with_type(group::PackType::Vertical);
    pack.set_spacing(5);
    
    // Add padding at the top
    let _padding = frame::Frame::default().with_size(0, 20);
    
    // Create and configure title label
    let mut title = frame::Frame::default()
        .with_size(0, 40)
        .with_label("Unit Converter");
    title.set_label_size(24);
    
    // Add all UI components to the pack
    pack.add(&ui.choice);
    pack.add(&ui.length_from);
    pack.add(&ui.length_to);
    pack.add(&ui.volume_from);
    pack.add(&ui.volume_to);
    pack.add(&ui.input);
    pack.add(&ui.button);
    pack.add(&ui.output);
    
    // Finalize pack layout
    pack.end();
}

// Function to set up event callbacks
fn setup_callbacks(ui: &mut ConverterUI) {
    // Set callback for unit type selection
    ui.choice.set_callback({
        let mut length_from = ui.length_from.clone();
        let mut length_to = ui.length_to.clone();
        let mut volume_from = ui.volume_from.clone();
        let mut volume_to = ui.volume_to.clone();
        move |c| {
            if let Some(choice) = c.choice() {
                match choice.as_str() {
                    // Show length units and hide volume units
                    "Length" => {
                        length_from.show();
                        length_to.show();
                        volume_from.hide();
                        volume_to.hide();
                    },
                    // Show volume units and hide length units
                    "Volume" => {
                        volume_from.show();
                        volume_to.show();
                        length_from.hide();
                        length_to.hide();
                    },
                    _ => {}
                }
            }
        }
    });

    // Set callback for conversion button
    ui.button.set_callback({
        let input = ui.input.clone();
        let mut output = ui.output.clone();
        let choice = ui.choice.clone();
        let length_from = ui.length_from.clone();
        let length_to = ui.length_to.clone();
        let volume_from = ui.volume_from.clone();
        let volume_to = ui.volume_to.clone();
        
        move |_| {
            // Parse input value and perform conversion
            if let Some(value) = input.value().parse::<f64>().ok() {
                if let Some(conv_type) = choice.choice() {
                    let result = match conv_type.as_str() {
                        // Perform length conversion
                        "Length" => length::convert_length(
                            value,
                            length_from.value().try_into().unwrap(),
                            length_to.value().try_into().unwrap()
                        ),
                        // Perform volume conversion
                        "Volume" => volume::convert_volume(
                            value,
                            volume_from.value().try_into().unwrap(),
                            volume_to.value().try_into().unwrap()
                        ),
                        // Default case returns original value
                        _ => value,
                    };
                    // Display result with 10 decimal places
                    output.set_value(&format!("{:.10}", result));
                }
            } else {
                // Handle invalid input
                println!("Invalid input");
            }
        }
    });
}