use rand::Rng;
use rand_chacha::ChaCha8Rng;

fn get_library_size() -> i32 {
    1
}

fn get_library_entry(item: i32) -> Vec<Vec<i32>> {
    match item {
        0 => vec![
            vec![
                1, 1, 1, 1, -1, 2, 2, 2, 2, 2, 2, 2, -1, 3, 3, 3, 3, 3, 3, -1, 4, 4, 4, 4,
            ],
            vec![
                1, 1, 1, 1, -1, 2, 2, 2, 2, 2, 2, 2, -1, 3, 3, 3, 3, 3, 3, -1, 4, 4, 4, 4,
            ],
            vec![
                1, 1, 1, 1, -1, 2, 2, 2, 2, 2, 2, 2, -1, 3, 3, 3, 3, 3, 3, -1, 4, 4, 4, 4,
            ],
            vec![
                1, 1, 1, 1, -1, -1, -1, 2, 2, 2, 2, 2, -1, 3, 3, 3, 3, -1, -1, -1, 4, 4, 4, 4,
            ],
            vec![
                1, 1, 1, 1, 1, 1, -1, 2, 2, 2, 2, -1, -1, 3, 3, 3, 3, -1, 4, 4, 4, 4, 4, 4,
            ],
            vec![
                1, 1, 1, 1, 1, 1, -1, 2, 2, 2, 2, -1, 3, 3, 3, 3, 3, -1, 4, 4, 4, 4, 4, 4,
            ],
            vec![
                1, 1, 1, 1, 1, 1, -1, -1, -1, -1, 2, -1, 3, 3, -1, -1, -1, -1, 4, 4, 4, 4, 4, 4,
            ],
            vec![
                -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, 2, -1, 3, 3, -1, 4, 4, 4, 4, 4, -1, -1, -1, -1,
            ],
            vec![
                5, 5, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6,
                6, 6,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 6, 6, 6, 6, 6, 6, 6,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 6, 6, 6, 6, 6, 6, 6,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 6, 6, 6, 6, 6, 6, 6,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 6, 6, 6, 6, 6, 6, 6,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 6, 6, 6, 6, 6, 6, 6,
            ],
            vec![
                5, 5, 5, 5, 5, 5, 5, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 6, 6, 6, 6, 6, 6, 6,
            ],
            vec![
                5, 5, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6,
                6, 6,
            ],
            vec![
                -1, -1, -1, -1, 7, 7, 7, 7, 7, -1, 8, -1, 9, 9, -1, 10, 10, 10, 10, 10, -1, -1, -1,
                -1,
            ],
            vec![
                7, 7, 7, 7, 7, 7, -1, -1, -1, -1, 8, -1, 9, 9, -1, -1, -1, -1, 10, 10, 10, 10, 10,
                10,
            ],
            vec![
                7, 7, 7, 7, 7, 7, -1, 8, 8, 8, 8, -1, 9, 9, 9, 9, 9, -1, 10, 10, 10, 10, 10, 10,
            ],
            vec![
                7, 7, 7, 7, 7, 7, -1, 8, 8, 8, 8, -1, 9, 9, 9, 9, 9, -1, 10, 10, 10, 10, 10, 10,
            ],
            vec![
                7, 7, 7, 7, -1, -1, -1, 8, 8, 8, 8, -1, -1, 9, 9, 9, 9, -1, -1, -1, 10, 10, 10, 10,
            ],
            vec![
                7, 7, 7, 7, -1, 8, 8, 8, 8, 8, 8, 8, -1, 9, 9, 9, 9, 9, 9, -1, 10, 10, 10, 10,
            ],
            vec![
                7, 7, 7, 7, -1, 8, 8, 8, 8, 8, 8, 8, -1, 9, 9, 9, 9, 9, 9, -1, 10, 10, 10, 10,
            ],
            vec![
                7, 7, 7, 7, -1, 8, 8, 8, 8, 8, 8, 8, -1, 9, 9, 9, 9, 9, 9, -1, 10, 10, 10, 10,
            ],
        ],
        // Add other cases if needed
        _ => panic!("Requested Invalid Map Template"),
    }
}

pub fn get_sub_division(rng: &mut ChaCha8Rng) -> Vec<Vec<i32>> {
    // select from a template

    let mut template = get_library_entry(rng.gen_range(0..get_library_size()));

    let should_flip = rng.gen_range(0..=1);
    if should_flip == 1 {
        flip_horizontally_in_place(&mut template);
    }

    let rotate_times = rng.gen_range(0..=3);

    for _n in 0..=rotate_times {
        rotate_90_degrees(&mut template);
    }

    template
}

fn flip_horizontally_in_place(matrix: &mut Vec<Vec<i32>>) {
    for row in matrix {
        row.reverse();
    }
}

fn rotate_90_degrees(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let height = matrix.len();
    let width = matrix[0].len();

    // Create a new matrix with swapped dimensions
    let mut rotated_matrix = vec![vec![0; height]; width];

    for i in 0..height {
        for j in 0..width {
            rotated_matrix[j][height - 1 - i] = matrix[i][j];
        }
    }

    rotated_matrix
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_flip() {
        let mut original_matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        super::flip_horizontally_in_place(&mut original_matrix);

        let result = vec![vec![4, 3, 2, 1], vec![8, 7, 6, 5], vec![12, 11, 10, 9]];

        assert_eq!(original_matrix, result);
    }

    #[test]
    fn test_rotate_90() {
        let original_matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        let rotated = super::rotate_90_degrees(&original_matrix);

        let result = vec![
            vec![9, 5, 1],
            vec![10, 6, 2],
            vec![11, 7, 3],
            vec![12, 8, 4],
        ];

        assert_eq!(result, rotated);
    }
}
