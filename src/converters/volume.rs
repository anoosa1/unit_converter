pub fn convert_volume(value: f64, from: usize, to: usize) -> f64 {
    // Convert to liters first
    let liters = match from {
        0 => value * 0.001,        // Milliliter to liters
        1 => value * 0.001,        // Cubic Centimeter to liters
        2 => value * 1.0,          // Liter to liters
        3 => value * 1000.0,       // Cubic Meter to liters
        4 => value * 0.00492892,   // Teaspoon to liters
        5 => value * 0.0147868,    // Tablespoon to liters
        6 => value * 0.0295735,    // Fluid Ounce to liters
        7 => value * 0.236588,     // Cup to liters
        8 => value * 0.473176,     // Pint to liters
        9 => value * 0.946353,     // Quart to liters
        10 => value * 3.78541,     // Gallon to liters
        11 => value * 0.0163871,   // Cubic Inch to liters
        12 => value * 28.3168,     // Cubic Foot to liters
        13 => value * 764.555,     // Cubic Yard to liters
        _ => value,
    };
    
    // Convert from liters to target unit
    match to {
        0 => liters * 1000.0,      // Liters to Milliliter
        1 => liters * 1000.0,      // Liters to Cubic Centimeter
        2 => liters * 1.0,         // Liters to Liter
        3 => liters * 0.001,       // Liters to Cubic Meter
        4 => liters / 0.00492892,  // Liters to Teaspoon
        5 => liters / 0.0147868,   // Liters to Tablespoon
        6 => liters / 0.0295735,   // Liters to Fluid Ounce
        7 => liters / 0.236588,    // Liters to Cup
        8 => liters / 0.473176,    // Liters to Pint
        9 => liters / 0.946353,    // Liters to Quart
        10 => liters / 3.78541,    // Liters to Gallon
        11 => liters / 0.0163871,  // Liters to Cubic Inch
        12 => liters / 28.3168,    // Liters to Cubic Foot
        13 => liters / 764.555,    // Liters to Cubic Yard
        _ => liters,
    }
} 