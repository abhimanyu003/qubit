#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn precedence_test() {
        assert_eq!(2_f64, parse("2/(3/3)"));
        assert_eq!(0.000000000001000088900582341_f64, parse("1 + 1e-12 - 1"));
        assert_eq!(4096_f64, parse("2^(3*4)"));
        assert_eq!(14_f64, parse("2*(3+4)"));
        assert_eq!(-14_f64, parse("2 - 2^3*2"));
        assert_eq!(22.5_f64, parse("6*3/4*5"));
        assert_eq!(10.666666666666666_f64, parse("2/3*4^2"));
        assert_eq!(8.666666666666666_f64, parse("1+2/3*4+5"));
    }

    #[test]
    fn addition() {
        assert_eq!(4_f64, parse("2+2"));
        assert_eq!(4_f64, parse("2 + 2"));
        assert_eq!(4_f64, parse("2 + +2"));
        assert_eq!(4_f64, parse("2 + (+2)"));
        assert_eq!(4_f64, parse("2 + (+1 +1)"));

        assert_eq!(4_f64, parse("2 with 2"));
        assert_eq!(4_f64, parse("2 plus 2"));
        assert_eq!(4_f64, parse("2 add 2"));
    }

    #[test]
    fn subtraction() {
        assert_eq!(0_f64, parse("2-2"));
        assert_eq!(0_f64, parse("2 - 2"));
        assert_eq!(4_f64, parse("2 - -2"));
        assert_eq!(4_f64, parse("2 - (-2)"));
        assert_eq!(4_f64, parse("2 - (-1 -1)"));

        assert_eq!(0_f64, parse("2 without 2"));
        assert_eq!(0_f64, parse("2 subtract 2"));
        assert_eq!(0_f64, parse("2 minus 2"));
    }

    #[test]
    fn multiplication() {
        assert_eq!(4_f64, parse("2 * 2"));
        assert_eq!(4_f64, parse("2 times 2"));
        assert_eq!(4_f64, parse("2 multiply by 2"));
        assert_eq!(4_f64, parse("2 mul 2"));
    }

    #[test]
    fn time() {
        assert_eq!(115.74074074074075_f64, parse("10000000000000000 nanoseconds to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000000 nanosecond to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000000 nanosecs to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000000 nanosec to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000000 ns to days"));

        assert_eq!(115.74074074074075_f64, parse("10000000000000 microseconds to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000 microsecond to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000 microsecs to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000 microsec to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000000 µs to days"));

        assert_eq!(115.74074074074075_f64, parse("10000000000 milliseconds to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000 millisecond to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000 millisecs to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000 millisec to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000000 ms to days"));

        assert_eq!(115.74074074074075_f64, parse("10000000 seconds to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000 second to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000 secs to days"));
        assert_eq!(115.74074074074075_f64, parse("10000000 sec to days"));

        assert_eq!(69.44444444444444_f64, parse("100000 minutes to days"));
        assert_eq!(69.44444444444444_f64, parse("100000 minute to days"));
        assert_eq!(69.44444444444444_f64, parse("100000 min to days"));

        assert_eq!(4.166666666666667_f64, parse("100 hours to days"));
        assert_eq!(4.166666666666667_f64, parse("100 hour to days"));
        assert_eq!(4.166666666666667_f64, parse("100 hrs to days"));
        assert_eq!(4.166666666666667_f64, parse("100 hr to days"));

        assert_eq!(100_f64, parse("100 days to days"));
        assert_eq!(100_f64, parse("100 day to days"));

        assert_eq!(700_f64, parse("100 weeks to days"));
        assert_eq!(700_f64, parse("100 week to days"));
        assert_eq!(700_f64, parse("100 wks to days"));
        assert_eq!(700_f64, parse("100 wk to days"));

        assert_eq!(30.436805555555555_f64, parse("1 months to days"));
        assert_eq!(30.436805555555555_f64, parse("1 month to days"));
        assert_eq!(30.436805555555555_f64, parse("1 mos to days"));
        assert_eq!(30.436805555555555_f64, parse("1 mo to days"));

        assert_eq!(36524.18981481482_f64, parse("100 years to days"));
        assert_eq!(36524.18981481482_f64, parse("100 year to days"));
        assert_eq!(36524.18981481482_f64, parse("100 yrs to days"));
        assert_eq!(36524.18981481482_f64, parse("100 yr to days"));

        assert_eq!(365241.89814814815_f64, parse("100 decades to days"));
        assert_eq!(365241.89814814815_f64, parse("100 decade to days"));

        assert_eq!(3652418.9814814813_f64, parse("100 centuries to days"));
        assert_eq!(3652418.9814814813_f64, parse("100 centry to days"));

        assert_eq!(36524219.90740741_f64, parse("100 milleniums to days"));
        assert_eq!(36524219.90740741_f64, parse("100 millenium to days"));
        assert_eq!(36524219.90740741_f64, parse("100 millenia to days"));

        assert_eq!(60_f64, parse("1 min to sec"));
        assert_eq!(1_f64, parse("60 sec to min"));
        assert_eq!(3600_f64, parse("1 hr to sec"));
    }

    #[test]
    fn area() {
        assert_eq!(0.01_f64, parse("100 metres2 to ha"));
        assert_eq!(0.01_f64, parse("100 metre2 to ha"));
        assert_eq!(0.01_f64, parse("100 meters2 to ha"));
        assert_eq!(0.01_f64, parse("100 meter2 to ha"));
        assert_eq!(0.01_f64, parse("100 sqm to ha"));
        assert_eq!(0.01_f64, parse("100 m2 to ha"));

        assert_eq!(100_f64, parse("0.01 hectare to m2"));
        assert_eq!(100_f64, parse("0.01 ha to m2"));

        assert_eq!(1000000_f64, parse("1 kilometres2 to m2"));
        assert_eq!(1000000_f64, parse("1 kilometre2 to m2"));
        assert_eq!(1000000_f64, parse("1 kilometers2 to m2"));
        assert_eq!(1000000_f64, parse("1 kilometer2 to m2"));
        assert_eq!(1000000_f64, parse("1 sqkm to m2"));
        assert_eq!(1000000_f64, parse("1 km2 to m2"));

        assert_eq!(0.064516_f64, parse("100 inches2 to m2"));
        assert_eq!(0.064516_f64, parse("100 inch2 to m2"));
        assert_eq!(0.064516_f64, parse("100 sqin to m2"));
        assert_eq!(0.064516_f64, parse("100 in2 to m2"));

        assert_eq!(9.290304_f64, parse("100 feet2 to m2"));
        assert_eq!(9.290304_f64, parse("100 foot2 to m2"));
        assert_eq!(9.290304_f64, parse("100 sqft to m2"));
        assert_eq!(9.290304_f64, parse("100 ft2 to m2"));

        assert_eq!(83.612736_f64, parse("100 yards2 to m2"));
        assert_eq!(83.612736_f64, parse("100 yard2 to m2"));
        assert_eq!(83.612736_f64, parse("100 sqyd to m2"));
        assert_eq!(83.612736_f64, parse("100 yd2 to m2"));

        assert_eq!(404685.64224_f64, parse("100 acre to m2"));
        assert_eq!(404685.64224_f64, parse("100 ac to m2"));

        assert_eq!(258998811.0336_f64, parse("100 miles2 to m2"));
        assert_eq!(258998811.0336_f64, parse("100 mile2 to m2"));
        assert_eq!(258998811.0336_f64, parse("100 sqmi to m2"));
        assert_eq!(258998811.0336_f64, parse("100 mi2 to m2"));

        assert_eq!(0.01_f64, parse("100 m2 to ha"));
        assert_eq!(1_f64, parse("100 ha to km2"));
        assert_eq!(0.64516_f64, parse("10000000 in2 to ha"));
    }

    #[test]
    fn temperate() {
        assert_eq!(212_f64, parse("100 c to f"));
        assert_eq!(212_f64, parse("100 C to F"));

        assert_eq!(310.9277777777778_f64, parse("100 f to k"));
        assert_eq!(310.9277777777778_f64, parse("100 F to K"));

        assert_eq!(373.15_f64, parse("100 c to k"));
        assert_eq!(373.15_f64, parse("100 C to K"));
    }

    #[test]
    fn mass() {
        assert_eq!(0.001_f64, parse("10000 μg to kg"));
        assert_eq!(0.001_f64, parse("10000 microgram to kg"));
        assert_eq!(0.001_f64, parse("10000 microgramme to kg"));
        assert_eq!(0.001_f64, parse("10000 mcg to kg"));

        assert_eq!(1_f64, parse("1000000 milligram to kg"));
        assert_eq!(1_f64, parse("1000000 mg to kg"));

        assert_eq!(1_f64, parse("1000 gram to kg"));
        assert_eq!(1_f64, parse("1000 g to kg"));

        // assert_eq!(1_f64, run("1 kilo to kg"));
        assert_eq!(1_f64, parse("1 kg to kg"));

        assert_eq!(10000000_f64, parse("10000 tonne to kg"));
        assert_eq!(10000000_f64, parse("10000 ton to kg"));

        assert_eq!(283.495_f64, parse("10000 ounce to kg"));

        assert_eq!(4535.92_f64, parse("10000 pound to kg"));

        assert_eq!(63502.9_f64, parse("10000 stone to kg"));

        assert_eq!(9071850_f64, parse("10000 short ton to kg"));

        assert_eq!(10160469.088_f64, parse("10000 long ton to kg"));

        assert_eq!(1000_f64, parse("1 kg to g"));
        assert_eq!(0.001_f64, parse("1 mg to g"));
    }

    #[test]
    fn speed() {
        assert_eq!(359.9997120002304_f64, parse("100 mps to kph"));
        assert_eq!(100_f64, parse("100 kph to kph"));
        assert_eq!(100_f64, parse("100 kmh to kph"));
        assert_eq!(109.72791221767022_f64, parse("100 fps to kph"));
        assert_eq!(160.934271252583_f64, parse("100 mph to kph"));
        assert_eq!(185.19969184024652_f64, parse("100 knots to kph"));
        assert_eq!(185.19969184024652_f64, parse("100 knot to kph"));
        assert_eq!(185.19969184024652_f64, parse("100 kn to kph"));
        assert_eq!(185.19969184024652_f64, parse("100 kt to kph"));
    }

    #[test]
    fn length() {
        assert_eq!(0.1_f64, parse("100 millimeters to meter"));
        assert_eq!(0.1_f64, parse("100 millimeter to meter"));
        assert_eq!(0.1_f64, parse("100 millimetre to meter"));
        assert_eq!(0.1_f64, parse("100 millimetres to meter"));
        assert_eq!(0.1_f64, parse("100 mm to meter"));

        assert_eq!(1_f64, parse("100 centimeters to meter"));
        assert_eq!(1_f64, parse("100 centimeter to meter"));
        assert_eq!(1_f64, parse("100 centimetres to meter"));
        assert_eq!(1_f64, parse("100 centimetre to meter"));
        assert_eq!(1_f64, parse("100 cm to meter"));

        assert_eq!(100_f64, parse("100 metres to meter"));
        assert_eq!(100_f64, parse("100 metre to meter"));
        assert_eq!(100_f64, parse("100 meters to meter"));
        assert_eq!(100_f64, parse("100 meter to meter"));
        assert_eq!(100_f64, parse("100 m to meter"));

        assert_eq!(100000_f64, parse("100 kilometers to meter"));
        assert_eq!(100000_f64, parse("100 kilometre to meter"));
        assert_eq!(100000_f64, parse("100 kilometres to meter"));
        assert_eq!(100000_f64, parse("100 kilometer to meter"));
        assert_eq!(100000_f64, parse("100 km to meter"));

        assert_eq!(2.54_f64, parse("100 inches to meter"));
        assert_eq!(2.54_f64, parse("100 inch to meter"));
        assert_eq!(2.54_f64, parse("100 in to meter"));

        assert_eq!(30.48_f64, parse("100 foot to meter"));
        assert_eq!(30.48_f64, parse("100 feet to meter"));
        assert_eq!(30.48_f64, parse("100 ft to meter"));

        assert_eq!(91.44_f64, parse("100 yards to meter"));
        assert_eq!(91.44_f64, parse("100 yard to meter"));
        assert_eq!(91.44_f64, parse("100 yd to meter"));

        assert_eq!(160934_f64, parse("100 miles to meter"));
        assert_eq!(160934_f64, parse("100 mile to meter"));
        assert_eq!(160934_f64, parse("100 mi to meter"));

        assert_eq!(185200_f64, parse("100 nautical mile to meter"));
        assert_eq!(185200_f64, parse("100 mni to meter"));
    }

    #[test]
    fn digital() {
        assert_eq!(0.12499968, parse("1024 bits to kb"));
        assert_eq!(0.12499968, parse("1024 bit to kb"));

        assert_eq!(1.000000512, parse("1024 bytes to kb"));
        assert_eq!(1.000000512, parse("1024 byte to kb"));

        assert_eq!(128.0, parse("1024 kilobits to kb"));
        assert_eq!(128.0, parse("1024 kilobit to kb"));
        assert_eq!(128.0, parse("1024 kbit to kb"));

        assert_eq!(1024.0, parse("1024 kilobytes to kb"));
        assert_eq!(1024.0, parse("1024 kilobyte to kb"));
        assert_eq!(1024.0, parse("1024 kb to kb"));

        assert_eq!(131072.0, parse("1024 megabits to kb"));
        assert_eq!(131072.0, parse("1024 megabit to kb"));
        assert_eq!(131072.0, parse("1024 mbit to kb"));

        assert_eq!(1048576.0, parse("1024 megabytes to kb"));
        assert_eq!(1048576.0, parse("1024 megabyte to kb"));
        assert_eq!(1048576.0, parse("1024 mb to kb"));

        assert_eq!(134217728.0, parse("1024 gigabits to kb"));
        assert_eq!(134217728.0, parse("1024 gigabit to kb"));
        assert_eq!(134217728.0, parse("1024 gbit to kb"));

        assert_eq!(1074176000.0, parse("1024 gigabytes to kb"));
        assert_eq!(1074176000.0, parse("1024 gigabyte to kb"));
        assert_eq!(1074176000.0, parse("1024 gb to kb"));

        assert_eq!(137420800000.0, parse("1024 terabits to kb"));
        assert_eq!(137420800000.0, parse("1024 terabit to kb"));
        assert_eq!(137420800000.0, parse("1024 tbit to kb"));

        assert_eq!(1099776000000.0, parse("1024 terabytes to kb"));
        assert_eq!(1099776000000.0, parse("1024 terabyte to kb"));
        assert_eq!(1099776000000.0, parse("1024 tb to kb"));

        assert_eq!(140697600000000.0, parse("1024 petabits to kb"));
        assert_eq!(140697600000000.0, parse("1024 petabit to kb"));
        assert_eq!(140697600000000.0, parse("1024 pbit to kb"));

        assert_eq!(1126400000000000.0, parse("1024 petabytes to kb"));
        assert_eq!(1126400000000000.0, parse("1024 petabyte to kb"));
        assert_eq!(1126400000000000.0, parse("1024 pb to kb"));
    }
}
