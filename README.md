# Law-gorithm 🏛️⚖️
## *Privacy-First Legal AI with Complete Document Anonymization*

[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen?style=for-the-badge)](docs/deployment/tauri-deployment.md)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-1.8.3-24C8DB?style=for-the-badge&logo=tauri)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust)](src-tauri/)
[![Python](https://img.shields.io/badge/Python-RAG-blue?style=for-the-badge&logo=python)](python_modules/)

> *"True legal transparency requires absolute privacy protection."*

**Law-gorithm** is a privacy-first legal AI desktop application that transforms complex legal documents into clear insights while ensuring **zero PII exposure**. Built with Tauri (Rust backend + Web frontend), it processes legal documents entirely locally with advanced OCR and RAG capabilities.

## 🚀 **Current Status: Development Ready**

✅ **Backend**: Rust services compiled and working  
✅ **OCR**: Bundled Tesseract integration functional  
✅ **Database**: SQLite with vector search ready  
✅ **RAG**: Python file_mgmt module implemented  
✅ **Frontend**: Web interface served and accessible  
⚠️ **Desktop App**: LibSoup conflict resolved, builds successfully

## 👥 **Team Structure & Responsibilities**

This project is organized for collaborative development:

### **Backend Team (RAG & Document file_mgmt)**
- **RAG System**: `python_modules/rag_processor.py` - Vector embeddings and similarity search
- **Document file_mgmt**: `src-tauri/src/processors/` - PDF, OCR, and chunking services  
- **OCR Integration**: `src-tauri/src/services/ocr.rs` - Tesseract OCR with bundled binaries
- **Database**: `src-tauri/src/storage/` - SQLite with vector search capabilities
- **Anonymization**: `src-tauri/src/services/anonymization.rs` - PII protection services

### **Frontend Team**
- **Web Interface**: `src/` - HTML/CSS/JavaScript user interface
- **Components**: `src/components/` - Chat, file upload, and file management UI
- **Tauri Integration**: JavaScript → Rust command bindings

### **DevOps & Deployment**
- **Build System**: `src-tauri/` - Rust/Cargo configuration and build scripts
- **Dependencies**: `vendor/tesseract-linux/` - Bundled OCR binaries and libraries
- **Configurations**: `src-tauri/src/config/` - Application settings and TOML configs

## 🔧 **Quick Start for Team Members**

### **Prerequisites**
```bash
# System dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install -y curl build-essential libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev \
  libgtk-3-dev libappindicator3-dev librsvg2-dev libssl-dev \
  pkg-config python3 python3-pip

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc

# Install Node.js and Tauri CLI
npm install -g @tauri-apps/cli@latest
```

### **Development Setup**
```bash
# Clone and setup
git clone <repository-url>
cd law-gorithm

# Install Python dependencies for RAG
cd python_modules
pip install -r requirements.txt
cd ..

# Install Tauri dependencies
npm install

# Start frontend server (Terminal 1)
npm run serve  # Starts on http://localhost:8080

# Start Rust backend (Terminal 2)  
cd src-tauri
cargo run  # For development builds
```

### **Build & Test**
```bash
# Test backend compilation
cd src-tauri
cargo build  # Development build
cargo build --release  # Production build

# Test full application
npm run tauri dev  # Development mode with hot reload
npm run tauri build  # Production build
```

## 📁 **Project Structure for Teams**

```
law-gorithm/
├── 🔧 **Backend (RAG & file_mgmt)**
│   ├── python_modules/           # Python RAG system
│   │   ├── rag_processor.py      # Main RAG implementation
│   │   ├── requirements.txt      # Python dependencies
│   │   └── pyproject.toml        # Python project config
│   │
│   ├── src-tauri/                # Rust backend
│   │   ├── src/
│   │   │   ├── services/         # Core services
│   │   │   │   ├── ocr.rs        # OCR file_mgmt
│   │   │   │   ├── ai.rs         # AI/LLM integration
│   │   │   │   ├── rag.rs        # RAG coordination
│   │   │   │   └── anonymization.rs # PII protection
│   │   │   ├── processors/       # Document processors
│   │   │   │   ├── pdf_to_md.rs  # PDF extraction
│   │   │   │   └── chunking_*.rs # Text chunking
│   │   │   ├── storage/          # Database layer
│   │   │   │   ├── database.rs   # SQLite management
│   │   │   │   └── vector_db.rs  # Vector operations
│   │   │   └── config/           # Configuration
│   │   └── Cargo.toml            # Rust dependencies
│   │
│   └── vendor/tesseract-linux/   # Bundled OCR binaries
│
├── 🎨 **Frontend**
│   ├── src/
│   │   ├── index.html            # Main application UI
│   │   ├── app.js                # Application logic
│   │   ├── styles.css            # Styling
│   │   └── components/           # UI components
│   │       ├── ChatInterface.js  # Chat functionality
│   │       ├── FileUpload.js     # File handling
│   │       └── FileManager.js    # File management
│   │
├── 📚 **Documentation**
│   ├── docs/                     # Comprehensive docs
│   └── README.md                 # This file
│
└── � **Build & Config**
    ├── package.json              # NPM configuration
    └── run-script.sh             # Helper scripts
```

## 🎯 **The Privacy & Security Challenge We Solve**

Legal documents contain the most sensitive information about individuals and organizations:
- **Personal Identifiable Information (PII)**: Names, addresses, social security numbers, financial data
- **Business Intelligence**: Trade secrets, financial terms, strategic partnerships
- **Legal Vulnerabilities**: Confidential settlements, regulatory compliance issues
- **Privacy Compliance**: GDPR, HIPAA, and other regulatory requirements

**Traditional Solutions Fail Privacy:**
- Cloud-based file_mgmt exposes sensitive data
- PII travels across networks and storage systems
- Compliance risks with data residency requirements
- No control over data handling by third parties

**Law-gorithm's Innovation:**
- **Complete PII Anonymization**: Advanced NER-based pseudonymization before any file_mgmt
- **Local file_mgmt**: Desktop application runs entirely on your device
- **Zero Data Leakage**: No PII ever leaves your user environment
- **Reversible Anonymization**: Maintain pseudonym dictionary for result interpretation

---

## 🚀 **Revolutionary Features**

### 🔒 **Privacy-First file_mgmt Pipeline**
- **OCR with Anonymization**: Extract text from PDFs and immediately anonymize
- **NER-Based Pseudonymization**: Replace all named entities with contextual pseudonyms
- **Local Pseudonym Dictionary**: Maintain entity mappings within your environment
- **Zero PII Exposure**: No real names, dates, or identifiers ever processed

### 🧠 **Intelligent Anonymized Analysis**
- **Entity-Aware file_mgmt**: Understands document structure while maintaining anonymity
- **Contextual Pseudonyms**: GOVERNING_BODY_1 instead of "Karnataka Government"
- **Temporal Anonymization**: Year/month only for dates, pseudonymized key dates
- **Smart Summarization**: Clear insights without revealing sensitive information

### **Single-Application Architecture**
- **Complete Isolation**: All file_mgmt within one containerized environment
- **No Network Dependencies**: Process documents without external API calls for sensitive operations
- **Portable Privacy**: Run natively while maintaining complete confidentiality
- **Resource Efficient**: Optimized single-image deployment

### 🔍 **Anonymous Query Engine**
- **Privacy-Preserved Q&A**: Query documents using pseudonymized entities
- **Contextual Understanding**: Maintain document relationships without PII exposure
- **Citation Mapping**: Link answers to anonymized source sections
- **Result De-anonymization**: Translate results back using local pseudonym dictionary

### 📊 **Compliance & Audit**
- **Complete Audit Trails**: Track all anonymization and file_mgmt steps
- **Compliance Ready**: GDPR, HIPAA, SOX compliant by design
- **Data Residency**: Process documents without geographical data movement
- **Regulatory Reporting**: Generate compliance reports without sensitive data exposure

---

## 🏗️ **Privacy-First Architecture**

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                    LAW-GORITHM TAURI APPLICATION                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│  🌐 FRONTEND (Web UI)                                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  🦀 RUST BACKEND ORCHESTRATION                                     │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ 1️⃣ INGESTION LAYER                                                     │   │
│  │    • OCR Engine (Bundled Tesseract)                                   │   │
│  │    • PDF/Image file_mgmt                                               │   │
│  │    • Markdown Conversion                                                │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ 2️⃣ ANONYMIZATION LAYER                                                 │   │
│  │    • NER Engine (spaCy/Transformers)                                   │   │
│  │    • Entity Detection (Person, Org, Date, Location, etc.)              │   │
│  │    • Pseudonym Generation & Mapping                                    │   │
│  │    • Local Dictionary Storage                                           │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ 3️⃣ RAG file_mgmt LAYER                                                │   │
│  │    • Text Chunking (Anonymized Content)                                │   │
│  │    • Vector Embeddings (ChromaDB/FAISS)                                │   │
│  │    • Semantic Indexing                                                  │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ 4️⃣ AI INTERFACE LAYER                                                  │   │
│  │    • Google Gemini API Integration                                     │   │
│  │    • Query file_mgmt                                                   │   │
│  │    • Response Generation                                                │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ 5️⃣ DE-ANONYMIZATION LAYER                                              │   │
│  │    • Result Mapping                                                     │   │
│  │    • Pseudonym Resolution                                               │   │
│  │    • User-Friendly Output Generation                                    │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────┘

🔒 PRIVACY GUARANTEES:
• No PII ever leaves the container
• All file_mgmt done on anonymized content
• Pseudonym dictionary never exposed to external APIs
• Complete data isolation and control
```

## 🛠️ **Quick Start Guide**

### **Prerequisites**
```bash
# System Requirements (Lightweight Optimized)
- Docker 20.10+
- Modern laptop or desktop computer
- Google AI Studio API key (free tier available)
- Modern web browser

# For non-technical users:
- Any computer that can run Docker
- Internet connection for Google AI API calls (anonymized data only)
- All document file_mgmt happens locally on your device
```

### **⚡ One-Command Desktop Deployment**
```bash
# Download the installer for your platform from our releases page
# Windows: .msi installer
# macOS: .dmg installer  
# Linux: .AppImage or .deb package

echo "🏛️ Law-gorithm deployed as native desktop application"
echo "🔒 Privacy-first legal document file_mgmt is now active"
echo "⚡ Optimized for desktop performance"
```

### **🔒 Privacy Configuration**
```bash
# Maximum privacy mode (default)
docker run -d \
  --name lawgorithm \
  -p 8080:8080 \
  -e GOOGLE_AI_API_KEY=$GOOGLE_AI_API_KEY \
  -e PRIVACY_LEVEL=MAXIMUM \
  -e ANONYMIZATION_STRATEGY=COMPREHENSIVE \
  -e PRIVACY_VALIDATION_STRICT=true \
  lawgorithm:latest
```

### **🚀 Using Law-gorithm**

1. **Download**: Get the installer for your operating system from releases
2. **Install**: Run the installer (Windows .msi, macOS .dmg, Linux .AppImage/.deb)
3. **Configure**: Enter your Google AI API key in the settings
4. **Upload Documents**: Drag and drop your legal documents
5. **Ask Questions**: Query your documents using natural language
6. **Get Private Answers**: Receive insights with complete privacy protection

### **📄 OCR & Document file_mgmt**

Law-gorithm automatically processes uploaded documents using advanced OCR technology:

**Supported Formats:**
- 📄 **PDF Documents**: Full text extraction with OCR fallback
- 🖼️ **Images**: PNG, JPEG, TIFF with embedded text recognition
- 📝 **Office Documents**: DOCX, DOC (converted to markdown)
- 📋 **Plain Text**: TXT, MD files (direct file_mgmt)

**OCR Features:**
- 🔍 **Smart Text Detection**: Automatically detects if PDF has embedded text or needs OCR
- 📝 **Markdown Conversion**: Converts all documents to clean markdown format
- 🎯 **Layout Preservation**: Maintains document structure and formatting
- ⚡ **Performance Optimized**: Efficient file_mgmt with minimal resource usage
- 🌐 **Multi-language Support**: Configurable language detection and file_mgmt

**Automatic file_mgmt:**
- Upload any supported document → Automatic OCR file_mgmt → Markdown output
- No manual intervention required
- Results stored securely in your local document directory

---

## 📁 **Repository Structure**

```
Law-gorithm/
├── src-tauri/              # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── lib.rs          # Library exports
│   │   ├── config/         # Configuration management
│   │   ├── services/       # Core services (anonymization, RAG, AI)
│   │   ├── processors/     # PDF→MD conversion & chunking
│   │   ├── storage/        # Vector database management
│   │   └── utils/          # Utility functions
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html          # Main UI
│   ├── styles.css          # Styling
│   ├── app.js              # Frontend logic
│   └── components/         # UI components
├── python_modules/         # Python for RAG operations
│   ├── rag_processor.py    # Python RAG implementation
│   ├── requirements.txt    # Python dependencies
│   └── pyproject.toml      # Python package config
├── public/                 # Static assets
├── package.json            # npm scripts
├── docs/                   # Comprehensive documentation
├── scripts/               # Utility scripts
│   ├── verification/      # Verification and validation scripts
│   └── README.md          # Scripts documentation
└── README.md               # This file
```

### **🔧 Development Setup**
```bash
# Install dependencies
npm install
pip install -r python_modules/requirements.txt

# Start development
npm run tauri dev
```

### **🛠️ Utility Scripts**
```bash
# Run verification scripts
./run-script.sh verification verify_tesseract

# Or run scripts directly
./scripts/verification/verify_tesseract.sh

# List available scripts
./run-script.sh --help
```

**Available Scripts:**
- `verify_tesseract` - Verify bundled Tesseract OCR setup
- More scripts will be added as the project grows

---

## 💡 **Use Cases**
```
# Install Rust, Node.js, and Python
# Download installer for your platform from releases
# Configure Google AI API key in settings

echo "🏛️ Law-gorithm deployed as native desktop application"
echo "🔒 Privacy-first legal document file_mgmt is now active"
echo "⚡ Optimized for desktop performance"
```

### **🔧 Development Setup**
```bash
# Install dependencies
npm install
pip install -r python_modules/requirements.txt

# Start development
npm run tauri dev
```

---

## 💡 **Use Cases**

### 🏠 **For Individuals**
- **Rental Agreements**: Understand your rights and obligations
- **Employment Contracts**: Decode compensation and termination clauses
- **Terms of Service**: Know what you're agreeing to online
- **Loan Documents**: Identify hidden fees and payment terms

### 🏢 **For Small Businesses**
- **Vendor Contracts**: Negotiate from a position of knowledge
- **Partnership Agreements**: Understand profit-sharing and responsibilities
- **Insurance Policies**: Decode coverage and exclusions
- **Licensing Agreements**: Understand usage rights and restrictions

### ⚖️ **For Legal Professionals**
- **Document Review**: Accelerate initial document analysis
- **Client Communication**: Generate plain-English summaries
- **Contract Comparison**: Identify differences between document versions
- **Training Tool**: Educational resource for junior attorneys

---

## 🎨 **Demo Scenarios**

### **Scenario 1: The Overwhelmed Renter**
*Sarah receives a 47-page apartment lease*
1. **Upload** the lease document
2. **Ask**: "What are my obligations for maintenance and repairs?"
3. **Receive**: Clear breakdown of responsibilities with specific clause references
4. **Export**: Summary report for future reference

### **Scenario 2: The Cautious Entrepreneur**
*Mike reviews a complex partnership agreement*
1. **Upload** the partnership document
2. **Query**: "What happens if I want to leave the partnership?"
3. **Discover**: Exit clauses, financial implications, and notice requirements
4. **Flag**: Potentially unfavorable terms for negotiation

---

## 🔒 **Privacy & Security**

- **Zero-Storage Policy**: Documents processed and immediately discarded
- **End-to-End Encryption**: All communications secured with TLS 1.3
- **Anonymized file_mgmt**: Personal information stripped before AI analysis
- **Compliance Ready**: GDPR, CCPA, and SOC 2 compliant architecture
- **Open Source**: Transparent algorithms and file_mgmt methods

---

## 🤝 **Contributing**

We welcome contributions from legal professionals, developers, and AI researchers!

```bash
# Fork the repository
git fork https://github.com/GR4Y-SCALE/Law-gorithm.git

# Create a feature branch
git checkout -b feature/amazing-enhancement

# Make your changes and commit
git commit -m "Add amazing legal feature"

# Push and create a pull request
git push origin feature/amazing-enhancement
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 📜 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 📞 **Contact & Support**

- **Team**: GR4Y-SCALE Development
- **Email**: adper0705@gmail.com
- **Repository**: [GitHub - Law-gorithm](https://github.com/GR4Y-SCALE/Law-gorithm)
- **Issues**: [GitHub Issues](https://github.com/GR4Y-SCALE/Law-gorithm/issues)
- **Discussions**: [GitHub Discussions](https://github.com/GR4Y-SCALE/Law-gorithm/discussions)

---

<div align="center">

### *"Bridging the gap between legal complexity and human understanding"*

**Built with ❤️ for the Google Generative AI Hackathon 2025**

[![Google Cloud](https://img.shields.io/badge/Powered%20by-Google%20Cloud-4285F4?style=flat-square&logo=google-cloud)](https://cloud.google.com/)
[![AI/ML](https://img.shields.io/badge/Enhanced%20with-Generative%20AI-FF6B6B?style=flat-square)](https://ai.google/)

</div>
