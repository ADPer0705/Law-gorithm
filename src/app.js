// Add Tauri window API imports
let tauriWindowApi;
if (window.__TAURI__) {
    tauriWindowApi = window.__TAURI__.window.getCurrent();
}

// LawGorithm - Elite Legal Intelligence Platform
// Enhanced professional interface with advanced features

class LawGorithm {
    constructor() {
        this.sessionStartTime = Date.now();
        this.documentsAnalyzed = 6;
        this.queriesCount = 0;
        this.init();
    }

    init() {
        this.setupThemeToggle();
        this.setupChat();
        this.setupFileUpload();
        this.setupSessionTimer();
        this.setupKeyboardShortcuts();
        this.setupGlossary();
        this.setupAnalytics();
        this.setupWindowControls();
        this.setupClearDataFeature();
        this.setupRefreshButton();
        this.loadFiles(); // Load files on startup
    }

    // Enhanced theme management
    setupThemeToggle() {
        const themeToggle = document.getElementById("theme-toggle");
        if (themeToggle) {
            themeToggle.addEventListener("click", () => {
                const html = document.documentElement;
                const currentTheme = html.dataset.theme || "dark";
                const newTheme = currentTheme === "light" ? "dark" : "light";
                html.dataset.theme = newTheme;
                
                // Smooth theme transition
                document.body.style.transition = "all 0.3s ease";
                setTimeout(() => {
                    document.body.style.transition = "";
                }, 300);
            });
        }
    }

    // Professional chat system
    setupChat() {
        const chatBox = document.getElementById("chat-area");
        const input = document.getElementById("message-input");
        const form = document.getElementById("message-form");

        if (!chatBox || !input || !form) return;

        const addMessage = (text, sender = "user", metadata = {}) => {
            const msg = document.createElement("div");
            msg.className = `message message-${sender} animate-fade-in-up`;
            
            if (sender === "user") {
                msg.innerHTML = `
                    <div class="message-content user">
                        <div class="flex items-start">
                            <div class="flex-1">
                                <p>${text}</p>
                                <div class="text-xs text-blue-200 mt-2 opacity-70">
                                    <i class="fas fa-clock mr-1"></i>
                                    ${new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                                </div>
                            </div>
                            <div class="w-8 h-8 bg-white/20 rounded-full flex items-center justify-center ml-3 mt-1">
                                <i class="fas fa-user text-white text-sm"></i>
                            </div>
                        </div>
                    </div>
                `;
            } else {
                msg.innerHTML = `
                    <div class="message-content bot">
                        <div class="flex items-start">
                            <div class="w-8 h-8 bg-gradient-to-r from-blue-500 to-indigo-600 rounded-full flex items-center justify-center mr-3 mt-1">
                                <i class="fas fa-robot text-white text-sm"></i>
                            </div>
                            <div class="flex-1">
                                <p class="font-semibold text-blue-400 mb-1">LawGorithm Analysis</p>
                                <p class="mb-3">${text}</p>
                                ${metadata.sources && metadata.sources.length > 0 ? `
                                    <div class="mb-3">
                                        <p class="text-xs text-gray-400 mb-2">Sources referenced:</p>
                                        <div class="flex flex-wrap gap-2">
                                            ${metadata.sources.map(source => 
                                                `<span class="px-2 py-1 bg-blue-500/20 text-blue-400 rounded text-xs">${source}</span>`
                                            ).join('')}
                                        </div>
                                    </div>
                                ` : ''}
                                <div class="flex items-center justify-between text-xs text-gray-400 pt-2 border-t border-gray-700/30">
                                    <div class="flex items-center">
                                        <i class="fas fa-clock mr-1"></i>
                                        ${new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                                    </div>
                                    <div class="flex items-center space-x-3">
                                        <button class="hover:text-blue-400 transition-colors" title="Helpful">
                                            <i class="fas fa-thumbs-up"></i>
                                        </button>
                                        <button class="hover:text-red-400 transition-colors" title="Not helpful">
                                            <i class="fas fa-thumbs-down"></i>
                                        </button>
                                        <button class="hover:text-gray-300 transition-colors" title="Copy response">
                                            <i class="fas fa-copy"></i>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                `;
            }
            
            chatBox.appendChild(msg);
            chatBox.scrollTop = chatBox.scrollHeight;
        };

        // Enhanced send functionality
        const handleSend = async () => {
            const text = input.value.trim();
            if (!text) return;
            
            addMessage(text, "user");
            input.value = "";
            this.queriesCount++;
            this.updateAnalytics();

            // Show typing indicator
            const typingMessage = this.addTypingIndicator();

            try {
                // Call the actual backend query function if Tauri is available
                if (window.__TAURI__ && window.__TAURI__.invoke) {
                    const result = await window.__TAURI__.invoke('query', text);
                    
                    // Remove typing indicator
                    this.removeTypingIndicator(typingMessage);
                    
                    if (result.success) {
                        addMessage(result.response, "bot", { 
                            sources: result.relevant_sources || []
                        });
                    } else {
                        addMessage("I encountered an error processing your query. Please try again.", "bot");
                    }
                    
                    // Refresh file list after query in case any processing occurred
                    this.loadFiles();
                } else {
                    // Fallback for development without Tauri
                    await this.handleFallbackQuery(text, addMessage, typingMessage);
                }
            } catch (error) {
                console.error("Query failed:", error);
                this.removeTypingIndicator(typingMessage);
                addMessage("I encountered an error processing your query. Please try again.", "bot");
                this.showNotification("Query failed", "error");
            }
        };

        // Event listeners
        if (form) {
            form.addEventListener("submit", (e) => {
                e.preventDefault();
                handleSend();
            });
        }

        // Auto-resize input
        if (input) {
            input.addEventListener('input', function() {
                this.style.height = 'auto';
                this.style.height = this.scrollHeight + 'px';
            });
        }
    }

    // Add typing indicator
    addTypingIndicator() {
        const chatBox = document.getElementById("chat-area");
        const typingMsg = document.createElement("div");
        typingMsg.className = "message message-bot animate-fade-in-up typing-indicator";
        typingMsg.id = "typing-indicator";
        
        typingMsg.innerHTML = `
            <div class="message-content bot">
                <div class="flex items-start">
                    <div class="w-8 h-8 bg-gradient-to-r from-blue-500 to-indigo-600 rounded-full flex items-center justify-center mr-3 mt-1">
                        <i class="fas fa-robot text-white text-sm"></i>
                    </div>
                    <div class="flex-1">
                        <p class="font-semibold text-blue-400 mb-1">LawGorithm is analyzing...</p>
                        <div class="flex items-center space-x-1">
                            <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce"></div>
                            <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                            <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        chatBox.appendChild(typingMsg);
        chatBox.scrollTop = chatBox.scrollHeight;
        return typingMsg;
    }

    // Remove typing indicator
    removeTypingIndicator(typingMessage) {
        if (typingMessage && typingMessage.parentElement) {
            typingMessage.remove();
        }
    }

    // Fallback query handler for development
    async handleFallbackQuery(text, addMessage, typingMessage) {
        const responses = [
            {
                text: "Based on my analysis of your uploaded documents, I've identified several key legal considerations. The contract terms show standard liability clauses, but Section 12 regarding indemnification needs review.",
                sources: ["Contract Agreement v3.pdf", "Legal Analysis Report"]
            },
            {
                text: "I've found relevant case precedents that align with your legal query. The motion follows established requirements, though jurisdictional arguments could benefit from additional citations.",
                sources: ["Motion to Dismiss.pdf", "Case Precedents Database"]
            },
            {
                text: "Your deposition transcript reveals strategic opportunities for cross-examination. I've identified 3 inconsistencies and 2 areas for impeachment evidence.",
                sources: ["Deposition Jane Doe.docx", "Evidence Analysis"]
            }
        ];

        // Simulate processing time
        await new Promise(resolve => setTimeout(resolve, Math.random() * 2000 + 1000));
        
        this.removeTypingIndicator(typingMessage);
        const response = responses[Math.floor(Math.random() * responses.length)];
        addMessage(response.text, "bot", { sources: response.sources });
        
        // Refresh file list after query
        this.loadFiles();
    }

    // Professional file upload system
    setupFileUpload() {
        const uploadBtn = document.getElementById("upload-btn");
        if (uploadBtn) {
            uploadBtn.addEventListener("click", () => {
                // Create file input
                const fileInput = document.createElement('input');
                fileInput.type = 'file';
                fileInput.multiple = true;
                fileInput.accept = '.pdf,.doc,.docx,.txt,.rtf';
                
                fileInput.onchange = async (e) => {
                    const files = Array.from(e.target.files);
                    if (files.length > 0) {
                        await this.handleFileUpload(files);
                    }
                };
                
                fileInput.click();
            });
        }
    }

    async handleFileUpload(files) {
        try {
            // Show upload progress notification
            this.showNotification(`Uploading ${files.length} file(s)...`, 'info', 2000);
            
            // Get current files from backend to merge with new ones
            let currentFiles = [];
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                currentFiles = await window.__TAURI__.invoke('list_files');
            }
            
            // Create new file entries for upload
            const newFiles = files.map(file => ({
                id: this.generateFileId(file.name),
                path: file.path || file.name, // Use file.path if available, otherwise filename
                name: file.name
            }));
            
            // Combine current files with new files
            const allFiles = [
                ...currentFiles.map(f => ({ id: f.id, path: f.original_path })),
                ...newFiles.map(f => ({ id: f.id, path: f.path }))
            ];
            
            // Call update_files with the combined file list
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                const result = await window.__TAURI__.invoke('update_files', allFiles);
                
                if (result.success) {
                    this.showNotification(`Successfully uploaded ${files.length} file(s)`, 'success');
                    // Refresh the file list to show updated files
                    await this.loadFiles();
                } else {
                    throw new Error('Update files failed');
                }
            } else {
                // Fallback for development without Tauri
                console.log('Tauri not available, simulating file upload');
                await this.simulateFileUpload(files);
            }
            
        } catch (error) {
            console.error("File upload failed:", error);
            this.showNotification(`Failed to upload files: ${error.message}`, 'error');
        }
    }
    
    // Generate a unique ID for a file
    generateFileId(filename) {
        const timestamp = Date.now();
        const random = Math.random().toString(36).substr(2, 9);
        const cleanName = filename.replace(/[^a-zA-Z0-9]/g, '_');
        return `${cleanName}_${timestamp}_${random}`;
    }
    
    // Fallback simulation for development
    async simulateFileUpload(files) {
        for (let i = 0; i < files.length; i++) {
            await new Promise(resolve => setTimeout(resolve, 500));
            this.showNotification(`${files[i].name} uploaded successfully`, 'success', 1500);
        }
        // Refresh file list after simulation
        await this.loadFiles();
    }

    // Session timer
    setupSessionTimer() {
        const updateTimer = () => {
            const elapsed = Date.now() - this.sessionStartTime;
            const minutes = Math.floor(elapsed / 60000);
            const seconds = Math.floor((elapsed % 60000) / 1000);
            const timerElement = document.getElementById('session-timer');
            if (timerElement) {
                timerElement.textContent = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
            }
        };
        
        setInterval(updateTimer, 1000);
    }

    // Professional keyboard shortcuts
    setupKeyboardShortcuts() {
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey || e.metaKey) {
                switch(e.key) {
                    case '/':
                        e.preventDefault();
                        this.focusInput();
                        break;
                    case 'k':
                        e.preventDefault();
                        this.focusInput();
                        break;
                    case 'u':
                        e.preventDefault();
                        document.getElementById('upload-btn')?.click();
                        break;
                }
            }
            
            if (e.key === 'Escape') {
                this.clearInput();
            }
        });
    }

    focusInput() {
        const input = document.getElementById("message-input");
        if (input) {
            input.focus();
            input.select();
        }
    }

    clearInput() {
        const input = document.getElementById("message-input");
        if (input) {
            input.value = "";
        }
    }

    // Enhanced glossary system
    setupGlossary() {
        const glossaryTerms = {
            "Tort": "A wrongful act that injures another person or their property, leading to civil legal liability",
            "Precedent": "A legal decision that serves as an authoritative rule in future similar cases (stare decisis)",
            "Liability": "Legal responsibility for one's acts or omissions that may result in damages or penalties",
            "Jurisdiction": "The authority of a court to hear and decide cases within a specific geographic area or subject matter",
            "Indemnification": "Compensation for harm, loss, or damages incurred, often through contractual provisions",
            "Discovery": "The pre-trial phase where parties exchange information and evidence relevant to the case",
            "Depositions": "Sworn out-of-court testimony taken during the discovery phase of litigation"
        };

        // Add click handlers for glossary terms
        Object.keys(glossaryTerms).forEach(term => {
            const elements = document.querySelectorAll(`[data-term="${term}"]`);
            elements.forEach(el => {
                el.addEventListener('click', () => {
                    this.showGlossaryDefinition(term, glossaryTerms[term]);
                });
            });
        });
    }

    showGlossaryDefinition(term, definition) {
        this.showNotification(`${term}: ${definition}`, 'info', 5000);
    }

    // Analytics and insights
    setupAnalytics() {
        this.updateAnalytics();
    }

    updateAnalytics() {
        const docsElement = document.querySelector('[data-metric="documents"]');
        const queriesElement = document.querySelector('[data-metric="queries"]');
        
        if (docsElement) docsElement.textContent = this.documentsAnalyzed;
        if (queriesElement) queriesElement.textContent = this.queriesCount;
    }

    // Professional notification system
    showNotification(message, type = 'info', duration = 3000) {
        const notification = document.createElement('div');
        notification.className = `fixed top-4 right-4 p-4 rounded-lg shadow-lg z-50 animate-slide-in-right ${
            type === 'success' ? 'bg-green-600' : 
            type === 'error' ? 'bg-red-600' : 
            'bg-blue-600'
        } text-white`;
        
        notification.innerHTML = `
            <div class="flex items-center">
                <i class="fas fa-${
                    type === 'success' ? 'check-circle' : 
                    type === 'error' ? 'exclamation-circle' : 
                    'info-circle'
                } mr-2"></i>
                <span>${message}</span>
                <button class="ml-4 text-white/80 hover:text-white" onclick="this.parentElement.parentElement.remove()">
                    <i class="fas fa-times"></i>
                </button>
            </div>
        `;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            if (notification.parentElement) {
                notification.remove();
            }
        }, duration);
    }

    // Add window controls for minimize, fullscreen, and close
    setupWindowControls() {
        // Only run if Tauri APIs are available
        if (!window.__TAURI__ || !window.__TAURI__.window) return;

        // Minimize
        const minimizeBtn = document.querySelector('.chat-header button:nth-child(1) i.fa-minus')?.parentElement;
        if (minimizeBtn) {
            minimizeBtn.addEventListener('click', async () => {
                try {
                    await window.__TAURI__.window.getCurrent().minimize();
                } catch (e) {}
            });
        }

        // Fullscreen (toggle maximize)
        const fullscreenBtn = document.querySelector('.chat-header button:nth-child(2) i.fa-square, .chat-header button:nth-child(2) i.far.fa-square')?.parentElement;
        if (fullscreenBtn) {
            fullscreenBtn.addEventListener('click', async () => {
                try {
                    const win = window.__TAURI__.window.getCurrent();
                    const isMaximized = await win.isMaximized();
                    if (isMaximized) {
                        await win.unmaximize();
                    } else {
                        await win.maximize();
                    }
                } catch (e) {}
            });
        }

        // Close
        const closeBtn = document.querySelector('.chat-header button:nth-child(3) i.fa-times')?.parentElement;
        if (closeBtn) {
            closeBtn.addEventListener('click', async () => {
                try {
                    await window.__TAURI__.window.getCurrent().close();
                } catch (e) {}
            });
        }
    }

    // Clear all data feature with confirmation
    setupClearDataFeature() {
        const clearDataBtn = document.getElementById("clear-data-btn");
        const modal = document.getElementById("clear-data-modal");
        const cancelBtn = document.getElementById("cancel-clear-btn");
        const confirmBtn = document.getElementById("confirm-clear-btn");

        if (!clearDataBtn || !modal || !cancelBtn || !confirmBtn) return;

        // Show confirmation modal
        clearDataBtn.addEventListener("click", () => {
            modal.classList.remove("hidden");
            document.body.style.overflow = "hidden"; // Prevent background scrolling
        });

        // Cancel button - hide modal
        cancelBtn.addEventListener("click", () => {
            this.hideModal();
        });

        // Confirm button - actually clear the data
        confirmBtn.addEventListener("click", async () => {
            await this.clearAllData();
        });

        // Close modal when clicking outside
        modal.addEventListener("click", (e) => {
            if (e.target === modal) {
                this.hideModal();
            }
        });

        // Close modal on Escape key
        document.addEventListener("keydown", (e) => {
            if (e.key === "Escape" && !modal.classList.contains("hidden")) {
                this.hideModal();
            }
        });
    }

    hideModal() {
        const modal = document.getElementById("clear-data-modal");
        if (modal) {
            modal.classList.add("hidden");
            document.body.style.overflow = ""; // Restore scrolling
        }
    }

    async clearAllData() {
        const confirmBtn = document.getElementById("confirm-clear-btn");
        
        if (!confirmBtn) return;

        // Show loading state
        const originalContent = confirmBtn.innerHTML;
        confirmBtn.innerHTML = '<i class="fas fa-spinner fa-spin mr-2"></i>Clearing...';
        confirmBtn.disabled = true;

        try {
            // Check if Tauri is available
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                // Call the Tauri command
                await window.__TAURI__.invoke('delete_all_user_data');
                
                // Success - reset UI state
                this.documentsAnalyzed = 0;
                this.queriesCount = 0;
                this.updateAnalytics();
                
                // Reload files from backend to refresh the list
                await this.loadFiles();
                
                // Clear chat messages
                const chatArea = document.getElementById("chat-area");
                if (chatArea) {
                    chatArea.innerHTML = `
                        <div class="message message-bot animate-fade-in-up">
                            <div class="message-content bot">
                                <div class="flex items-start">
                                    <div class="w-8 h-8 bg-gradient-to-r from-blue-500 to-indigo-600 rounded-full flex items-center justify-center mr-3 mt-1">
                                        <i class="fas fa-robot text-white text-sm"></i>
                                    </div>
                                    <div>
                                        <p class="font-semibold text-blue-400 mb-2">Welcome to LawGorithm</p>
                                        <p class="text-gray-300 leading-relaxed">
                                            Your elite legal intelligence platform is ready. Upload your documents and ask me anything about your case. 
                                            I can analyze contracts, review depositions, research precedents, and help draft legal documents.
                                        </p>
                                        <div class="flex items-center mt-3 text-xs text-gray-400">
                                            <i class="fas fa-shield-alt text-green-400 mr-2"></i>
                                            All conversations are protected by attorney-client privilege
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    `;
                }

                this.hideModal();
                this.showNotification("All data has been successfully cleared", "success");
                
            } else {
                // Tauri not available - show error
                throw new Error("Application backend not available");
            }
        } catch (error) {
            console.error("Failed to clear data:", error);
            this.showNotification("Failed to clear data. Please try again.", "error");
        } finally {
            // Restore button state
            confirmBtn.innerHTML = originalContent;
            confirmBtn.disabled = false;
        }
    }

    // Setup refresh button for file list
    setupRefreshButton() {
        const refreshBtn = document.getElementById("refresh-files-btn");
        if (refreshBtn) {
            refreshBtn.addEventListener("click", () => {
                const icon = refreshBtn.querySelector('i');
                icon.classList.add('fa-spin');
                this.loadFiles().finally(() => {
                    setTimeout(() => icon.classList.remove('fa-spin'), 500);
                });
            });
        }
    }

    // Load and display files from backend
    async loadFiles() {
        try {
            // Check if Tauri is available
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                const files = await window.__TAURI__.invoke('list_files');
                this.renderFileList(files);
                this.updateDocumentCount(files.length);
            } else {
                // Fallback for development/testing without Tauri
                console.warn('Tauri not available, using mock data');
                this.renderFileList([]);
                this.updateDocumentCount(0);
            }
        } catch (error) {
            console.error("Failed to load files:", error);
            this.showNotification("Failed to load files", "error");
            this.renderFileList([]);
            this.updateDocumentCount(0);
        }
    }

    // Render the file list in the UI
    renderFileList(files) {
        const documentList = document.getElementById('document-list');
        if (!documentList) return;

        if (files.length === 0) {
            documentList.innerHTML = `
                <div class="text-center py-8 text-gray-400">
                    <i class="fas fa-folder-open text-3xl mb-3 opacity-50"></i>
                    <p class="text-sm">No documents uploaded</p>
                    <p class="text-xs text-gray-500 mt-1">Upload files to get started</p>
                </div>
            `;
            return;
        }

        // Create file items
        const fileItems = files.map(file => {
            const extension = this.getFileExtension(file.name);
            const iconInfo = this.getFileIcon(extension);
            
            return `
                <div class="document-item flex items-center p-3 rounded-lg text-gray-300 hover:text-white cursor-pointer" data-file-id="${file.id}">
                    <div class="w-8 h-8 ${iconInfo.bgColor} rounded-lg flex items-center justify-center mr-3">
                        <i class="${iconInfo.icon} ${iconInfo.color} text-sm"></i>
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium truncate" title="${file.name}">${file.name}</p>
                        <p class="text-xs text-gray-500">${file.original_path}</p>
                    </div>
                    <button class="ml-2 p-1 hover:bg-red-600/20 rounded text-red-400 hover:text-red-300 transition-colors" 
                            onclick="window.lawGorithm.removeFile('${file.id}')" 
                            title="Remove file">
                        <i class="fas fa-times text-xs"></i>
                    </button>
                </div>
            `;
        }).join('');

        documentList.innerHTML = fileItems;
    }

    // Get file extension from filename
    getFileExtension(filename) {
        return filename.split('.').pop().toLowerCase();
    }

    // Get appropriate icon and color for file type
    getFileIcon(extension) {
        const iconMap = {
            pdf: { icon: 'fas fa-file-pdf', color: 'text-red-400', bgColor: 'bg-red-500/20' },
            doc: { icon: 'fas fa-file-word', color: 'text-blue-400', bgColor: 'bg-blue-500/20' },
            docx: { icon: 'fas fa-file-word', color: 'text-blue-400', bgColor: 'bg-blue-500/20' },
            txt: { icon: 'fas fa-file-alt', color: 'text-gray-400', bgColor: 'bg-gray-500/20' },
            rtf: { icon: 'fas fa-file-alt', color: 'text-purple-400', bgColor: 'bg-purple-500/20' },
            zip: { icon: 'fas fa-file-archive', color: 'text-purple-400', bgColor: 'bg-purple-500/20' },
            rar: { icon: 'fas fa-file-archive', color: 'text-purple-400', bgColor: 'bg-purple-500/20' },
            jpg: { icon: 'fas fa-file-image', color: 'text-green-400', bgColor: 'bg-green-500/20' },
            jpeg: { icon: 'fas fa-file-image', color: 'text-green-400', bgColor: 'bg-green-500/20' },
            png: { icon: 'fas fa-file-image', color: 'text-green-400', bgColor: 'bg-green-500/20' },
            gif: { icon: 'fas fa-file-image', color: 'text-green-400', bgColor: 'bg-green-500/20' }
        };

        return iconMap[extension] || { icon: 'fas fa-file', color: 'text-gray-400', bgColor: 'bg-gray-500/20' };
    }

    // Update document count display
    updateDocumentCount(count) {
        const docCountElement = document.getElementById('doc-count');
        if (docCountElement) {
            docCountElement.textContent = `${count} doc${count !== 1 ? 's' : ''}`;
        }
        
        // Also update the documentsAnalyzed property for analytics
        this.documentsAnalyzed = count;
        this.updateAnalytics();
    }

    // Remove a specific file
    async removeFile(fileId) {
        try {
            if (window.__TAURI__ && window.__TAURI__.invoke) {
                // Get current files
                const currentFiles = await window.__TAURI__.invoke('list_files');
                
                // Filter out the file to be removed
                const remainingFiles = currentFiles
                    .filter(f => f.id !== fileId)
                    .map(f => ({ id: f.id, path: f.original_path }));
                
                // Update with remaining files
                const result = await window.__TAURI__.invoke('update_files', remainingFiles);
                
                if (result.success) {
                    this.showNotification("File removed successfully", "success");
                    await this.loadFiles(); // Refresh the list
                } else {
                    throw new Error('Remove file failed');
                }
            } else {
                // Fallback for development
                console.log('Simulating file removal for ID:', fileId);
                this.showNotification("File removed successfully", "success");
                await this.loadFiles();
            }
        } catch (error) {
            console.error("Failed to remove file:", error);
            this.showNotification(`Failed to remove file: ${error.message}`, "error");
        }
    }
}

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    window.lawGorithm = new LawGorithm();
});
