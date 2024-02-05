extern crate ndarray;
use ndarray::{arr1, arr2, ArrayBase, Data, Ix1, Ix2};
use pgrx::array::RawArray;
use pgrx::*;
use std::{f32, f64};

#[pg_extern(immutable, parallel_safe, strict, name = "logsigmoid")]
fn logsigmoid(x: f32) -> f32 {
    -((1.0 + (-x).exp()).ln())
}

fn exp(x: f32) -> f32 {
    x.exp()
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l2")]
fn transe_l2(
    head_vector: Array<f32>,
    relation_vector: Array<f32>,
    tail_vector: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
) -> f32 {
    // exp(logsigmoid(gamma - th.norm(score, p=2, dim=-1)))
    let score = gamma
        - head_vector
            .iter_deny_null()
            .zip(relation_vector.iter_deny_null())
            .zip(tail_vector.iter_deny_null())
            .map(|((h, r), t)| (h + r - t).powi(2))
            .sum::<f32>()
            .sqrt();

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "transe_l1")]
fn transe_l1(
    head_vector: Array<f32>,
    relation_vector: Array<f32>,
    tail_vector: Array<f32>,
    gamma: f32,
    exp_enabled: bool,
) -> f32 {
    // exp(logsigmoid(gamma - th.norm(score, p=1, dim=-1)))
    let score = gamma
        - head_vector
            .iter_deny_null()
            .zip(relation_vector.iter_deny_null())
            .zip(tail_vector.iter_deny_null())
            .map(|((h, r), t)| (h + r - t).abs())
            .sum::<f32>();

    if exp_enabled {
        exp(logsigmoid(score))
    } else {
        logsigmoid(score)
    }
}

#[pg_extern(immutable, parallel_safe, strict, name = "distmult")]
fn distmult(head_vector: Array<f32>, relation_vector: Array<f32>, tail_vector: Array<f32>, exp_enabled: bool) -> f32 {
    // th.sum(head * relation * tail, dim=-1)
    let score = head_vector
        .iter_deny_null()
        .zip(relation_vector.iter_deny_null())
        .zip(tail_vector.iter_deny_null())
        .map(|((h, r), t)| h * r * t)
        .sum::<f32>();

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
    fn test_transe_l2() {
        let result = Spi::get_one::<f32>(
            "SELECT pgml.transe_l2(ARRAY[1.0, 2.0, 3.0], ARRAY[4.0, 5.0, 6.0], ARRAY[7.0, 8.0, 9.0], 10.0, true)",
        );
        assert_eq!(result, Ok(Some(0.99957544)));
    }

    #[pg_test]
    fn test_transe_l1() {
        let result = Spi::get_one::<f32>(
            "SELECT pgml.transe_l1(ARRAY[1.0, 2.0, 3.0], ARRAY[4.0, 5.0, 6.0], ARRAY[7.0, 8.0, 9.0], 10.0, true)",
        );
        assert_eq!(result, Ok(Some(0.999089)));
    }

    #[pg_test]
    fn test_distmult() {
        let result = Spi::get_one::<f32>(
            "SELECT pgml.distmult(ARRAY[0.1, 0.2, 3.0], ARRAY[0.4, 0.5, 0.6], ARRAY[0.7, 0.8, 0.9], true)",
        );
        assert_eq!(result, Ok(Some(0.8491564)));
    }
}
