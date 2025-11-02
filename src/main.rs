mod utility;
use utility::*;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    const N: i16 = 4;
    let mut points: [Point; MAX_N] = [Point { x: 0, y: 0 }; MAX_N];
    points[0] = Point { x: 0, y: 0 };
    points[1] = Point { x: 1, y: 0 };

    let max_distance = N * (N - 1) / 2;
    let offsets = compute_offsets(max_distance);

    let mut used_distances = [false; MAX_DIST];
    mark_distances(&points[..2], points[0], &mut used_distances);
    mark_distances(&points[..2], points[1], &mut used_distances);

    let filename = format!("{}-solutions.txt", N);
    let mut file = File::create(&filename).expect("Failed to create file");

    let start = Instant::now();
    let mut solution_count = 0;

    backtrack(
        &mut points,
        2,
        N as usize,
        &offsets,
        &mut used_distances,
        &mut file,
        &mut solution_count,
    );

    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    println!(
        "Found {} solutions in {:.2} s, {:.2}/s",
        solution_count,
        elapsed_secs,
        solution_count as f64 / elapsed_secs
    );
}

fn backtrack(
    points: &mut [Point; MAX_N],
    point_count: usize,
    target_count: usize,
    offsets: &[Vec<Point>],
    used: &mut [bool; MAX_DIST],
    file: &mut File,
    solution_count: &mut usize,
) {
    if point_count == target_count {
        // Write solution to file
        let line: Vec<String> = points[..point_count]
            .iter()
            .map(|p| format!("({}, {})", p.x, p.y))
            .collect();
        writeln!(file, "{}", line.join(" ")).expect("Failed to write to file");

        *solution_count += 1;
        return;
    }

    let candidates = generate_candidates(&points[..point_count], offsets, used);

    for candidate in candidates {
        if !check_distances_fast(&points[..point_count], candidate, used) {
            continue;
        }

        mark_distances(&points[..point_count], candidate, used);
        points[point_count] = candidate;
        backtrack(
            points,
            point_count + 1,
            target_count,
            offsets,
            used,
            file,
            solution_count,
        );
        unmark_distances(&points[..point_count], candidate, used);
    }
}
