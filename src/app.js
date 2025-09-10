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
                                ${metadata.insights ? `
                                    <div class="flex flex-wrap gap-2 mb-3">
                                        ${metadata.insights.map(insight => 
                                            `<span class="px-2 py-1 bg-blue-500/20 text-blue-400 rounded text-xs">${insight}</span>`
                                        ).join('')}
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
        const handleSend = () => {
            const text = input.value.trim();
            if (!text) return;
            
            addMessage(text, "user");
            input.value = "";
            this.queriesCount++;
            this.updateAnalytics();

            // Professional AI responses
            const responses = [
                {
                    text: "Based on my analysis of your uploaded documents, I've identified several key legal considerations. The contract terms show standard liability clauses, but Section 12 regarding indemnification needs review.",
                    insights: ["Contract Review", "Risk Assessment", "Liability Analysis"]
                },
                {
                    text: "I've found relevant case precedents that align with your legal query. The motion follows established requirements, though jurisdictional arguments could benefit from additional citations.",
                    insights: ["Precedent Research", "Motion Analysis", "Jurisdiction"]
                },
                {
                    text: "Your deposition transcript reveals strategic opportunities for cross-examination. I've identified 3 inconsistencies and 2 areas for impeachment evidence.",
                    insights: ["Deposition Analysis", "Strategy Planning", "Evidence Review"]
                }
            ];

            setTimeout(() => {
                const response = responses[Math.floor(Math.random() * responses.length)];
                addMessage(response.text, "bot", { insights: response.insights });
            }, Math.random() * 2000 + 1000);
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
                
                fileInput.onchange = (e) => {
                    const files = Array.from(e.target.files);
                    if (files.length > 0) {
                        this.handleFileUpload(files);
                    }
                };
                
                fileInput.click();
            });
        }
    }

    handleFileUpload(files) {
        // Simulate file upload with professional feedback
        files.forEach((file, index) => {
            setTimeout(() => {
                this.showNotification(`${file.name} uploaded successfully`, 'success');
                this.documentsAnalyzed++;
                this.updateAnalytics();
            }, index * 500);
        });
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
}

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    window.lawGorithm = new LawGorithm();
});
