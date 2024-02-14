extern crate ndarray;

use ndarray::linalg::general_mat_vec_mul;
use ndarray::{s, Array as NdArray, Array1, Array2, Axis};
use pgrx::array::RawArray;
use pgrx::*;
use std::f32::consts::E;
use std::{f32, f64};

#[pg_extern(immutable, parallel_safe, strict, name = "logsigmoid")]
fn logsigmoid(x: f32) -> f32 {
    -((1.0 + (-x).exp()).ln())
}

fn exp(x: f32) -> f32 {
    x.exp()
}

fn logsigmoid_vectorized(x: &Array1<f32>) -> Array1<f32> {
    -(&(x.mapv(|x| (-x).exp()) + 1.0).mapv(f32::ln))
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l2_parallel")]
fn transe_l2_parallel(
    head: Array<f32>,
    rel: Array<f32>,
    tails: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
    reverse: bool,
) -> Vec<Option<f32>> {
    let head_len = head.len();
    let rel_len = rel.len();
    let tails_len = tails.len();
    if head_len != rel_len {
        error!("The length of the head, relation arrays must be the same.");
    }

    if tails_len % head_len != 0 {
        error!("The length of the tail array must be a multiple of the head array.");
    }

    let head = head.iter_deny_null().collect::<Vec<f32>>();
    let rel = rel.iter_deny_null().collect::<Vec<f32>>();
    let tails = tails.iter_deny_null().collect::<Vec<f32>>();

    // Ensure head and rel are column vectors for matrix operations
    let head = NdArray::from_vec(head).into_shape((1, head_len)).unwrap();
    let rel = NdArray::from_vec(rel).into_shape((1, rel_len)).unwrap();
    let tails = NdArray::from_vec(tails)
        .into_shape((tails_len / head_len, head_len))
        .unwrap();

    // Broadcasting head and rel over tails, and computing difference
    let head_broad = head.broadcast(tails.dim()).unwrap();
    let rel_broad = rel.broadcast(tails.dim()).unwrap();

    let diff = if reverse {
        &(&tails + &rel_broad) - &head_broad
    } else {
        &(&head_broad + &rel_broad) - &tails
    };
    // info!(
    //     "Head shape: {:?}, Rel shape: {:?}, Tails shape: {:?}, Tails: {:?}, Head: {:?}, Rel: {:?}",
    //     head.dim(),
    //     rel.dim(),
    //     tails.dim(),
    //     tails,
    //     head,
    //     rel
    // );

    // Squaring the differences and summing over columns to get distances
    let squared_diff = &diff * &diff;
    let distances = squared_diff.sum_axis(Axis(1)).mapv(f32::sqrt);

    let adjusted_distances = gamma - distances;

    let result = if exp_enabled {
        logsigmoid_vectorized(&adjusted_distances).mapv(|x| E.powf(x))
    } else {
        logsigmoid_vectorized(&adjusted_distances)
    };

    result.iter().map(|&x| Some(x)).collect()
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l2_ndarray")]
fn transe_l2_ndarray(
    head_array: Array<f32>,
    relation_array: Array<f32>,
    tail_array: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
    reverse: bool,
) -> f32 {
    if head_array.len() != relation_array.len() || head_array.len() != tail_array.len() {
        error!("The length of the head, relation, and tail arrays must be the same.");
    }

    // exp(logsigmoid(gamma - th.norm(score, p=2, dim=-1)))
    let head_array = Array1::from_vec(head_array.iter_deny_null().collect());
    let relation_array = Array1::from_vec(relation_array.iter_deny_null().collect());
    let tail_array = Array1::from_vec(tail_array.iter_deny_null().collect());
    let score = if reverse {
        gamma
            - (&tail_array + &relation_array - &head_array)
                .mapv(|x| x.powi(2))
                .sum()
                .sqrt()
    } else {
        gamma
            - (&head_array + &relation_array - &tail_array)
                .mapv(|x| x.powi(2))
                .sum()
                .sqrt()
    };

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l2")]
fn transe_l2(
    head_array: Array<f32>,
    relation_array: Array<f32>,
    tail_array: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
    reverse: bool,
) -> f32 {
    if head_array.len() != relation_array.len() || head_array.len() != tail_array.len() {
        error!("The length of the head, relation, and tail arrays must be the same.");
    }

    // exp(logsigmoid(gamma - th.norm(score, p=2, dim=-1)))
    let score = if reverse {
        gamma
            - tail_array
                .iter_deny_null()
                .zip(relation_array.iter_deny_null())
                .zip(head_array.iter_deny_null())
                .map(|((h, r), t)| (t + r - h).powi(2))
                .sum::<f32>()
                .sqrt()
    } else {
        gamma
            - head_array
                .iter_deny_null()
                .zip(relation_array.iter_deny_null())
                .zip(tail_array.iter_deny_null())
                .map(|((h, r), t)| (h + r - t).powi(2))
                .sum::<f32>()
                .sqrt()
    };

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l1")]
fn transe_l1(
    head_array: Array<f32>,
    relation_array: Array<f32>,
    tail_array: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
    reverse: bool,
) -> f32 {
    if head_array.len() != relation_array.len() || head_array.len() != tail_array.len() {
        error!("The length of the head, relation, and tail arrays must be the same.");
    }

    // exp(logsigmoid(gamma - th.norm(score, p=1, dim=-1)))
    let score = if reverse {
        gamma
            - tail_array
                .iter_deny_null()
                .zip(relation_array.iter_deny_null())
                .zip(head_array.iter_deny_null())
                .map(|((h, r), t)| (h + r - t).abs())
                .sum::<f32>()
    } else {
        gamma
            - head_array
                .iter_deny_null()
                .zip(relation_array.iter_deny_null())
                .zip(tail_array.iter_deny_null())
                .map(|((h, r), t)| (h + r - t).abs())
                .sum::<f32>()
    };

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l1_ndarray")]
fn transe_l1_ndarray(
    head_array: Array<f32>,
    relation_array: Array<f32>,
    tail_array: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
    reverse: bool,
) -> f32 {
    if head_array.len() != relation_array.len() || head_array.len() != tail_array.len() {
        error!("The length of the head, relation, and tail arrays must be the same.");
    }

    // exp(logsigmoid(gamma - th.norm(score, p=1, dim=-1)))
    let head_array = Array1::from_vec(head_array.iter_deny_null().collect());
    let relation_array = Array1::from_vec(relation_array.iter_deny_null().collect());
    let tail_array = Array1::from_vec(tail_array.iter_deny_null().collect());
    let score = if reverse {
        gamma - (&tail_array + &relation_array - &head_array).mapv(|x| x.abs()).sum()
    } else {
        gamma - (&head_array + &relation_array - &tail_array).mapv(|x| x.abs()).sum()
    };

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l1_parallel")]
fn transe_l1_parallel(
    head: Array<f32>,
    rel: Array<f32>,
    tails: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
    reverse: bool,
) -> Vec<Option<f32>> {
    let head_len = head.len();
    let rel_len = rel.len();
    let tails_len = tails.len();
    if head_len != rel_len {
        error!("The length of the head, relation arrays must be the same.");
    }

    if tails_len % head_len != 0 {
        error!("The length of the tail array must be a multiple of the head array.");
    }

    let head = head.iter_deny_null().collect::<Vec<f32>>();
    let rel = rel.iter_deny_null().collect::<Vec<f32>>();
    let tails = tails.iter_deny_null().collect::<Vec<f32>>();

    // Ensure head and rel are column vectors for matrix operations
    let head = NdArray::from_vec(head).into_shape((1, head_len)).unwrap();
    let rel = NdArray::from_vec(rel).into_shape((1, rel_len)).unwrap();
    let tails = NdArray::from_vec(tails)
        .into_shape((tails_len / head_len, head_len))
        .unwrap();

    // Broadcasting head and rel over tails, and computing difference
    let head_broad = head.broadcast(tails.dim()).unwrap();
    let rel_broad = rel.broadcast(tails.dim()).unwrap();

    let diff = if reverse {
        &(&tails + &rel_broad) - &head_broad
    } else {
        &(&head_broad + &rel_broad) - &tails
    };
    // info!(
    //     "Head shape: {:?}, Rel shape: {:?}, Tails shape: {:?}, Tails: {:?}, Head: {:?}, Rel: {:?}",
    //     head.dim(),
    //     rel.dim(),
    //     tails.dim(),
    //     tails,
    //     head,
    //     rel
    // );

    let distances = diff.mapv(|x| x.abs()).sum_axis(Axis(1));
    let adjusted_distances = gamma - distances;

    let result = if exp_enabled {
        logsigmoid_vectorized(&adjusted_distances).mapv(|x| E.powf(x))
    } else {
        logsigmoid_vectorized(&adjusted_distances)
    };

    result.iter().map(|&x| Some(x)).collect()
}

#[pg_extern(immutable, parallel_safe, strict, name = "distmult")]
fn distmult(
    head_array: Array<f32>,
    relation_array: Array<f32>,
    tail_array: Array<f32>,
    exp_enabled: bool,
    reverse: bool,
) -> f32 {
    if head_array.len() != relation_array.len() || head_array.len() != tail_array.len() {
        error!("The length of the head, relation, and tail arrays must be the same.");
    }

    // th.sum(head * relation * tail, dim=-1)
    let score = if reverse {
        tail_array
            .iter_deny_null()
            .zip(relation_array.iter_deny_null())
            .zip(head_array.iter_deny_null())
            .map(|((h, r), t)| h * r * t)
            .sum::<f32>()
    } else {
        head_array
            .iter_deny_null()
            .zip(relation_array.iter_deny_null())
            .zip(tail_array.iter_deny_null())
            .map(|((h, r), t)| h * r * t)
            .sum::<f32>()
    };

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "distmult_ndarray")]
fn distmult_ndarray(
    head_array: Array<f32>,
    relation_array: Array<f32>,
    tail_array: Array<f32>,
    exp_enabled: bool,
    reverse: bool,
) -> f32 {
    if head_array.len() != relation_array.len() || head_array.len() != tail_array.len() {
        error!("The length of the head, relation, and tail arrays must be the same.");
    }

    // th.sum(head * relation * tail, dim=-1)
    let head_array = Array1::from_vec(head_array.iter_deny_null().collect());
    let relation_array = Array1::from_vec(relation_array.iter_deny_null().collect());
    let tail_array = Array1::from_vec(tail_array.iter_deny_null().collect());

    let score = if reverse {
        (&tail_array * &relation_array * &head_array).sum()
    } else {
        (&head_array * &relation_array * &tail_array).sum()
    };

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[pg_test]
    fn test_logsigmoid() {
        let result = Spi::get_one::<f32>("SELECT pgml.logsigmoid(1.0)");
        assert_eq!(result, Ok(Some(-0.31326166)));
    }

    #[test]
    fn test_exp() {
        assert_eq!(exp(0.0), 1.0);
        assert_eq!(exp(1.0), 2.7182817);
        assert_eq!(exp(2.0), 7.389056);
    }

    #[pg_test]
    fn test_transe_l2_parallel() {
        let result = Spi::get_one::<Vec<f32>>(
            "SELECT pgml.transe_l2_parallel(ARRAY[1.0, 2.0, 3.0], ARRAY[4.0, 5.0, 6.0], ARRAY[7.0, 8.0, 9.0, 10.0, 11.0, 12.0], 10.0, true, false)",
        );
        assert_eq!(result, Ok(Some(vec![0.99957544, 0.9492583])));
    }

    #[pg_test]
    fn test_transe_l2() {
        let result = Spi::get_one::<f32>(
            "SELECT pgml.transe_l2(ARRAY[1.0, 2.0, 3.0], ARRAY[4.0, 5.0, 6.0], ARRAY[7.0, 8.0, 9.0], 10.0, true, false)",
        );
        assert_eq!(result, Ok(Some(0.99957544)));

        let result = Spi::get_one::<f32>(
            "SELECT pgml.transe_l2(ARRAY[1.0, 2.0, 3.0], ARRAY[4.0, 5.0, 6.0], ARRAY[10.0, 11.0, 12.0], 10.0, true, false)",
        );
        assert_eq!(result, Ok(Some(0.9492583)));
    }

    #[pg_test]
    fn test_transe_l1() {
        let result = Spi::get_one::<f32>(
            "SELECT pgml.transe_l1(ARRAY[1.0, 2.0, 3.0], ARRAY[4.0, 5.0, 6.0], ARRAY[7.0, 8.0, 9.0], 10.0, true, false)",
        );
        assert_eq!(result, Ok(Some(0.999089)));
    }

    #[pg_test]
    fn test_distmult() {
        let result = Spi::get_one::<f32>(
            "SELECT pgml.distmult(ARRAY[0.1, 0.2, 3.0], ARRAY[0.4, 0.5, 0.6], ARRAY[0.7, 0.8, 0.9], true, false)",
        );
        assert_eq!(result, Ok(Some(0.8491564)));
    }
}
