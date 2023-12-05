use std::{fmt::Display, collections::HashMap};

fn main() {
	println!("Advent of Code 2023");

	let input = include_str!("day2_input.txt");

	let result = match day2_pt1(input) {
		Ok(result) => result.to_string(),
		Err(err) => err.to_string(),
	};

	println!("Day 2 (part 1): {result}");

	let result = match day2_pt2(input) {
		Ok(result) => result.to_string(),
		Err(err) => err.to_string(),
	};

	println!("Day 2 (part 2): {result}");
}

fn day2_pt1(input: &str) -> Result<usize, Error> {
	let bag: HashMap<_, _> = [
		(Colour::Red, 12),
		(Colour::Green, 13),
		(Colour::Blue, 14),
	].into_iter().collect();

	let games = input.lines().map(parse).collect::<Result<Vec<_>, Error>>()?;

	let possible_games_sum: usize = games.iter().filter(|game| {
		game.cubes.iter().all(|cubes| {
			cubes.iter().all(|cube| {
				if let Some(bag_count) = bag.get(&cube.0) {
					*bag_count >= cube.1
				} else {
					false
				}
			})
		})
	})
	.map(|game| game.id)
	.sum();

	Ok(possible_games_sum)
}

fn day2_pt2(input: &str) -> Result<usize, Error> {
	let games = input.lines().map(parse).collect::<Result<Vec<_>, Error>>()?;

	let power_sum: usize = games.iter().map(|game| {
		let mut max_values: HashMap<Colour, usize> = HashMap::new();
		game.cubes.iter().for_each(|draw| {
			draw.iter().for_each(|cube| {
				if let Some(val) = max_values.get(&cube.0) {
					max_values.insert(cube.0, *val.max(&cube.1));
				} else {
					max_values.insert(cube.0, cube.1);
				}
			})
		});
		if max_values.is_empty() {
			0
		} else {
			let mut power = 1;
			max_values.iter().for_each(|(_, val)| power *= val);
			power
		}
	})
	.sum();

	Ok(power_sum)
}

fn parse(line: &str) -> Result<Game, Error> {
	let parts: Vec<_> = line.split(':').collect();
	if parts.len() != 2 {
		return Err(Error::WrongNumberOfParts)
	}
	let id: usize = parts[0].strip_prefix("Game ")
		.ok_or(Error::InvalidId)?
		.parse()
		.map_err(|err| Error::Other(Box::new(err)))?;

	let cubes = parts[1].split(';').map(|group| {
		group.split(',').map(|cubes| {
			let cubes: Vec<_> = cubes.split(' ').filter(|c| !c.is_empty()).collect();
			if cubes.len() != 2 {
				Err(Error::WrongNumberOfParts)
			} else {
				let colour = match cubes[1] {
					"blue" => Ok(Colour::Blue),
					"red" => Ok(Colour::Red),
					"green" => Ok(Colour::Green),
					_ => Err(Error::InvalidColour)
				};
				let count: Result<usize, Error> = str::parse(cubes[0])
					.map_err(|err| Error::Other(Box::new(err)));

				match (colour, count) {
					(Ok(colour), Ok(count)) => Ok((colour, count)),
					(Err(err), _) => Err(err),
					(_, Err(err)) => Err(err),
				}
			}
		}).collect::<Result<Vec<_>, Error>>()
	}).collect::<Result<Vec<_>, Error>>()?;

	Ok(Game { id, cubes })
}

#[derive(Debug)]
struct Game {
	id: usize,
	cubes: Vec<Vec<(Colour, usize)>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Colour {
	Blue,
	Red,
	Green
}

#[derive(Debug)]
enum Error {
	Other(Box<dyn std::error::Error>),
	WrongNumberOfParts,
	InvalidId,
	InvalidColour,
}

impl std::error::Error for Error { }

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(err) => err.fmt(f),
            Error::WrongNumberOfParts => write!(f, "Wrong number of parts"),
            Error::InvalidId => write!(f, "Invalid id"),
			Error::InvalidColour => write!(f, "Invalid colour"),
        }
    }
}


