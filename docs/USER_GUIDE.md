# Law-gorithm User Guide - For Everyone

## What is Law-gorithm?

Law-gorithm is like having a **smart legal assistant on your computer** that helps you understand legal documents in simple language. It **never shares your personal information** with anyone - everything stays private on your device.

## Who Should Use This?

✅ **Small business owners** understanding contracts  
✅ **Renters** reviewing lease agreements  
✅ **Employees** analyzing job contracts  
✅ **Students** researching legal cases  
✅ **Anyone** who needs to understand legal documents  

❌ **NOT for**: Professional legal advice (always consult a lawyer for important decisions)

## How Does Privacy Work?

Think of Law-gorithm like a **smart translator** that:
1. **Hides your real name** (replaces "John Smith" with "PARTY_1")
2. **Hides company names** (replaces "Acme Corp" with "COMPANY_A")
3. **Hides dates** (replaces "January 15, 2024" with "CONTRACT_DATE")
4. **Sends only the hidden version** to Google AI for analysis
5. **Translates the answer back** to show real names in the final result

**Your real information NEVER leaves your device!**

## Getting Started (Super Simple)

### Step 1: Download Law-gorithm
- **Download** the installer for your operating system from our releases page
- **Windows**: `.msi` installer
- **macOS**: `.dmg` installer  
- **Linux**: `.AppImage` or `.deb` package

### Step 2: Get Your Free Google AI Key
1. Go to https://makersuite.google.com/app/apikey
2. Sign in with your Google account
3. Click "Create API Key"
4. Copy the long code (looks like: `AIza...`)

### Step 3: Configure and Start
1. **Open Law-gorithm** after installation
2. **Enter your Google AI API key** in the settings
3. **Start using the application!**

## How to Use the Interface

### Uploading Documents
1. **Click "Upload Document"** button in the application
2. **Select your PDF file** from your computer
3. **Or drag and drop** the file onto the upload area
4. Wait for the processing confirmation ✅

### Asking Questions
Type questions like:
- "What are my main responsibilities in this contract?"
- "When does this agreement end?"
- "What happens if I break this contract?"
- "How much will I have to pay?"
- "Can I cancel this agreement?"

### Understanding Answers
- Answers are in **plain English**, not legal language
- **Important parts** are highlighted
- **Page references** show you where to find details
- **Warning signs** 🚨 highlight potential problems

## Example Use Cases

### Rental Lease Agreement
**Question**: "What are the penalties if I move out early?"
**Law-gorithm Answer**: "According to your lease with ABC Properties, if you terminate the agreement before the end date (June 30, 2025), you must pay a penalty equal to 2 months' rent ($2,400) plus forfeit your security deposit ($1,200). However, there's an exception if you provide 60 days written notice and pay a reduced fee of $800."

### Employment Contract
**Question**: "Can my employer fire me without cause?"
**Law-gorithm Answer**: "Your employment contract with XYZ Company includes an 'at-will' clause (Section 4.2), meaning either you or the company can end employment at any time without giving a reason. However, you're protected against discrimination and you'll receive 2 weeks severance pay if terminated without cause after your 6-month probation period."

### Business Service Agreement
**Question**: "What happens if the service provider doesn't deliver on time?"
**Law-gorithm Answer**: "According to the Service Level Agreement, if TechCorp fails to deliver services by the agreed deadline, you can: (1) Request a 10% discount on that month's fees, (2) Terminate the contract with 30 days notice if delays exceed 15 days, (3) Switch to their competitor without penalty fees (Clause 7.3)."

## Privacy Assurance - What Actually Happens

### When You Upload a Contract:
1. **Law-gorithm sees**: "John Smith agrees to pay Acme Corp $5,000 on January 15, 2024"
2. **Law-gorithm processes**: "PARTY_1 agrees to pay COMPANY_A AMOUNT_1 on CONTRACT_DATE"
3. **Google AI receives**: "PARTY_1 agrees to pay COMPANY_A AMOUNT_1 on CONTRACT_DATE"
4. **You see the answer**: "John Smith agrees to pay Acme Corp $5,000 on January 15, 2024"

**Google AI never sees your real name, company name, or exact amounts!**

### What Gets Stored on Your Device:
- Your original documents (encrypted)
- The anonymized versions
- The translation dictionary (PARTY_1 = John Smith)
- Your questions and answers

### What Never Leaves Your Device:
- Your real name, address, phone number
- Company names and contact details
- Exact dollar amounts and dates
- Any personally identifying information

## Troubleshooting

### "Container won't start"
- Make sure Docker is running
- Check if port 8080 is available: `docker ps`
- Try a different port: `-p 8081:8080`

### "Can't connect to Google AI"
- Verify your API key is correct
- Check your internet connection
- Make sure you have API quota remaining

### "Upload fails"
- Try smaller file sizes (under 50MB)
- Ensure PDF is not password protected
- Check file format (PDF, PNG, JPG supported)

### "Slow responses"
- Close other browser tabs
- Restart the container: `docker restart lawgorithm`
- Use lightweight mode: add `-e RESOURCE_MODE=LIGHT`

## Cost Information

### Free Usage:
- Google AI gives you **free API calls per month**
- Typical user: 10-50 documents = **$0-5/month**
- Heavy user: 100+ documents = **$10-20/month**

### Compared to Alternatives:
- **Lawyer consultation**: $200-500/hour
- **LegalZoom**: $50-200/month
- **Law-gorithm**: $0-20/month

## Security & Compliance

### For Business Use:
- **GDPR Compliant**: No PII transmitted
- **HIPAA Compatible**: Medical records stay anonymized
- **SOX Compliant**: Financial data never exposed
- **Audit Trail**: Complete logs of what was processed

### For Personal Use:
- **Bank safe privacy**: Your documents are protected like bank vaults
- **No data selling**: We don't sell or share your information
- **Local storage**: Everything stays on your device
- **Delete anytime**: Remove all data with one command

## Getting Help

### Built-in Help:
- Help button in the top-right corner
- Tooltips explain every feature
- Example questions to get started

### Community Support:
- GitHub issues for bug reports
- Discord community for questions
- Video tutorials on YouTube

### Professional Support:
- Business support available
- Custom deployments for enterprises
- Training sessions for teams

## Legal Disclaimer

**Important**: Law-gorithm helps you **understand** legal documents but **does not provide legal advice**. For important legal decisions, always consult with a qualified attorney. Law-gorithm is a tool for comprehension, not legal counsel.

---

## Quick Reference Commands

### Start Law-gorithm:
```bash
docker run -d --name lawgorithm -p 8080:8080 -e GOOGLE_AI_API_KEY=YOUR_KEY lawgorithm:latest
```

### Stop Law-gorithm:
```bash
docker stop lawgorithm
```

### Check if running:
```bash
docker ps
```

### View logs:
```bash
docker logs lawgorithm
```

### Clean up completely:
```bash
docker stop lawgorithm && docker rm lawgorithm
```

**Need help?** Visit: http://localhost:8080/help
