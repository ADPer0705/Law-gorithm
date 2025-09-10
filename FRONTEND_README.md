# Frontend Development Guide - Law-gorithm UI

## 🎨 **Frontend Team Responsibilities**

This guide is specifically for the **Frontend Team** working on:
- **Web User Interface** - HTML/CSS/JavaScript application
- **File Management** - Upload, display, and organize legal documents  
- **Chat Interface** - Interactive Q&A with processed documents
- **Tauri Integration** - Frontend-to-Rust command communication
- **User Experience** - Responsive design and accessibility

## 🏗️ **Frontend Architecture**

The frontend is a **web application** served by a Python HTTP server and integrated with Tauri:

```
┌─────────────────────┐
│   Web Browser       │
│   (User Interface)  │
└─────────┬───────────┘
          │
          ▼ (HTTP: localhost:8080)
┌─────────────────────┐
│   Python Server     │
│   (Static Files)    │  
└─────────┬───────────┘
          │
          ▼ (Tauri Commands)
┌─────────────────────┐
│   Rust Backend      │
│   (Business Logic)  │
└─────────────────────┘
```

## 📁 **Frontend File Structure**

```
src/                           # Frontend source files
├── index.html                 # 🔥 Main application page
├── app.js                     # 🔥 Core application logic
├── styles.css                 # 🔥 Application styling
│
└── components/                # UI components
    ├── README.md              # Component documentation
    ├── ChatInterface.js       # 🔥 Chat and Q&A interface
    ├── FileUpload.js          # 🔥 File upload handling  
    └── FileManager.js         # 🔥 Document list and management

public/                        # Static assets (if any)
package.json                   # NPM configuration
```

## 🔥 **Key Frontend Components**

### **1. Main Application (`src/index.html`)**
- **Purpose**: Primary application layout and structure
- **Features**: 
  - Responsive design
  - Component mounting points
  - Tauri app integration
  - Loading states and error handling

### **2. Application Logic (`src/app.js`)**
- **Purpose**: Core JavaScript functionality and state management
- **Features**:
  - Tauri command invocation
  - Component coordination
  - Event handling
  - Application state management

### **3. Chat Interface (`src/components/ChatInterface.js`)**
- **Purpose**: Interactive chat for document Q&A
- **Features**:
  - Message display and input
  - Document-specific queries
  - Response formatting
  - Chat history management

### **4. File Upload (`src/components/FileUpload.js`)**
- **Purpose**: Handle document upload and processing
- **Features**:
  - Drag-and-drop support
  - File validation (PDF, images)
  - Upload progress indication
  - Error handling and feedback

### **5. File Manager (`src/components/FileManager.js`)**  
- **Purpose**: Display and manage uploaded documents
- **Features**:
  - Document list display
  - Metadata viewing
  - Delete functionality
  - Processing status indicators

## 🚀 **Development Workflow**

### **Setting Up Frontend Development**
```bash
# 1. Start the frontend server
cd law-gorithm
npm run serve  # Starts HTTP server on localhost:8080

# 2. Start Tauri in development mode (separate terminal)
npm run tauri dev  # Hot reload for frontend changes

# 3. Open browser for testing
# http://localhost:8080 (if running without Tauri)
```

### **Frontend Development Process**
1. **Edit frontend files** in `src/`
2. **Refresh browser** to see changes (automatic with live server)
3. **Test Tauri commands** using browser developer tools
4. **Debug with console logs** and network inspection

### **Working with Components**
```javascript
// Example component structure
class ChatInterface {
    constructor() {
        this.initializeElements();
        this.bindEvents();
    }
    
    initializeElements() {
        this.chatContainer = document.getElementById('chat-container');
        this.messageInput = document.getElementById('message-input');
    }
    
    bindEvents() {
        this.messageInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                this.sendMessage();
            }
        });
    }
}
```

## 🔧 **Tauri Frontend Integration**

### **Available Tauri Commands**

The frontend communicates with the Rust backend through these commands:

#### **Document Operations**
```javascript
// Upload document
await window.__TAURI__.invoke('upload_document', {
    filePath: '/path/to/file.pdf',
    fileName: 'contract.pdf',
    fileSize: 1024000
});

// Get document list  
const documents = await window.__TAURI__.invoke('get_documents');

// Delete document
await window.__TAURI__.invoke('delete_document', {
    documentId: 'doc123'
});
```

#### **Search & Retrieval**
```javascript
// Search documents
const results = await window.__TAURI__.invoke('search_documents', {
    query: 'contract terms',
    limit: 5
});

// Get document chunks
const chunks = await window.__TAURI__.invoke('get_document_chunks', {
    documentId: 'doc123'
});
```

#### **AI Integration**
```javascript
// Generate document summary
const summary = await window.__TAURI__.invoke('generate_summary', {
    documentId: 'doc123'  
});

// Ask question about document
const answer = await window.__TAURI__.invoke('ask_question', {
    documentId: 'doc123',
    query: 'What are the key terms?'
});
```

#### **Application State**
```javascript
// Get application status
const status = await window.__TAURI__.invoke('get_app_status');

// Get processing status
const processing = await window.__TAURI__.invoke('get_processing_status');
```

### **Error Handling Pattern**
```javascript
async function safeInvoke(command, args) {
    try {
        const result = await window.__TAURI__.invoke(command, args);
        return { success: true, data: result };
    } catch (error) {
        console.error(`Command ${command} failed:`, error);
        return { success: false, error: error.message };
    }
}

// Usage
const result = await safeInvoke('search_documents', { query: 'terms' });
if (result.success) {
    displayResults(result.data);
} else {
    showError(result.error);
}
```

## 🎨 **UI Design Guidelines**

### **Color Scheme & Branding**
```css
:root {
    --primary-color: #2563eb;      /* Blue - trust, legal */
    --secondary-color: #10b981;    /* Green - success, secure */
    --accent-color: #f59e0b;       /* Amber - attention, warning */
    --text-primary: #1f2937;       /* Dark gray */
    --text-secondary: #6b7280;     /* Medium gray */
    --background: #ffffff;         /* White */
    --surface: #f9fafb;           /* Light gray */
    --error: #ef4444;             /* Red */
}
```

### **Component Styling Standards**
```css
/* Button styles */
.btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    transition: all 0.2s;
}

.btn-primary {
    background-color: var(--primary-color);
    color: white;
}

.btn-primary:hover {
    background-color: #1d4ed8;
}

/* Card styles */
.card {
    background: var(--background);
    border-radius: 0.5rem;
    padding: 1.5rem;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}
```

### **Responsive Design**
```css
/* Mobile-first approach */
@media (min-width: 768px) {
    /* Tablet styles */
}

@media (min-width: 1024px) {
    /* Desktop styles */
}
```

## 📱 **Component Specifications**

### **ChatInterface Component**
```javascript
class ChatInterface {
    // Required methods:
    // - sendMessage(message)
    // - displayMessage(message, isUser)  
    // - displayResponse(response)
    // - clearChat()
    // - showTyping()
    // - hideTyping()
}
```

### **FileUpload Component**
```javascript
class FileUpload {
    // Required methods:
    // - handleDragOver(event)
    // - handleDrop(event)
    // - handleFileSelect(files)
    // - uploadFile(file)
    // - showProgress(percentage)
    // - showSuccess(message)
    // - showError(error)
}
```

### **FileManager Component**
```javascript
class FileManager {
    // Required methods:
    // - loadDocuments()
    // - displayDocuments(documents)
    // - deleteDocument(id)
    // - selectDocument(id)
    // - showDocumentDetails(document)
    // - refreshList()
}
```

## 🧪 **Testing & Debugging**

### **Browser Testing**
```javascript
// Test Tauri commands in browser console
window.__TAURI__.invoke('get_app_status').then(console.log);

// Test component functionality
const fileManager = new FileManager();
fileManager.loadDocuments();
```

### **Development Tools**
- **Browser DevTools**: Inspect elements, debug JavaScript, monitor network
- **Vue/React DevTools**: If using frameworks (future enhancement)
- **Tauri DevTools**: Debug Rust-JavaScript communication

### **Common Issues & Solutions**

#### **Tauri Commands Not Working**
```javascript
// Check if Tauri is available
if (window.__TAURI__) {
    // Tauri commands available
} else {
    // Fallback for browser-only testing
    console.warn('Tauri not available, using mock data');
}
```

#### **CORS Issues with Local Server**
```bash
# Start server with CORS headers if needed
python3 -m http.server 8080 --bind 0.0.0.0
```

## 🔄 **Frontend-Backend Integration**

### **Data Flow Pattern**
```
User Action (Frontend)
    ↓
JavaScript Event Handler
    ↓
Tauri Command Invocation
    ↓
Rust Backend Processing
    ↓
Response to Frontend
    ↓
UI Update
```

### **State Management**
```javascript
// Simple state management pattern
const AppState = {
    currentDocument: null,
    documents: [],
    chatHistory: [],
    isProcessing: false,
    
    updateDocument(doc) {
        this.currentDocument = doc;
        this.notifyComponents();
    },
    
    notifyComponents() {
        // Update UI components
        document.dispatchEvent(new CustomEvent('stateChanged'));
    }
};
```

## 📋 **Frontend Development Checklist**

### **High Priority Tasks**
- [ ] Complete ChatInterface implementation
- [ ] Improve FileUpload drag-and-drop UX
- [ ] Add document processing status indicators
- [ ] Implement error handling and user feedback
- [ ] Add loading states for all async operations

### **Medium Priority**
- [ ] Enhance responsive design for mobile
- [ ] Add keyboard shortcuts and accessibility
- [ ] Implement document preview functionality
- [ ] Add user preferences and settings
- [ ] Create help and onboarding flows

### **Future Enhancements**
- [ ] Real-time processing updates via websockets
- [ ] Advanced document filtering and search
- [ ] Collaborative features for team usage
- [ ] Offline mode support
- [ ] PWA (Progressive Web App) capabilities

## 🐛 **Debugging Tips**

### **JavaScript Debugging**
```javascript
// Add debug logging
const DEBUG = true;
function debugLog(...args) {
    if (DEBUG) console.log('[DEBUG]', ...args);
}

// Use try-catch for error tracking  
async function handleUserAction() {
    try {
        debugLog('Starting user action');
        const result = await window.__TAURI__.invoke('some_command');
        debugLog('Command result:', result);
        return result;
    } catch (error) {
        console.error('User action failed:', error);
        showUserError('Operation failed. Please try again.');
    }
}
```

### **Network & Performance**
- **Monitor API calls** in browser Network tab
- **Check response times** for Tauri commands
- **Profile JavaScript performance** with browser tools
- **Monitor memory usage** for large document processing

## 📞 **Frontend Support & Communication**

- **UI Issues**: Layout, styling, responsiveness, component behavior
- **Tauri Integration**: Command communication, data binding, state management  
- **User Experience**: Accessibility, usability, performance, error handling
- **Browser Compatibility**: Cross-browser testing and polyfills

---

**🔥 Priority Focus**: ChatInterface, FileUpload, and FileManager are the core components that drive the user experience. Focus on making these robust and user-friendly.
