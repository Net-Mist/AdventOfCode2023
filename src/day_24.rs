use ndarray::prelude::*;
use ndarray_linalg::Solve;
use std::str::from_utf8;

type Input = Vec<Vec<f64>>;

pub fn generator(input: &[u8]) -> Input {
    let input = from_utf8(input).unwrap();
    input
        .lines()
        .map(|line| {
            line.split(" @ ")
                .flat_map(|part| part.split(", "))
                .map(|word| {
                    let word = word.trim();
                    word.strip_suffix(',')
                        .unwrap_or(word)
                        .parse::<f64>()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut n_inter = 0;
    for (i, line1) in input.iter().enumerate() {
        for line2 in input[i + 1..].iter() {
            let den = line1[3] - line2[3] * line1[4] / line2[4];
            if den == 0. {
                continue;
            }
            let t = (line2[0] - line1[0] + line2[3] * (line1[1] - line2[1]) / line2[4]) / den;
            if t < 0. {
                continue;
            }
            let t_prime = (line1[0] + line1[3] * t - line2[0]) / line2[3];
            if t_prime < 0. {
                continue;
            }
            let ix = line1[0] + line1[3] * t;
            let iy = line1[1] + line1[4] * t;
            if (200000000000000. ..=400000000000000.).contains(&ix)
                && (200000000000000. ..=400000000000000.).contains(&iy)
            {
                n_inter += 1;
            }
        }
    }
    n_inter
}

pub fn part2(input: &Input) -> usize {
    // equations are
    // x_0 + K_0 v_x0 = x_R + K_0 v_xR (1)
    // y_0 + K_0 v_y0 = y_R + K_0 v_yR (2)
    // z_0 + K_0 v_z0 = z_R + K_0 v_zR (3)
    // x_1 + K_1 v_x1 = x_R + K_1 v_xR (4)
    // y_1 + K_1 v_y1 = y_R + K_1 v_yR (5)
    // z_1 + K_1 v_z1 = z_R + K_1 v_zR (6)
    // x_2 + K_2 v_x2 = x_R + K_2 v_xR (7)
    // y_2 + K_2 v_y2 = y_R + K_2 v_yR (8)
    // z_2 + K_2 v_z2 = z_R + K_2 v_zR (9)
    //
    // isolate K_0 in (1) and (2) then develop
    // x_R v_y0 - x_R v_yR - x_0 v_y0 + x_0 v_yR - y_R v_x0 + y_0 v_x0 + y_R v_xR - y0 v_xR = 0
    //
    // same with (4) and (5)
    // x_R v_y1 - x_R v_yR - x_1 v_y1 + x_1 v_yR - y_R v_x1 + y_1 v_x1 + y_R v_xR - y1 v_xR = 0
    //
    // Then substract to remove terms of order 2
    // x_R (v_y0 - v_y1) + y_R (v_x1 - v_x0) + v_xR (y_1 - y_0) + v_yR (x_0 - x_1) = x_0 v_y0 - x_1 v_y1  - y_0 v_x0 +  y_1 v_x1
    //
    // same can be done with (1), (2), (7) and (8), .... to create a set of 6 linear equations with 6 unknowns

    let a: Array2<f64> = array![
        [
            input[0][4] - input[1][4],
            input[1][3] - input[0][3],
            0.,
            input[1][1] - input[0][1],
            input[0][0] - input[1][0],
            0.
        ],
        [
            input[0][4] - input[2][4],
            input[2][3] - input[0][3],
            0.,
            input[2][1] - input[0][1],
            input[0][0] - input[2][0],
            0.
        ],
        [
            input[0][5] - input[1][5],
            0.,
            input[1][3] - input[0][3],
            input[1][2] - input[0][2],
            0.,
            input[0][0] - input[1][0],
        ],
        [
            input[0][5] - input[2][5],
            0.,
            input[2][3] - input[0][3],
            input[2][2] - input[0][2],
            0.,
            input[0][0] - input[2][0],
        ],
        [
            0.,
            input[0][5] - input[1][5],
            input[1][4] - input[0][4],
            0.,
            input[1][2] - input[0][2],
            input[0][1] - input[1][1],
        ],
        [
            0.,
            input[0][5] - input[2][5],
            input[2][4] - input[0][4],
            0.,
            input[2][2] - input[0][2],
            input[0][1] - input[2][1],
        ],
    ];

    let b: Array1<f64> = array![
        input[0][0] * input[0][4] - input[1][0] * input[1][4] - input[0][1] * input[0][3]
            + input[1][1] * input[1][3],
        input[0][0] * input[0][4] - input[2][0] * input[2][4] - input[0][1] * input[0][3]
            + input[2][1] * input[2][3],
        input[0][0] * input[0][5] - input[1][0] * input[1][5] - input[0][2] * input[0][3]
            + input[1][2] * input[1][3],
        input[0][0] * input[0][5] - input[2][0] * input[2][5] - input[0][2] * input[0][3]
            + input[2][2] * input[2][3],
        input[0][1] * input[0][5] - input[1][1] * input[1][5] - input[0][2] * input[0][4]
            + input[1][2] * input[1][4],
        input[0][1] * input[0][5] - input[2][1] * input[2][5] - input[0][2] * input[0][4]
            + input[2][2] * input[2][4],
    ];
    let x = a.solve_into(b).unwrap();
    (x[0] + x[1] + x[2]) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(24, 21679, 566914635762564);
}
