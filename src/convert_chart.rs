#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Temperature {
    KELVIN,
    CELSIUS,
    FAHRENHEIT,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Acceleration {
    METRE_PER_SECOND_SQUARED,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Angle {
    TURN,
    RADIAN,
    DEGREE,
    GRADIAN,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Length {
    MILLIMETRE,
    CENTIMETRE,
    METRE,
    KILOMETRE,
    INCH,
    FOOT,
    YARD,
    MILE,
    NAUTICAL_MILE,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mass {
    MICROGRAM,
    MILLIGRAM,
    GRAM,
    KILOGRAM,
    METRIC_TON,
    OUNCE,
    POUND,
    STONE,
    SHORT_TON,
    LONG_TON,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Time {
    NANOSECOND,
    MICROSECOND,
    MILLISECOND,
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    WEEK,
    MONTH,
    YEAR,
    DECADE,
    CENTURY,
    MILLENIUM,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Area {
    SQUARE_METRE,
    HECTARE,
    SQUARE_KILOMETRE,
    SQUARE_INCH,
    SQUARE_FEET,
    SQUARE_YARD,
    ACRE,
    SQUARE_MILE,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Speed {
    METRE_PER_SECOND,
    KILOMETRES_PER_HOUR,
    FEET_PER_SECOND,
    MILES_PER_HOUR,
    KNOT,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DigitalInformation {
    BIT,
    BYTE,
    KILOBIT,
    KILOBYTE,
    MEGABIT,
    MEGABYTE,
    GIGABIT,
    GIGABYTE,
    TERABIT,
    TERABYTE,
    PETABIT,
    PETABYTE,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnitType {
    TEMPERATURE(Temperature),
    ACCELERATION(Acceleration),
    ANGLE(Angle),
    LENGTH(Length),
    MASS(Mass),
    TIME(Time),
    AREA(Area),
    SPEED(Speed),
    DIGITALINFORMATION(DigitalInformation),
}

pub struct InvalidConversion;

pub fn convert(value: f64, from: UnitType, to: UnitType) -> f64 {
    if from == to {
        return value;
    }
    if std::mem::discriminant(&from) != std::mem::discriminant(&to) {
        return f64::NAN;
    }
    match (find_conversion_factor(from), find_conversion_factor(to)) {
        (Ok(from), Ok(to)) => value * from / to,
        (Err(from), Err(to)) => match (from, to) {
            (Temperature::KELVIN, Temperature::CELSIUS) => value - 273.15,
            (Temperature::KELVIN, Temperature::FAHRENHEIT) => value.mul_add(1.8, -459.67),
            (Temperature::KELVIN, Temperature::KELVIN) => value,
            // CELSIUS
            (Temperature::CELSIUS, Temperature::CELSIUS) => value,
            (Temperature::CELSIUS, Temperature::FAHRENHEIT) => value.mul_add(1.8, 32.),
            (Temperature::CELSIUS, Temperature::KELVIN) => value + 273.15,
            // FAHRENHEIT
            (Temperature::FAHRENHEIT, Temperature::CELSIUS) => (value - 32f64) / 1.8,
            (Temperature::FAHRENHEIT, Temperature::FAHRENHEIT) => value,
            (Temperature::FAHRENHEIT, Temperature::KELVIN) => (value + 459.67) * 5f64 / 9f64,
        },
        _ => f64::NAN,
    }
}

/// Finds conversion factor if applicable, otherwise return which
/// actual unit does not have a fixed conversion factor.
pub fn find_conversion_factor(u: UnitType) -> Result<f64, Temperature> {
    Ok(match u {
        UnitType::TEMPERATURE(v) => return Err(v),
        UnitType::ACCELERATION(v) => match v {
            Acceleration::METRE_PER_SECOND_SQUARED => 1_f64,
        },
        UnitType::ANGLE(v) => match v {
            Angle::TURN => 6.28318531,
            Angle::RADIAN => 1_f64,
            Angle::DEGREE => 0.0174532925,
            Angle::GRADIAN => 0.015707963267949,
        },
        UnitType::LENGTH(v) => match v {
            Length::MILLIMETRE => 0.001,
            Length::CENTIMETRE => 0.01,
            Length::METRE => 1_f64,
            Length::KILOMETRE => 1000_f64,
            Length::INCH => 0.0254,
            Length::FOOT => 0.3048,
            Length::YARD => 0.9144,
            Length::MILE => 1609.34,
            Length::NAUTICAL_MILE => 1852_f64,
        },
        UnitType::MASS(v) => match v {
            Mass::MICROGRAM => 1e-7_f64,
            Mass::MILLIGRAM => 1e-6_f64,
            Mass::GRAM => 0.001,
            Mass::KILOGRAM => 1_f64,
            Mass::METRIC_TON => 1000_f64,
            Mass::OUNCE => 0.0283495,
            Mass::POUND => 0.453592,
            Mass::STONE => 6.35029,
            Mass::SHORT_TON => 907.185,
            Mass::LONG_TON => 1016.0469088,
        },
        UnitType::TIME(v) => match v {
            Time::NANOSECOND => 1e-9,
            Time::MICROSECOND => 1e-6,
            Time::MILLISECOND => 0.001,
            Time::SECOND => 1_f64,
            Time::MINUTE => 60_f64,
            Time::HOUR => 3600_f64,
            Time::DAY => 86400_f64,
            Time::WEEK => 604800_f64,
            Time::MONTH => 2.62974e6,
            Time::YEAR => 3.15569e7,
            Time::DECADE => 3.15569e8,
            Time::CENTURY => 3.15569e9,
            Time::MILLENIUM => 3.1556926e10,
        },
        UnitType::AREA(v) => match v {
            Area::SQUARE_METRE => 1_f64,
            Area::HECTARE => 10000_f64,
            Area::SQUARE_KILOMETRE => 1000000_f64,
            Area::SQUARE_INCH => 0.00064516,
            Area::SQUARE_FEET => 0.09290304,
            Area::SQUARE_YARD => 0.83612736,
            Area::ACRE => 4046.8564224,
            Area::SQUARE_MILE => 2589988.110336,
        },
        UnitType::SPEED(v) => match v {
            Speed::METRE_PER_SECOND => 1_f64,
            Speed::KILOMETRES_PER_HOUR => 0.277778,
            Speed::FEET_PER_SECOND => 0.3048,
            Speed::MILES_PER_HOUR => 0.44704,
            Speed::KNOT => 0.514444,
        },
        UnitType::DIGITALINFORMATION(v) => match v {
            DigitalInformation::BIT => 0.00012207,
            DigitalInformation::BYTE => 0.000976563,
            DigitalInformation::KILOBIT => 0.125,
            DigitalInformation::KILOBYTE => 1_f64,
            DigitalInformation::MEGABIT => 128_f64,
            DigitalInformation::MEGABYTE => 1024_f64,
            DigitalInformation::GIGABIT => 131072_f64,
            DigitalInformation::GIGABYTE => 1.049e+6,
            DigitalInformation::TERABIT => 1.342e+8,
            DigitalInformation::TERABYTE => 1.074e+9,
            DigitalInformation::PETABIT => 1.374e+11,
            DigitalInformation::PETABYTE => 1.1e+12,
        },
    })
}

impl std::str::FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Temprature
            "TEMPERATURE::KELVIN" => Ok(UnitType::TEMPERATURE(Temperature::KELVIN)),
            "TEMPERATURE::FAHRENHEIT" => Ok(UnitType::TEMPERATURE(Temperature::FAHRENHEIT)),
            "TEMPERATURE::CELSIUS" => Ok(UnitType::TEMPERATURE(Temperature::CELSIUS)),
            // Angle
            "ACCELERATION::METRE_PER_SECOND_SQUARED" => Ok(UnitType::ACCELERATION(
                Acceleration::METRE_PER_SECOND_SQUARED,
            )),
            // Angle
            "ANGLE::TURN" => Ok(UnitType::ANGLE(Angle::TURN)),
            "ANGLE::RADIAN" => Ok(UnitType::ANGLE(Angle::RADIAN)),
            "ANGLE::DEGREE" => Ok(UnitType::ANGLE(Angle::DEGREE)),
            "ANGLE::GRADIAN" => Ok(UnitType::ANGLE(Angle::GRADIAN)),
            // Length
            "LENGTH::MILLIMETRE" => Ok(UnitType::LENGTH(Length::MILLIMETRE)),
            "LENGTH::CENTIMETRE" => Ok(UnitType::LENGTH(Length::CENTIMETRE)),
            "LENGTH::METRE" => Ok(UnitType::LENGTH(Length::METRE)),
            "LENGTH::KILOMETRE" => Ok(UnitType::LENGTH(Length::KILOMETRE)),
            "LENGTH::INCH" => Ok(UnitType::LENGTH(Length::INCH)),
            "LENGTH::FOOT" => Ok(UnitType::LENGTH(Length::FOOT)),
            "LENGTH::YARD" => Ok(UnitType::LENGTH(Length::YARD)),
            "LENGTH::MILE" => Ok(UnitType::LENGTH(Length::MILE)),
            "LENGTH::NAUTICAL_MILE" => Ok(UnitType::LENGTH(Length::NAUTICAL_MILE)),
            // Mass
            "MASS::MICROGRAM" => Ok(UnitType::MASS(Mass::MICROGRAM)),
            "MASS::MILLIGRAM" => Ok(UnitType::MASS(Mass::MILLIGRAM)),
            "MASS::GRAM" => Ok(UnitType::MASS(Mass::GRAM)),
            "MASS::KILOGRAM" => Ok(UnitType::MASS(Mass::KILOGRAM)),
            "MASS::METRIC_TON" => Ok(UnitType::MASS(Mass::METRIC_TON)),
            "MASS::OUNCE" => Ok(UnitType::MASS(Mass::OUNCE)),
            "MASS::POUND" => Ok(UnitType::MASS(Mass::POUND)),
            "MASS::STONE" => Ok(UnitType::MASS(Mass::STONE)),
            "MASS::SHORT_TON" => Ok(UnitType::MASS(Mass::SHORT_TON)),
            "MASS::LONG_TON" => Ok(UnitType::MASS(Mass::LONG_TON)),
            // Time
            "TIME::NANOSECOND" => Ok(UnitType::TIME(Time::NANOSECOND)),
            "TIME::MICROSECOND" => Ok(UnitType::TIME(Time::MICROSECOND)),
            "TIME::MILLISECOND" => Ok(UnitType::TIME(Time::MILLISECOND)),
            "TIME::SECOND" => Ok(UnitType::TIME(Time::SECOND)),
            "TIME::MINUTE" => Ok(UnitType::TIME(Time::MINUTE)),
            "TIME::HOUR" => Ok(UnitType::TIME(Time::HOUR)),
            "TIME::DAY" => Ok(UnitType::TIME(Time::DAY)),
            "TIME::WEEK" => Ok(UnitType::TIME(Time::WEEK)),
            "TIME::MONTH" => Ok(UnitType::TIME(Time::MONTH)),
            "TIME::YEAR" => Ok(UnitType::TIME(Time::YEAR)),
            "TIME::DECADE" => Ok(UnitType::TIME(Time::DECADE)),
            "TIME::CENTURY" => Ok(UnitType::TIME(Time::CENTURY)),
            "TIME::MILLENIUM" => Ok(UnitType::TIME(Time::MILLENIUM)),
            // Area
            "AREA::SQUARE_METRE" => Ok(UnitType::AREA(Area::SQUARE_METRE)),
            "AREA::HECTARE" => Ok(UnitType::AREA(Area::HECTARE)),
            "AREA::SQUARE_KILOMETRE" => Ok(UnitType::AREA(Area::SQUARE_KILOMETRE)),
            "AREA::SQUARE_INCH" => Ok(UnitType::AREA(Area::SQUARE_INCH)),
            "AREA::SQUARE_FEET" => Ok(UnitType::AREA(Area::SQUARE_FEET)),
            "AREA::SQUARE_YARD" => Ok(UnitType::AREA(Area::SQUARE_YARD)),
            "AREA::ACRE" => Ok(UnitType::AREA(Area::ACRE)),
            "AREA::SQUARE_MILE" => Ok(UnitType::AREA(Area::SQUARE_MILE)),

            // Speed
            "SPEED::METRE_PER_SECOND" => Ok(UnitType::SPEED(Speed::METRE_PER_SECOND)),
            "SPEED::KILOMETRES_PER_HOUR" => Ok(UnitType::SPEED(Speed::KILOMETRES_PER_HOUR)),
            "SPEED::FEET_PER_SECOND" => Ok(UnitType::SPEED(Speed::FEET_PER_SECOND)),
            "SPEED::MILES_PER_HOUR" => Ok(UnitType::SPEED(Speed::MILES_PER_HOUR)),
            "SPEED::KNOT" => Ok(UnitType::SPEED(Speed::KNOT)),

            // DigitalInformation
            "DIGITALINFORMATION::BIT" => Ok(UnitType::DIGITALINFORMATION(DigitalInformation::BIT)),
            "DIGITALINFORMATION::BYTE" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::BYTE))
            }
            "DIGITALINFORMATION::KILOBIT" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::KILOBIT))
            }
            "DIGITALINFORMATION::KILOBYTE" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::KILOBYTE))
            }
            "DIGITALINFORMATION::MEGABIT" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::MEGABIT))
            }
            "DIGITALINFORMATION::MEGABYTE" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::MEGABYTE))
            }
            "DIGITALINFORMATION::GIGABIT" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::GIGABIT))
            }
            "DIGITALINFORMATION::GIGABYTE" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::GIGABYTE))
            }
            "DIGITALINFORMATION::TERABIT" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::TERABIT))
            }
            "DIGITALINFORMATION::TERABYTE" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::TERABYTE))
            }
            "DIGITALINFORMATION::PETABIT" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::PETABIT))
            }
            "DIGITALINFORMATION::PETABYTE" => {
                Ok(UnitType::DIGITALINFORMATION(DigitalInformation::PETABYTE))
            }

            _ => Err(format!("'{}' is not a valid value for UnitType", s)),
        }
    }
}
