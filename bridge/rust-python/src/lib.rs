use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::sync::Arc;
use tokio::sync::RwLock;

// Import Axionax core modules
use consensus::{ConsensusEngine, Validator, ConsensusConfig, Challenge};
use blockchain::{Blockchain, Block, Transaction, BlockchainConfig};
use crypto::VRF;

mod simple_wrapper;
use simple_wrapper::{default_consensus_config, default_blockchain_config};

/// Python wrapper for VRF operations
#[pyclass]
struct PyVRF {
    vrf: VRF,
}

#[pymethods]
impl PyVRF {
    #[new]
    fn new() -> Self {
        PyVRF { vrf: VRF::new() }
    }

    /// Generate VRF proof and hash
    fn prove(&self, input: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
        let (proof, hash) = self.vrf.prove(&input);
        Ok((proof, hash.to_vec()))
    }
}

/// Python wrapper for Validator
#[pyclass]
#[derive(Clone)]
struct PyValidator {
    pub address: String,
    pub stake: u128,
    pub is_active: bool,
}

#[pymethods]
impl PyValidator {
    #[new]
    fn new(address: String, stake: u64) -> Self {
        PyValidator {
            address,
            stake: stake as u128,
            is_active: true,
        }
    }

    #[getter]
    fn address(&self) -> PyResult<String> {
        Ok(self.address.clone())
    }

    #[getter]
    fn stake(&self) -> PyResult<u64> {
        Ok(self.stake as u64)
    }

    #[getter]
    fn is_active(&self) -> PyResult<bool> {
        Ok(self.is_active)
    }
}

/// Python wrapper for Consensus Engine
#[pyclass]
struct PyConsensusEngine {
    runtime: tokio::runtime::Runtime,
    engine: Arc<RwLock<ConsensusEngine>>,
}

#[pymethods]
impl PyConsensusEngine {
    #[new]
    fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyValueError::new_err(format!("Failed to create runtime: {}", e)))?;
        
        let config = default_consensus_config();
        let engine = Arc::new(RwLock::new(ConsensusEngine::new(config)));
        
        Ok(PyConsensusEngine { runtime, engine })
    }

    /// Register a validator
    fn register_validator(&mut self, validator: PyValidator) -> PyResult<()> {
        let engine = self.engine.clone();
        let rust_validator = Validator {
            address: validator.address.clone(),
            stake: validator.stake,
            total_votes: 0,
            correct_votes: 0,
            false_pass: 0,
            is_active: validator.is_active,
        };
        
        self.runtime.block_on(async move {
            let eng = engine.read().await;
            eng.register_validator(rust_validator).await
        }).map_err(|e| PyValueError::new_err(e))
    }

    /// Generate challenge
    fn generate_challenge(&self, job_id: String, output_size: usize) -> PyResult<PyChallenge> {
        let engine = self.engine.clone();
        let challenge = self.runtime.block_on(async move {
            let eng = engine.read().await;
            let vrf_seed = [1u8; 32]; // TODO: generate properly
            eng.generate_challenge(job_id, output_size, vrf_seed)
        });
        
        Ok(PyChallenge { inner: challenge })
    }

    /// Calculate fraud detection probability (static method)
    #[staticmethod]
    fn fraud_probability(fraud_rate: f64, sample_size: usize) -> PyResult<f64> {
        Ok(ConsensusEngine::fraud_detection_probability(fraud_rate, sample_size))
    }
}

/// Python wrapper for Challenge
#[pyclass]
struct PyChallenge {
    inner: Challenge,
}

#[pymethods]
impl PyChallenge {
    #[getter]
    fn job_id(&self) -> PyResult<String> {
        Ok(self.inner.job_id.clone())
    }

    #[getter]
    fn sample_size(&self) -> PyResult<usize> {
        Ok(self.inner.sample_size)
    }

    #[getter]
    fn samples(&self) -> PyResult<Vec<usize>> {
        Ok(self.inner.samples.clone())
    }
}

/// Python wrapper for Transaction
#[pyclass]
#[derive(Clone)]
struct PyTransaction {
    pub from: String,
    pub to: String,
    pub value: u128,
    pub data: Vec<u8>,
}

#[pymethods]
impl PyTransaction {
    #[new]
    fn new(from: String, to: String, value: u64, data: Vec<u8>) -> Self {
        PyTransaction {
            from,
            to,
            value: value as u128,
            data,
        }
    }

    #[getter]
    fn from_address(&self) -> PyResult<String> {
        Ok(self.from.clone())
    }

    #[getter]
    fn to_address(&self) -> PyResult<String> {
        Ok(self.to.clone())
    }

    #[getter]
    fn value(&self) -> PyResult<u64> {
        Ok(self.value as u64)
    }
}

/// Python wrapper for Block
#[pyclass]
struct PyBlock {
    inner: Block,
}

#[pymethods]
impl PyBlock {
    #[getter]
    fn number(&self) -> PyResult<u64> {
        Ok(self.inner.number)
    }

    #[getter]
    fn timestamp(&self) -> PyResult<u64> {
        Ok(self.inner.timestamp)
    }

    #[getter]
    fn proposer(&self) -> PyResult<String> {
        Ok(self.inner.proposer.clone())
    }

    #[getter]
    fn transactions_count(&self) -> PyResult<usize> {
        Ok(self.inner.transactions.len())
    }

    #[getter]
    fn gas_used(&self) -> PyResult<u64> {
        Ok(self.inner.gas_used)
    }
}

/// Python wrapper for Blockchain
#[pyclass]
struct PyBlockchain {
    runtime: tokio::runtime::Runtime,
    chain: Arc<RwLock<Blockchain>>,
}

#[pymethods]
impl PyBlockchain {
    #[new]
    fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyValueError::new_err(format!("Failed to create runtime: {}", e)))?;
        
        let config = default_blockchain_config();
        let chain = Arc::new(RwLock::new(Blockchain::new(config)));
        
        // Initialize with genesis block
        let chain_clone = chain.clone();
        runtime.block_on(async move {
            let bc = chain_clone.read().await;
            bc.init_with_genesis().await;
        });
        
        Ok(PyBlockchain { runtime, chain })
    }

    /// Get the latest block number
    fn latest_block_number(&self) -> PyResult<u64> {
        let chain = self.chain.clone();
        let num = self.runtime.block_on(async move {
            let bc = chain.read().await;
            bc.get_latest_block_number().await
        });
        
        Ok(num)
    }

    /// Get block by number
    fn get_block(&self, number: u64) -> PyResult<Option<PyBlock>> {
        let chain = self.chain.clone();
        let block = self.runtime.block_on(async move {
            let bc = chain.read().await;
            bc.get_block(number).await
        });
        
        Ok(block.map(|b| PyBlock { inner: b }))
    }
}

/// Python module definition
#[pymodule]
fn axionax_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyVRF>()?;
    m.add_class::<PyValidator>()?;
    m.add_class::<PyConsensusEngine>()?;
    m.add_class::<PyChallenge>()?;
    m.add_class::<PyTransaction>()?;
    m.add_class::<PyBlock>()?;
    m.add_class::<PyBlockchain>()?;
    Ok(())
}
