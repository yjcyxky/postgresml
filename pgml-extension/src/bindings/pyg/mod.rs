use pgrx::*;
/// Knowledge Graph Embedding (KGE) models.
///
/// We use the PyTorch Geometric (PyG) library to implement KGE models.
///
/// It use a Tuple[List[str], List[str], List[str]] format for triples.
///
/// Our implementation below calls into Python wrappers
/// defined in `src/bindings/pyg/kge.py`.
use std::collections::HashMap;

use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::{bindings::Bindings, create_pymodule, orm::*};

create_pymodule!("/src/bindings/pyg/kge.py");

struct HrtDataset {
    heads: Vec<String>,
    relations: Vec<String>,
    tails: Vec<String>,
}

impl HrtDataset {
    fn new(heads: Vec<String>, relations: Vec<String>, tails: Vec<String>) -> Self {
        Self {
            heads,
            relations,
            tails,
        }
    }

    fn to_hrt(&self) -> [Vec<String>; 3] {
        [
            self.heads.clone(),
            self.relations.clone(),
            self.tails.clone(),
        ]
    }
}

fn train_kge(
    dataset: &HrtDataset,
    hyperparams: &Hyperparams,
    model_name: &'static str,
) -> Result<Box<dyn Bindings>> {
    let hyperparams = serde_json::to_string(hyperparams).unwrap();

    let (trainer, predict) = Python::with_gil(|py| -> Result<(Py<PyAny>, Py<PyAny>)> {
        let module = get_module!(PY_MODULE);

        let trainer: Py<PyAny> = module.getattr(py, "create_trainer")?;

        let train: Py<PyAny> = trainer.call1(
            py,
            PyTuple::new(
                py,
                &[
                    // a KGE model name. (TransE, ComplEx, DistMult, RotatE etc.)
                    String::from(model_name).into_py(py),
                    hyperparams.into_py(py),
                ],
            ),
        )?;

        let trainer: Py<PyAny> = train.call1(
            py,
            PyTuple::new(
                py,
                [PyTuple::new(
                    py,
                    &[
                        // a list of heads
                        dataset.heads.clone().into_py(py),
                        // a list of relations
                        dataset.relations.clone().into_py(py),
                        // a list of tails
                        dataset.tails.clone().into_py(py),
                    ],
                )],
            ),
        )?;

        let predict: Py<PyAny> = module
            .getattr(py, "create_predictor")?
            .call1(py, PyTuple::new(py, [&trainer]))?
            .extract(py)?;

        Ok((trainer, predict))
    })?;

    Ok(Box::new(Trainer { trainer, predict }))
}

pub struct Trainer {
    trainer: Py<PyAny>,
    predict: Py<PyAny>,
}

unsafe impl Send for Trainer {}
unsafe impl Sync for Trainer {}

impl std::fmt::Debug for Trainer {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.debug_struct("Trainer").finish()
    }
}

impl Bindings for Trainer {
    /// Predict a novel h,t,r triple
    fn predict(&self, h: usize, t: usize, r: usize) -> Result<f32> {
        Python::with_gil(|py| {
            Ok(self
                .predict
                .call1(py, PyTuple::new(py, [h, t, r]))?
                .extract(py)?)
        })
    }

    /// Serialize self to bytes
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Python::with_gil(|py| {
            let save = get_module!(PY_MODULE).getattr(py, "save")?;
            Ok(save
                .call1(py, PyTuple::new(py, [&self.trainer]))?
                .extract(py)?)
        })
    }

    /// Deserialize self from bytes, with additional context
    fn from_bytes(bytes: &[u8]) -> Result<Box<dyn Bindings>>
    where
        Self: Sized,
    {
        Python::with_gil(|py| -> Result<Box<dyn Bindings>> {
            let module = get_module!(PY_MODULE);

            let load = module.getattr(py, "load")?;
            let trainer: Py<PyAny> = load.call1(py, PyTuple::new(py, [bytes]))?.extract(py)?;

            let predict: Py<PyAny> = module
                .getattr(py, "create_predictor")?
                .call1(py, PyTuple::new(py, [&trainer]))?
                .extract(py)?;

            Ok(Box::new(Trainer {
                trainer,
                predict,
            }))
        })
    }
}
