# Axionax DeAI - AI/ML Layer for Security and Auto Selection

**Python-based AI/ML components for the Axionax protocol**

## Modules

### 🤖 ASR (Auto Selection Router)
- Worker selection algorithm
- Suitability scoring
- Performance-based routing
- VRF-weighted selection

### 🔒 Security Analytics
- Fraud detection models
- Anomaly detection
- Behavioral analysis
- Risk scoring

### 🧠 AI Models
- Worker performance prediction
- Resource optimization
- Load balancing ML
- Pattern recognition

### 🛡️ Fraud Detection
- PoPC verification assistance
- Statistical analysis
- Real-time monitoring
- Alert system

## Installation

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

## Requirements

```
torch>=2.0.0
numpy>=1.24.0
pandas>=2.0.0
scikit-learn>=1.3.0
scipy>=1.10.0
```

## Usage

```python
from deai.asr import AutoSelectionRouter
from deai.security import FraudDetector

# Initialize ASR
asr = AutoSelectionRouter(config)
selected_worker = asr.select_worker(job_specs)

# Initialize Fraud Detector
detector = FraudDetector()
is_fraudulent = detector.analyze(proof_data)
```

## Integration with Rust Core

This Python layer integrates with the Rust core through PyO3 bindings located in `/bridge/rust-python`.

## License

MIT
