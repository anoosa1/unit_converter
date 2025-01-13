// Import necessary FLTK components
use fltk::{prelude::*, *};

// Main UI structure containing all components
pub struct ConverterUI {
    pub choice: menu::Choice,        // Conversion type selector (Length/Volume)
    pub length_from: menu::Choice,   // Length unit selector (From)
    pub length_to: menu::Choice,     // Length unit selector (To)
    pub volume_from: menu::Choice,   // Volume unit selector (From)
    pub volume_to: menu::Choice,     // Volume unit selector (To)
    pub input: input::FloatInput,    // Numeric input field
    pub output: output::Output,      // Read-only output field
    pub button: button::Button,      // Convert button
}

impl ConverterUI {
    // Constructor for creating new UI instance
    pub fn new() -> Self {
        // Create conversion type selector
        let mut choice = menu::Choice::default().with_size(80, 30).with_label("Convert: ");
        choice.add_choice("Length");
        choice.add_choice("Volume");
        choice.set_value(0);

        // Create length and volume unit selectors
        let (length_from, length_to) = Self::create_length_choices();
        let (volume_from, volume_to) = Self::create_volume_choices();

        // Create input field
        let input = input::FloatInput::default().with_size(150, 30);
        
        // Create output field
        let mut output = output::Output::default()
            .with_size(150, 30)
            .with_label("Result:");
        output.set_readonly(true);

        // Create convert button
        let button = button::Button::default()
            .with_size(80, 30)
            .with_label("Convert");

        // Return new UI instance
        Self {
            choice,
            length_from,
            length_to,
            volume_from,
            volume_to,
            input,
            output,
            button,
        }
    }

    // Helper function to create length unit selectors
    fn create_length_choices() -> (menu::Choice, menu::Choice) {
        // Create from/to selectors
        let mut from = menu::Choice::default().with_size(80, 30).with_label("From: ");
        let mut to = menu::Choice::default().with_size(80, 30).with_label("To: ");
        
        // Define available length units
        let units = [
            "Angstrom", "Nanometer", "Micron", "Millimeter", "Centimeter",
            "Meter", "Kilometer", "Inch", "Foot", "Yard", "Mile"
        ];

        // Add units to both selectors
        for unit in units {
            from.add_choice(unit);
            to.add_choice(unit);
        }

        // Set default selections (Meter to Kilometer)
        from.set_value(5);
        to.set_value(6);

        // Initially hide length selectors
        from.hide();
        to.hide();
        
        (from, to)
    }

    // Helper function to create volume unit selectors
    fn create_volume_choices() -> (menu::Choice, menu::Choice) {
        // Create from/to selectors
        let mut from = menu::Choice::default().with_size(80, 30).with_label("From: ");
        let mut to = menu::Choice::default().with_size(80, 30).with_label("To: ");
        
        // Define available volume units
        let units = [
            "Milliliter", "Cubic Centimeter", "Liter", "Cubic Meter",
            "Teaspoon", "Tablespoon", "Fluid Ounce", "Cup", "Pint",
            "Quart", "Gallon", "Cubic Inch", "Cubic Foot", "Cubic Yard"
        ];

        // Add units to both selectors
        for unit in units {
            from.add_choice(unit);
            to.add_choice(unit);
        }

        // Set default selections (Milliliter to Liter)
        from.set_value(0);
        to.set_value(2);
        
        (from, to)
    }
}