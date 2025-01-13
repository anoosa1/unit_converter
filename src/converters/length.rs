pub fn convert_length(value: f64, from: usize, to: usize) -> f64 {
    // Convert to meters first
    let meters = match from {
        0 => value * 1e-10,    // Angstrom to meters
        1 => value * 1e-9,     // Nanometer to meters
        2 => value * 1e-6,     // Micron to meters
        3 => value * 1e-3,     // Millimeter to meters
        4 => value * 1e-2,     // Centimeter to meters
        5 => value * 1.0,      // Meter to meters
        6 => value * 1e3,      // Kilometer to meters
        7 => value * 0.0254,   // Inch to meters
        8 => value * 0.3048,   // Foot to meters
        9 => value * 0.9144,   // Yard to meters
        10 => value * 1609.344,// Mile to meters
        _ => value,
    };
    
    // Convert from meters to target unit
    match to {
        0 => meters * 1e10,    // Meters to Angstrom
        1 => meters * 1e9,     // Meters to Nanometer
        2 => meters * 1e6,     // Meters to Micron
        3 => meters * 1e3,     // Meters to Millimeter
        4 => meters * 1e2,     // Meters to Centimeter
        5 => meters * 1.0,     // Meters to Meter
        6 => meters * 1e-3,    // Meters to Kilometer
        7 => meters / 0.0254,  // Meters to Inch
        8 => meters / 0.3048,  // Meters to Foot
        9 => meters / 0.9144,  // Meters to Yard
        10 => meters / 1609.344, // Meters to Mile
        _ => meters,
    }
} 