use std::{collections::HashMap, fs};

#[derive(Debug, Default, Clone, Copy)]
struct Page {
    source: usize,
    destination: usize,
    range: usize,
}
impl Page {
    fn new(source: usize, destination: usize,range: usize) -> Self {
        Self{
            source,destination,range, 
        }
    }
    
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
enum Index {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn main() {
    let input = fs::read_to_string("input/mini.txt").expect(
        "Unable to find
    input",
    );
    let mut almanac: HashMap<Index,Vec<Page>> = HashMap::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut seeds_part_two: Vec<usize> = Vec::new();
    let mut guide:Index = Index::None;
    for line in input.lines() {
        match line {
            x if x.contains("seeds") => {
                seeds.extend(x.split_ascii_whitespace().filter_map(|f| f.parse::<usize>().ok()).collect::<Vec<usize>>());
            },
            x if x.contains("-to-") => {
                guide =  match x.split(' ').next().unwrap() {
                    "seed-to-soil" => Index::SeedToSoil,
                    "soil-to-fertilizer" => Index::SoilToFertilizer,
                    "fertilizer-to-water" => Index::FertilizerToWater,
                    "water-to-light" => Index::WaterToLight,
                    "light-to-temperature" => Index::LightToTemperature,
                    "temperature-to-humidity" => Index::TemperatureToHumidity,
                    "humidity-to-location" => Index::HumidityToLocation,
                    _ => panic!("Error did not match the string"),

                };
            },
            x if x.split_ascii_whitespace().collect::<Vec<_>>().len() == 3 => {
                let nums = x.split_ascii_whitespace().map(|f|
                f.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                let tuple:Page = Page::new(nums[1], nums[0], nums[2]);
                almanac.entry(guide).and_modify(|f|
                f.push(tuple)).or_insert(vec![tuple]);
            }
            _ => continue,
        };
    }
    let first = first_part(&almanac, &seeds);
    println!("First part is {first}");
    for range in seeds.chunks(2) {
        for value in range[0]..(range[0] + range[1]) {
            seeds_part_two.push(value);
        }
    }
    let second = first_part(&almanac, &seeds_part_two);
    println!("Second part is {second}");
}

fn first_part(almanac: &HashMap<Index,Vec<Page>>, seeds: &Vec<usize>) -> usize {
    let mut smallest_location = usize::MAX;
    for seed in seeds {
        let mut location = seed.clone();
        for method in [&Index::SeedToSoil, &Index::SoilToFertilizer,
        &Index::FertilizerToWater, &Index::WaterToLight,
        &Index::LightToTemperature, &Index::TemperatureToHumidity,
        &Index::HumidityToLocation] {
            location = get_next_number(almanac, method, location);
        }
        if location < smallest_location {
            smallest_location = location;
        }
    }
   smallest_location 
}

fn get_next_number(almanac: &HashMap<Index,Vec<Page>>, index:&Index, seed: usize) -> usize {
    // Cases : soil is not mapped: return input, soil is explicitely mapped: mapped
    let current_page = almanac.get(index).unwrap();
   // println!("{:?}",current_page);
    let chosen = current_page.iter().filter(|f|
    (f.source..(f.source+f.range)).contains(&seed)).collect::<Vec<_>>();
    if chosen.is_empty(){
        return seed
    } else {
        return (seed - chosen[0].source) + chosen[0].destination;

    }
    
}

//fn first_part(&input) {
//    todo!()
//}
//
//fn second_part(&input) {
//    todo!()
//}
