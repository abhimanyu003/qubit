use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

use crate::convert_chart::convert;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(modulus, Left),
            Operator::new(power, Right),
            Operator::new(percentOf, Left) | Operator::new(percentOn, Left),
            Operator::new(rightShift, Right) | Operator::new(leftShift, Right),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::convert => {
                let mut i = pair.into_inner();
                let value = i
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<f64>()
                    .unwrap();
                // Try to figure out rule name for the conversion between units
                // weight = kilo to gram
                // length = kilometer to meter
                let si_unit_type = i
                    .clone()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                let from = i
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                let to = i
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();

                convert(
                    value,
                    format!("{:?}", si_unit_type),
                    format!("{:?}", from),
                    format!("{:?}", to),
                )
            }
            Rule::function => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                let value = eval(i);
                apply_fun(name, value)
            }
            Rule::pi => std::f64::consts::PI,
            Rule::e => std::f64::consts::E,
            Rule::tau => std::f64::consts::TAU,
            Rule::num => pair.as_str().trim().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => f64::NAN,
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            Rule::percentOf => percent_of(lhs, rhs),
            Rule::percentOn => percent_on(lhs, rhs),
            Rule::rightShift => (lhs as i64 >> rhs as i64) as f64,
            Rule::leftShift => ((lhs as i64) << rhs as i64) as f64,
            Rule::modulus => (lhs % rhs) as f64,
            _ => f64::NAN,
        },
    )
}

fn percent_on(a: f64, b: f64) -> f64 {
    a / 100_f64 * b + b
}

fn percent_of(a: f64, b: f64) -> f64 {
    a / 100_f64 * b
}

fn apply_fun(name: &str, arg: f64) -> f64 {
    return match name {
        "sin" => arg.to_radians().sin(),
        "cos" => arg.to_radians().cos(),
        "tan" => arg.to_radians().tan(),
        "asin" => arg.asin(),
        "acos" => arg.cos(),
        "atan" => arg.atan(),
        "sinh" => arg.sinh(),
        "cosh" => arg.cosh(),
        "tanh" => arg.tanh(),
        "asinh" => arg.asinh(),
        "acosh" => arg.acosh(),
        "atanh" => arg.atanh(),
        "log" => arg.log10(),
        "sqrt" => arg.sqrt(),
        "cbrt" => arg.cbrt(),
        "round" => arg.round(),
        "ceil" => arg.ceil(),
        "floor" => arg.floor(),
        _ => f64::NAN,
    };
}

pub fn parse(input: &str) -> f64 {
    let parse_result = Calculator::parse(Rule::calculation, input);
    match parse_result {
        Ok(r) => eval(r),
        Err(_) => f64::NAN,
    }
}
