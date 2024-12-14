use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, opt},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

use crate::common::{
    nom_utils::{parser_i64, parser_u64},
    string_utils::read_file_to_string,
};

#[derive(Debug)]
struct ClawGame {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_claw_game(input: &str) -> IResult<&str, ClawGame> {
    let parser = terminated(
        tuple((
            map(
                tuple((
                    tag("Button A: X+"),
                    parser_i64,
                    tag(", Y+"),
                    parser_i64,
                    line_ending,
                )),
                |(_, x, _, y, _)| (x, y),
            ),
            map(
                tuple((
                    tag("Button B: X+"),
                    parser_i64,
                    tag(", Y+"),
                    parser_i64,
                    line_ending,
                )),
                |(_, x, _, y, _)| (x, y),
            ),
            map(
                tuple((
                    tag("Prize: X="),
                    parser_i64,
                    tag(", Y="),
                    parser_i64,
                    line_ending,
                )),
                |(_, x, _, y, _)| (x, y),
            ),
        )),
        opt(line_ending),
    )(input);

    match parser {
        Ok((remaining_input, (a, b, prize))) => Ok((
            remaining_input,
            ClawGame {
                a: a,
                b: b,
                prize: prize,
            },
        )),
        Err(e) => Err(e),
    }
}

fn eliminate(game: &ClawGame) -> Option<i64> {
    let firstWay = inner_eliminate(game.a, game.b, game.prize);
    let secondWay = inner_eliminate(game.b, game.a, game.prize);

    let firstTokens = match firstWay {
        None => None,
        Some((a, b)) => Some((a * 3) + b),
    };

    let secondTokens = match secondWay {
        None => None,
        Some((b, a)) => Some((a * 3) + b),
    };

    if firstTokens.is_some() && secondTokens.is_none() {
        return firstTokens;
    }

    if firstTokens.is_none() && secondTokens.is_some() {
        return secondTokens;
    }

    if firstTokens.is_none() && secondTokens.is_none() {
        return None;
    } else {
        if firstTokens.unwrap() > secondTokens.unwrap() {
            return secondTokens;
        } else {
            return firstTokens;
        }
    }
}

fn inner_eliminate(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> Option<(i64, i64)> {
    // eliminate b
    // bY (aX + bX = prizeX )
    let prizeX_bY = prize.0 * b.1;
    let aX_bY = a.0 * b.1;
    let bX_bY = b.0 * b.1;

    // -bX (aY + bY = prizeY)
    let prizeY__neg_bX = prize.1 * -b.0;
    let aY_neg_bX = a.1 * -b.0;
    let bY_neg_bX = b.1 * -b.0;

    // add to elimiate
    let prize_neg_b = prizeX_bY + prizeY__neg_bX;
    let aX = aX_bY + aY_neg_bX;

    // solve for a
    let aResult = prize_neg_b / aX;

    // println!("{}a = {}", aX, prize_neg_b);
    // println!("A = {}", a);

    // plug in for b
    // aX + bX = prizeX
    let aX = aResult * a.0;
    let prizeX_neg_aX = prize.0 - aX;

    let bResult = prizeX_neg_aX / b.0;

    // println!("B = {}", b);
    // confirm it actualy worked
    if ((aResult * a.0) + (bResult * b.0)) == prize.0
        && ((aResult * a.1) + (bResult * b.1)) == prize.1
    {
        // println!("Can't solve for A={},B={}", a, b);
        return Some((aResult, bResult));
    }

    return None;
}

pub fn run() {
    let input = read_file_to_string("input/day13.txt");
    let result = many0(parse_claw_game)(input.as_str());
    // println!("{:?}", result);

    // let mut a_count = 0;
    // let mut b_count = 0;
    let mut tokens = 0;
    for game in result.unwrap().1.iter() {
        tokens = tokens + eliminate(game).unwrap_or((0));
        // println!("a presses {}, b presses {}", a_press, b_press);
        // if a_press <= 100 && a_press > 0 && b_press <= 100 && b_press > 0 {
        //     a_count = a_count + a_press;
        //     b_count = b_count + b_press;
        // }
    }
    // println!("a presses {}, b presses {}", a_count, b_count);
    // let tokens = (a_count * 3) + b_count;
    println!("tokens: {}", tokens);
}

pub fn run2() {
    let input = read_file_to_string("input/day13.txt");
    let mut result = many0(parse_claw_game)(input.as_str());
    // println!("{:?}", result);

    // let mut a_count = 0;
    // let mut b_count = 0;
    let mut tokens = 0;
    for game in result.unwrap().1.iter_mut() {
        game.prize.0 = game.prize.0 + 10000000000000;
        game.prize.1 = game.prize.1 + 10000000000000;
        tokens = tokens + eliminate(game).unwrap_or((0));
        // println!("a presses {}, b presses {}", a_press, b_press);
        // if a_press <= 100 && a_press > 0 && b_press <= 100 && b_press > 0 {
        //     a_count = a_count + a_press;
        //     b_count = b_count + b_press;
        // }
    }
    // println!("a presses {}, b presses {}", a_count, b_count);
    // let tokens = (a_count * 3) + b_count;
    println!("tokens: {}", tokens);
}
#[cfg(test)]
mod test {
    use nom::multi::many0;

    use crate::day13::{eliminate, parse_claw_game};

    const TEST_DATA: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    const TEST_DATA_2: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
";

    #[test]
    pub fn test2() {
        let result = many0(parse_claw_game)(TEST_DATA_2);
        println!("{:?}", result);

        let mut tokens = 0;
        for game in result.unwrap().1.iter() {
            let tokens_used = eliminate(game).unwrap_or((0));
            println!("{}", tokens_used);
            tokens = tokens + tokens_used;
            // println!("a presses {}, b presses {}", a_press, b_press);
            // if a_press <= 100 && a_press > 0 && b_press <= 100 && b_press > 0 {
            //     a_count = a_count + a_press;
            //     b_count = b_count + b_press;
            // }
        }
        // println!("a presses {}, b presses {}", a_count, b_count);
        // let tokens = (a_count * 3) + b_count;
        println!("tokens: {}", tokens);
    }
    #[test]
    pub fn test1() {
        let result = many0(parse_claw_game)(TEST_DATA);
        println!("{:?}", result);

        let mut tokens = 0;
        for game in result.unwrap().1.iter() {
            tokens = tokens + eliminate(game).unwrap_or((0));
            // println!("a presses {}, b presses {}", a_press, b_press);
            // if a_press <= 100 && a_press > 0 && b_press <= 100 && b_press > 0 {
            //     a_count = a_count + a_press;
            //     b_count = b_count + b_press;
            // }
        }
        // println!("a presses {}, b presses {}", a_count, b_count);
        // let tokens = (a_count * 3) + b_count;
        println!("tokens: {}", tokens);
    }
}
