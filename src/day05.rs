use std::fs::read_to_string;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, i64 as parse_i64, newline};
use nom::{IResult, Parser};
use nom::error::Error;
use nom::multi::{many0, separated_list0, separated_list1};
use nom::number::complete::le_i64;
use nom::sequence::{pair, preceded, tuple};
use crate::helpers;

pub fn part_1() -> i64 {
    read_from("src/input/day05.txt")
}

pub fn part_2() -> i64 {
    read_from_v2("src/input/day05.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = read_to_string(filepath).unwrap();
    let almanach = parse_maps(sample.as_str());
    let min = almanach.seeds.iter().filter_map(|&seed| {
        // println!("seeds {:?}", seed);
        let soil = map_source_to_dest(
            seed,
            almanach.sts_map,
        );
        if let Some(soil) = soil {
            // println!("soil {:?}", soil);
            let fertilizer = map_source_to_dest(
                soil,
                almanach.stf_map,
            );
            if let Some(fertilizer) = fertilizer {
                // println!("ferr {:?}", fertilizer);
                let water = map_source_to_dest(
                    fertilizer,
                    almanach.ftw_map,
                );
                if let Some(water) = water {
                    // println!("water {:?}", water);
                    let light = map_source_to_dest(
                        water,
                        almanach.wtl_map,
                    );
                    if let Some(light) = light {
                        // println!("light {:?}", light);
                        let temp = map_source_to_dest(
                            light,
                            almanach.ltt_map,
                        );
                        if let Some(temp) = temp {
                            // println!("temp {:?}", temp);
                            let humidity = map_source_to_dest(
                                temp,
                                almanach.tth_map,
                            );
                            if let Some(humidity) = humidity {
                                // println!("humidi {:?}", humidity);
                                let location = map_source_to_dest(
                                    humidity,
                                    almanach.htl_map,
                                );
                                // println!("locatione {:?}", location);
                                return location;
                            }
                        }
                    }
                }
            }
        } else {
            println!("no soil");
        }
        None
    }).min().unwrap_or(0);
    min
}

fn map_source_to_dest(id: i64, mapping: Vec<(i64, i64, i64)>) -> Option<i64> {
    // println!("looking for id:{:?} in {:?}", id, mapping);
    mapping.iter().find_map(|(destination, source, range)| {
        if (source <= &id) && ((source + range) > id) {
            let res = destination + (id - source);
            // println!("mapping:{:?} to {}, because between {} and {}", id, res, source, (source + range));
            Some(res)
        } else {
            None
        }
    }).or(Some(id))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, seeds): (&str, Vec<i64>) = preceded(tag("seeds: "), separated_list1(tag(" "), parse_i64)).parse(input)?;
    Ok((input, seeds))
}

fn parse_seeds_v2(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list0(tag(" "), pair(parse_i64, preceded(tag(" "), parse_i64)))).parse(input)?;
    Ok((input, seeds))
}

fn parse_seed_to_soil_block(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("seed-to-soil map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

// fn parse_generic_block<'a>(tag_name: &'a str) -> impl FnMut {
//     return move |input: &'a str| -> IResult<&'a str, Vec<(&str,&str,&str)>> {
//         let (input, mapping) = preceded(tag(tag_name), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
//         Ok((input, mapping))
//     };
// }

fn parse_soil_to_fertilizer(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("soil-to-fertilizer map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

fn parse_fertilizer_to_water(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("fertilizer-to-water map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

fn parse_water_to_light(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("water-to-light map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

fn parse_light_to_temperature(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("light-to-temperature map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

fn parse_temperature_to_humidity(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("temperature-to-humidity map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

fn parse_humidity_to_location(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let (input, mapping) = preceded(tag("humidity-to-location map:"), preceded(newline, separated_list1(newline, parse_conversion_map))).parse(input)?;
    Ok((input, mapping))
}

fn parse_conversion_map(input: &str) -> IResult<&str, (i64, i64, i64)> {
    let (input, conversion_map) = tuple((parse_i64, preceded(tag(" "), parse_i64), preceded(tag(" "), parse_i64))).parse(input)?;
    Ok((input, conversion_map))
}

struct Almanach {
    seeds: Vec<i64>,
    sts_map: Vec<(i64, i64, i64)>,
    stf_map: Vec<(i64, i64, i64)>,
    ftw_map: Vec<(i64, i64, i64)>,
    wtl_map: Vec<(i64, i64, i64)>,
    ltt_map: Vec<(i64, i64, i64)>,
    tth_map: Vec<(i64, i64, i64)>,
    htl_map: Vec<(i64, i64, i64)>,
}

fn parse_maps(input: &str) -> Almanach {
    // separated_list0(preceded(tuple(newline, newline)), pa
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, sts_map) = preceded(many0(newline), parse_seed_to_soil_block).parse(input).unwrap();
    let (input, stf_map) = preceded(many0(newline), parse_soil_to_fertilizer).parse(input).unwrap();
    let (input, ftw_map) = preceded(many0(newline), parse_fertilizer_to_water).parse(input).unwrap();
    let (input, wtl_map) = preceded(many0(newline), parse_water_to_light).parse(input).unwrap();
    let (input, ltt_map) = preceded(many0(newline), parse_light_to_temperature).parse(input).unwrap();
    let (input, tth_map) = preceded(many0(newline), parse_temperature_to_humidity).parse(input).unwrap();
    let (input, htl_map) = preceded(many0(newline), parse_humidity_to_location).parse(input).unwrap();

    Almanach {
        seeds,
        sts_map,
        stf_map,
        ftw_map,
        wtl_map,
        ltt_map,
        tth_map,
        htl_map,
    }
}

fn parse_maps_v2(input: &str) -> Almanach {
    // separated_list0(preceded(tuple(newline, newline)), pa
    let (input, seeds) = parse_seeds_v2(input).unwrap();
    let (input, sts_map) = preceded(many0(newline), parse_seed_to_soil_block).parse(input).unwrap();
    let (input, stf_map) = preceded(many0(newline), parse_soil_to_fertilizer).parse(input).unwrap();
    let (input, ftw_map) = preceded(many0(newline), parse_fertilizer_to_water).parse(input).unwrap();
    let (input, wtl_map) = preceded(many0(newline), parse_water_to_light).parse(input).unwrap();
    let (input, ltt_map) = preceded(many0(newline), parse_light_to_temperature).parse(input).unwrap();
    let (input, tth_map) = preceded(many0(newline), parse_temperature_to_humidity).parse(input).unwrap();
    let (input, htl_map) = preceded(many0(newline), parse_humidity_to_location).parse(input).unwrap();

    let range_seed = seeds.iter().map(|&(start, range)| {
        let end_of_range = start + range;
        let res: Vec<i64> = (start..end_of_range).map(|e| e).collect();
        res
    }).flatten().collect();
    Almanach {
        seeds: range_seed,
        sts_map,
        stf_map,
        ftw_map,
        wtl_map,
        ltt_map,
        tth_map,
        htl_map,
    }
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = read_to_string(filepath).unwrap();
    let almanach = parse_maps_v2(sample.as_str());
    let min = almanach.seeds.iter().filter_map(|&seed| {
        // println!("seeds {:?}", seed);
        let soil = map_source_to_dest(
            seed,
            almanach.sts_map.clone(),
        );
        if let Some(soil) = soil {
            // println!("soil {:?}", soil);
            let fertilizer = map_source_to_dest(
                soil,
                almanach.stf_map.clone(),
            );
            if let Some(fertilizer) = fertilizer {
                // println!("ferr {:?}", fertilizer);
                let water = map_source_to_dest(
                    fertilizer,
                    almanach.ftw_map.clone(),
                );
                if let Some(water) = water {
                    // println!("water {:?}", water);
                    let light = map_source_to_dest(
                        water,
                        almanach.wtl_map.clone(),
                    );
                    if let Some(light) = light {
                        // println!("light {:?}", light);
                        let temp = map_source_to_dest(
                            light,
                            almanach.ltt_map.clone(),
                        );
                        if let Some(temp) = temp {
                            // println!("temp {:?}", temp);
                            let humidity = map_source_to_dest(
                                temp,
                                almanach.tth_map.clone(),
                            );
                            if let Some(humidity) = humidity {
                                // println!("humidi {:?}", humidity);
                                let location = map_source_to_dest(
                                    humidity,
                                    almanach.htl_map.clone(),
                                );
                                // println!("locatione {:?}", location);
                                return location;
                            }
                        }
                    }
                }
            }
        } else {
            println!("no soil");
        }
        None
    }).min().unwrap_or(0);
    min
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_1() {
        let res = part_1();
        assert_eq!(res, 309796150);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample05.txt");
        assert_eq!(res, 35);
    }

    #[test]
    fn test_parse_seeds() {
        let res = parse_seeds("seeds: 79 14 55 13").unwrap();
        assert_eq!(res.1, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_seed_to_soil_block() {
        let res = parse_seed_to_soil_block("seed-to-soil map:
50 98 2
52 50 48").unwrap();
        assert_eq!(res.1, vec![(50, 98, 2), (52, 50, 48)]);
    }

    #[test]
    fn test_parse_conversion_map() {
        let res = parse_conversion_map("79 14 55").unwrap();
        assert_eq!(res.1, (79, 14, 55));
    }

    #[test]
    fn test_map_source_to_dest() {
        let res = map_source_to_dest(16, vec![(50, 15, 2)]);
        assert_eq!(res, Some(51));
    }

    #[test]
    fn test_parse_seeds_v2() {
        let res = parse_seeds_v2("seeds: 79 14 55 13").unwrap();
        assert_eq!(res.1, vec![(79, 14), (55, 13)]);
    }


    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample05.txt");
        assert_eq!(res, 46);
    }


    #[test]
    fn test_day5_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}