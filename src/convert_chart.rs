pub fn convert(value: f64, si_unit_type: String, from: String, to: String) -> f64 {
    if si_unit_type == "TEMPERATURE" {
        return match (from.as_str(), to.as_str()) {
            // KELVIN
            ("KELVIN", "CELSIUS") => value - 273.15,
            ("KELVIN", "FAHRENHEIT") => (value * 1.8) - 459.67,
            ("KELVIN", "KELVIN") => value,
            // CELSIUS
            ("CELSIUS", "CELSIUS") => value,
            ("CELSIUS", "FAHRENHEIT") => (value * 1.8) + 32f64,
            ("CELSIUS", "KELVIN") => value + 273.15,
            // FAHRENHEIT
            ("FAHRENHEIT", "CELSIUS") => (value - 32f64) / 1.8,
            ("FAHRENHEIT", "FAHRENHEIT") => value,
            ("FAHRENHEIT", "KELVIN") => (value + 459.67) * 5f64 / 9f64,
            _ => f64::NAN,
        };
    }
    (value * find_chart_value(si_unit_type.clone(), from)) / find_chart_value(si_unit_type, to)
}

fn find_chart_value(u: String, v: String) -> f64 {
    match u.as_str() {
        "ACCELERATION" => match v.as_str() {
            "METRE_PER_SECOND_SQUARED" => 1_f64,
            _ => f64::NAN,
        },
        "ANGLE" => match v.as_str() {
            "TURN" => 6.28318531,
            "RADIAN" => 1_f64,
            "DEGREE" => 0.0174532925,
            "GRADIAN" => 0.015707963267949,
            _ => f64::NAN,
        },
        "LENGTH" => match v.as_str() {
            "MILLIMETRE" => 0.001,
            "CENTIMETRE" => 0.01,
            "METRE" => 1_f64,
            "KILOMETRE" => 1000_f64,
            "INCH" => 0.0254,
            "FOOT" => 0.3048,
            "YARD" => 0.9144,
            "MILE" => 1609.34,
            "NAUTICAL_MILE" => 1852_f64,
            _ => f64::NAN,
        },
        "MASS" => match v.as_str() {
            "MICROGRAM" => 1e-7_f64,
            "MILLIGRAM" => 1e-6_f64,
            "GRAM" => 0.001,
            "KILOGRAM" => 1_f64,
            "METRIC_TON" => 1000_f64,
            "OUNCE" => 0.0283495,
            "POUND" => 0.453592,
            "STONE" => 6.35029,
            "SHORT_TON" => 907.185,
            "LONG_TON" => 1016.0469088,
            _ => f64::NAN,
        },
        "TIME" => match v.as_str() {
            "NANOSECOND" => 1e-9,
            "MICROSECOND" => 1e-6,
            "MILLISECOND" => 0.001,
            "SECOND" => 1_f64,
            "MINUTE" => 60_f64,
            "HOUR" => 3600_f64,
            "DAY" => 86400_f64,
            "WEEK" => 604800_f64,
            "MONTH" => 2.62974e6,
            "YEAR" => 3.15569e7,
            "DECADE" => 3.15569e8,
            "CENTURY" => 3.15569e9,
            "MILLENIUM" => 3.1556926e10,
            _ => f64::NAN,
        },
        "AREA" => match v.as_str() {
            "SQUARE_METRE" => 1_f64,
            "HECTARE" => 10000_f64,
            "SQUARE_KILOMETRE" => 1000000_f64,
            "SQUARE_INCH" => 0.00064516,
            "SQUARE_FEET" => 0.09290304,
            "SQUARE_YARD" => 0.83612736,
            "ACRE" => 4046.8564224,
            "SQUARE_MILE" => 2589988.110336,
            _ => f64::NAN,
        },
        "SPEED" => match v.as_str() {
            "METRE_PER_SECOND" => 1_f64,
            "KILOMETRES_PER_HOUR" => 0.277778,
            "FEET_PER_SECOND" => 0.3048,
            "MILES_PER_HOUR" => 0.44704,
            "KNOT" => 0.514444,
            _ => f64::NAN,
        },
        "DIGITALINFORMATION" => match v.as_str() {
            "BIT" => 0.00012207,
            "BYTE" => 0.000976563,
            "KILOBIT" => 0.125,
            "KILOBYTE" => 1_f64,
            "MEGABIT" => 128_f64,
            "MEGABYTE" => 1024_f64,
            "GIGABIT" => 131072_f64,
            "GIGABYTE" => 1.049e+6,
            "TERABIT" => 1.342e+8,
            "TERABYTE" => 1.074e+9,
            "PETABIT" => 1.374e+11,
            "PETABYTE" => 1.1e+12,
            // "EXABIT" => 1152921504606846976 _f64,
            // "EXABYTE" => 9223372036854775808_f64,
            // "ZETTABIT" => 1180591620717411303424_f64,
            // "ZETTABYTE" => 9444732965739290427392_f64,
            // "YOTTABIT" => 1208925819614629174706176_f64,
            // "YOTTABYTE" => 9671406556917033397649408_f64,
            _ => f64::NAN,
        },
        _ => f64::NAN,
    }
}
