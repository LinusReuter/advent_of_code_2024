advent_of_code::solution!(12);

#[derive(Debug)]
struct Region {
    label: char,
    replaced_by: Option<usize>,
    area: u64,
    perimeter: i64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut regions: Vec<Region> = vec![]; // all the regions
    let mut representatives: Vec<usize> = vec![]; // mapping of representatives to region index. Multiple representatives can point to the same region.

    let dimensions = (input.lines().next()?.len(), input.lines().count());
    let mut representatives_grid: Vec<Vec<usize>> = vec![vec![0; dimensions.0]; dimensions.1];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut new_perimeter: i64 = 4;
            let mut representativ: Option<usize> = None;

            if y > 0 {
                let representative_idx = representatives_grid[y - 1][x];
                let mut upper_region_idx = representatives[representative_idx];
                let mut upper_region = &regions[upper_region_idx];
                if upper_region.label == c {
                    while upper_region.replaced_by.is_some() {
                        upper_region_idx = upper_region.replaced_by.unwrap();
                        upper_region = &regions[upper_region_idx];
                    }
                    representatives[representative_idx] = upper_region_idx;
                    representativ = Some(representative_idx);
                    new_perimeter -= 2;
                }
            }
            if x > 0 {
                let representative_idx = representatives_grid[y][x - 1];
                let left_region = &regions[representatives[representative_idx]];
                if left_region.label == c {
                    new_perimeter -= 2;

                    if representativ.is_some() {
                        let left_representative = representatives_grid[y][x - 1];
                        let upper_representative = representativ.unwrap();
                        let left_region_idx = representatives[left_representative];
                        let upper_region_idx = representatives[upper_representative];

                        if left_region_idx != upper_region_idx {
                            // merge regions
                            let (left_region, upper_region): (&mut Region, &mut Region) = unsafe {
                                let ptr = regions.as_mut_ptr();
                                (
                                    &mut *ptr.add(left_region_idx),
                                    &mut *ptr.add(upper_region_idx),
                                )
                            };

                            upper_region.replaced_by = Some(left_region_idx);
                            left_region.area += upper_region.area;
                            left_region.perimeter += upper_region.perimeter;
                            representatives[upper_representative] = left_region_idx;
                        }
                    } else {
                        representativ = Some(representatives_grid[y][x - 1]);
                    }
                }
            }
            if representativ.is_none() {
                let region = Region {
                    label: c,
                    replaced_by: None,
                    area: 1,
                    perimeter: new_perimeter,
                };
                regions.push(region);
                representatives.push(regions.len() - 1);
                representativ = Some(representatives.len() - 1);
            } else {
                let region = regions
                    .get_mut(representatives[representativ.unwrap()])
                    .unwrap();
                region.area += 1;
                region.perimeter += new_perimeter;
            }
            representatives_grid[y][x] = representativ.unwrap();
        }
    }

    // sum of areas * perimeters of all regions that are not deprecated
    Some(
        regions
            .iter()
            .filter(|r| r.replaced_by.is_none())
            .map(|r| r.area * r.perimeter as u64)
            .sum(),
    )
}

#[derive(Debug, Clone, Copy)]
struct RegionP2 {
    label: char,
    replaced_by: Option<usize>,
    area: u64,
    sides: i64,
}

#[derive(Debug)]
enum Neighbor {
    None,
    Wall,
    Other,
    SameRegion(usize),
}

#[derive(Debug)]
struct Neighbors {
    top_left: Neighbor,
    top: Neighbor,
    top_right: Neighbor,
    left: Neighbor,
}

fn get_neighbor(
    x: isize,
    y: isize,
    c: char,
    dimensions: (usize, usize),
    representatives_grid: &[Vec<usize>],
    representatives: &mut [usize],
    regions: &mut [RegionP2],
) -> Neighbor {
    if x >= dimensions.0 as isize || y >= dimensions.1 as isize || x < 0 || y < 0 {
        return Neighbor::Wall;
    }
    let representative_idx = representatives_grid[y as usize][x as usize];
    let mut region_idx = representatives[representative_idx];
    let mut region = &regions[region_idx];
    if region.label == c {
        while region.replaced_by.is_some() {
            region_idx = region.replaced_by.unwrap();
            region = &regions[region_idx];
        }
        representatives[representative_idx] = region_idx;
        return Neighbor::SameRegion(region_idx);
    }
    Neighbor::Other
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut regions: Vec<RegionP2> = vec![]; // all the regions
    let mut representatives: Vec<usize> = vec![]; // mapping of representatives to region index. Multiple representatives can point to the same region.

    let dimensions = (input.lines().next()?.len(), input.lines().count());
    let mut representatives_grid: Vec<Vec<usize>> = vec![vec![0; dimensions.0]; dimensions.1];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut delta_sides: i64 = 0;
            let mut representativ: Option<usize> = None;

            let mut neighbors = Neighbors {
                top_left: Neighbor::None,
                top: Neighbor::None,
                top_right: Neighbor::None,
                left: Neighbor::None,
            };

            // Set all neighbors for the current cell
            neighbors.top_left = get_neighbor(
                x as isize - 1,
                y as isize - 1,
                c,
                dimensions,
                &representatives_grid,
                &mut representatives,
                &mut regions,
            );
            neighbors.top = get_neighbor(
                x as isize,
                y as isize - 1,
                c,
                dimensions,
                &representatives_grid,
                &mut representatives,
                &mut regions,
            );
            neighbors.top_right = get_neighbor(
                x as isize + 1,
                y as isize - 1,
                c,
                dimensions,
                &representatives_grid,
                &mut representatives,
                &mut regions,
            );
            neighbors.left = get_neighbor(
                x as isize - 1,
                y as isize,
                c,
                dimensions,
                &representatives_grid,
                &mut representatives,
                &mut regions,
            );

            // calculate the number of sides my matching neighbors combinations
            match neighbors.top {
                Neighbor::Other | Neighbor::Wall => {
                    if let Neighbor::SameRegion(_) = neighbors.left {
                        if let Neighbor::SameRegion(_) = neighbors.top_left {
                            delta_sides = 2;
                        }
                        // we inherit the region from the left
                        representativ = Some(representatives_grid[y][x - 1]);
                    }
                    // else we are a new region
                }
                Neighbor::SameRegion(top_region_idx) => {
                    if let Neighbor::SameRegion(left_region_idx) = neighbors.left {
                        if let Neighbor::SameRegion(_) = neighbors.top_right {
                        } else {
                            delta_sides -= 2;
                        }
                        if left_region_idx != top_region_idx {
                            // merge regions
                            let (left_region, top_region): (&mut RegionP2, &mut RegionP2) = unsafe {
                                let ptr = regions.as_mut_ptr();
                                (
                                    &mut *ptr.add(left_region_idx),
                                    &mut *ptr.add(top_region_idx),
                                )
                            };
                            top_region.replaced_by = Some(left_region_idx);
                            left_region.area += top_region.area;
                            left_region.sides += top_region.sides;
                            representatives[representatives_grid[y][x - 1]] = left_region_idx;
                        }
                        representativ = Some(representatives_grid[y][x - 1]);
                    } else {
                        representativ = Some(representatives_grid[y - 1][x]);
                        if let Neighbor::SameRegion(_) = neighbors.top_left {
                            delta_sides += 2;
                        }
                        if let Neighbor::SameRegion(_) = neighbors.top_right {
                            delta_sides += 2;
                        }
                    }
                }
                _ => {
                    panic!("undefined neighbor")
                }
            }

            if representativ.is_none() {
                let region = RegionP2 {
                    label: c,
                    replaced_by: None,
                    area: 1,
                    sides: 4,
                };
                regions.push(region);
                representatives.push(regions.len() - 1);
                representativ = Some(representatives.len() - 1);
            } else {
                let region = regions
                    .get_mut(representatives[representativ.unwrap()])
                    .unwrap();
                region.area += 1;
                region.sides += delta_sides;
            }
            representatives_grid[y][x] = representativ.unwrap();
        }
    }

    // sum of areas * perimeters of all regions that are not deprecated
    Some(
        regions
            .iter()
            .filter(|r| r.replaced_by.is_none())
            .map(|r| r.area * r.sides as u64)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
