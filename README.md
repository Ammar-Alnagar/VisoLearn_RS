# VisoLearn

![VisoLearn Logo](static/logo.png)  
**VisoLearn** is an AI-powered educational tool designed to help autistic children improve communication and observational skills through interactive visual learning. The system generates custom images based on a child's learning needs and evaluates their descriptions to provide real-time feedback and guidance.

---

## 🚀 Key Features
- **AI-Powered Image Generation** – Uses **Stable Diffusion** to create tailored images based on user-defined criteria.
- **Vision-Language Evaluation** – Implements **Qwen-VL Max** to analyze user descriptions and measure comprehension.
- **Personalized Learning Adaptation** – Dynamically adjusts difficulty based on individual progress.
- **Interactive Feedback Loop** – Provides hints, suggestions, and multiple attempts for description refinement.
- **Session Tracking & Data Analysis** – Logs learning progress, allowing for comprehensive assessment over time.

---

## 📦 Installation Guide
### Prerequisites
- **Rust** (latest stable release)
- **Cargo** package manager
- **Python** (for optional ML integrations)
- **NVIDIA GPU** (recommended for optimal performance with Stable Diffusion)

### Setup Instructions
```sh
# Clone the repository
git clone https://github.com/Ammar-Alnagar/VisoLearn.git
cd VisoLearn

# Build the project
cargo build --release

# Run the application
cargo run --release
```

---

## 📂 Project Structure
```plaintext
VisoLearn/
├── Cargo.toml             # Main package manifest (dependencies, metadata, and configurations)
├── Cargo.lock             # Lock file to ensure reproducible builds
├── src/                   # Main source directory
│   ├── main.rs            # Application entry point, initializes core services
│   ├── config.rs          # Configuration settings (environment variables, paths, model parameters)
│   ├── models/            # AI model integration
│   │   ├── mod.rs         # Module declarations for models
│   │   ├── evaluation.rs  # Image description evaluation logic using Qwen-VL Max
│   │   ├── image_generation.rs # Image generation pipeline with Stable Diffusion
│   │   ├── prompt_generation.rs # Adaptive prompt creation for educational exercises
│   ├── ui/                # User interface components
│   │   ├── mod.rs         # UI module entry point
│   │   └── interface.rs   # Interface logic for handling user interactions
│   └── utils/             # Utility functions for system operations
│       ├── mod.rs         # Module declarations
│       ├── file_operations.rs # File I/O operations for saving/loading images and logs
│       ├── state_management.rs # Handles session persistence and user progress tracking
│       └── visualization.rs # UI helpers for rendering educational content
├── static/                # Static assets such as icons, templates, and pre-trained models
├── tests/                 # Integration tests for AI models and system components
├── benches/               # Performance benchmarking for AI components
├── examples/              # Example datasets and usage scenarios
├── .env                   # Environment variables for API keys, model paths, etc.
├── .gitignore             # Git ignore rules
├── LICENSE                # MIT License file
└── README.md              # Project documentation
```

---

## 🛠️ Technical Architecture
VisoLearn is built on a modular architecture, ensuring scalability and efficiency. The core system components include:

### 1️⃣ **Image Generation**
- Utilizes **Stable Diffusion Large Turbo** for real-time image synthesis.
- Adjusts visual complexity based on the child's cognitive level.
- Ensures images are clear, calm, and conducive to learning.

### 2️⃣ **Prompt Generation**
- Creates structured educational prompts tailored to different learning levels.
- Uses **LLaMA 3.3 70B** for dynamic language-based adjustments.

### 3️⃣ **Evaluation System**
- Implements **Qwen-VL Max** for detailed response analysis.
- Compares child's descriptions to an AI-generated "ground truth."
- Extracts key elements and provides targeted feedback.

### 4️⃣ **Feedback Mechanism**
- Provides step-by-step hints if key elements are missing.
- Offers multiple attempts before generating a new image.
- Adjusts future tasks based on previous performance.

### 5️⃣ **State Management & Tracking**
- Logs and tracks learning sessions.
- Enables educators to review a child's improvement over time.
- Uses **Rust’s stateful concurrency** for efficient tracking.

### 6️⃣ **User Interface (UI)**
- Designed for simplicity and accessibility.
- Allows caregivers to configure learning parameters easily.
- Supports **text-to-speech** for auditory guidance.

---

## 📊 Benchmarks & Performance
VisoLearn is optimized for **low-latency interaction**. Below are benchmark results for core AI operations:

| Component | Avg Time (ms) | Performance Optimization |
|-----------|--------------|--------------------------|
| **Image Generation** | 1200ms | Using optimized **Stable Diffusion** inference |
| **Evaluation** | 350ms | Qwen-VL Max runs on CUDA-optimized PyTorch |
| **Feedback Processing** | 200ms | Efficient prompt structuring for real-time suggestions |

---

## 🤝 Contributing to VisoLearn
We welcome contributions from the community! Here’s how you can contribute:

1. **Fork the Repository** – Clone the project and create a new branch.
   ```sh
   git checkout -b feature-name
   ```
2. **Make Your Changes** – Implement new features or fix bugs.
3. **Commit Your Work** – Write clear and concise commit messages.
   ```sh
   git commit -m "Added feature X"
   ```
4. **Push & Open a PR** – Submit your changes for review.
   ```sh
   git push origin feature-name
   ```

---

## 📜 License
VisoLearn is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 💡 Acknowledgments
Special thanks to the **open-source community** and contributors for making AI-driven education more accessible!

**Developed by [Ammar Alnagar](https://www.linkedin.com/in/ammar-alnagar-393413201/) 🚀**

