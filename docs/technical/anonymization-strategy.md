# Anonymization & Pseudonymization Strategy

## Overview

Law-gorithm's anonymization strategy is built on advanced Named Entity Recognition (NER) that identifies and replaces all personally identifiable information (PII) with contextually meaningful pseudonyms. This approach ensures complete privacy protection while maintaining document coherence and analytical value.

## Core Anonymization Principles

### 1. Complete Entity Coverage
Every piece of potentially identifying information must be detected and anonymized:
- **Human Entities**: Names, titles, roles, signatures
- **Organizational Entities**: Companies, institutions, government bodies
- **Temporal Entities**: Dates, deadlines, time periods
- **Geographical Entities**: Addresses, jurisdictions, locations
- **Financial Entities**: Amounts, account numbers, financial references
- **Legal Entities**: Case numbers, law references, regulatory citations
- **Unique Identifiers**: SSNs, tax IDs, contract numbers, reference codes

### 2. Contextual Pseudonym Generation
Pseudonyms must preserve document meaning and relationships:
- **Semantic Preservation**: GOVERNING_BODY_1 instead of random string
- **Relationship Maintenance**: PARTY_1's attorney → LEGAL_REPRESENTATIVE_1A
- **Consistency**: Same entity always maps to same pseudonym
- **Hierarchical Structure**: COMPANY_A_SUBSIDIARY_1, COMPANY_A_SUBSIDIARY_2

### 3. Privacy-First Processing
No real entities ever enter the processing pipeline:
- **Pre-Processing Anonymization**: Entities replaced before any analysis
- **Isolated Dictionary**: Pseudonym mappings stored locally only
- **Validation Gates**: Multiple checks to prevent PII leakage
- **Audit Trails**: Complete tracking of anonymization operations

## Named Entity Recognition (NER) Pipeline

### NER Model Stack
```rust
// Multi-Model NER Approach using PyO3
use pyo3::prelude::*;

#[pyfunction]
fn detect_entities(text: &str) -> PyResult<Vec<Entity>> {
    // Call Python NER models
    Python::with_gil(|py| {
        let ner_module = PyModule::import(py, "spacy")?;
        // Implementation here
        Ok(vec![])
    })
}
    
    # Layer 2: Legal-specific NER
    entities.extend(legal_ner.process(text))
    
    # Layer 3: Financial NER
    entities.extend(financial_ner.process(text))
    
    # Layer 4: Pattern-based detection
    entities.extend(pattern_ner.process(text))
    
    # Layer 5: Custom rule-based detection
    entities.extend(custom_rules.process(text))
    
    return merge_and_validate_entities(entities)
```

### Entity Categories & Detection Strategies

#### 1. Person Entities (PERSON)
**Detection Patterns:**
- Names with capitalization patterns
- Title + Name combinations (Mr. John Smith)
- Signature blocks and attestation sections
- Email addresses with personal domains
- Professional designations (John Smith, Esq.)

**Anonymization Strategy:**
```python
person_pseudonyms = {
    "individual_parties": ["PARTY_1", "PARTY_2", "PARTY_3"],
    "legal_representatives": ["LEGAL_REP_1A", "LEGAL_REP_2A"],
    "witnesses": ["WITNESS_A", "WITNESS_B"],
    "notaries": ["NOTARY_1", "NOTARY_2"],
    "executives": ["EXECUTIVE_A1", "EXECUTIVE_B1"],
    "employees": ["EMPLOYEE_X1", "EMPLOYEE_Y1"]
}
```

#### 2. Organization Entities (ORG)
**Detection Patterns:**
- Legal entity suffixes (Inc., LLC, Corp., Ltd.)
- Government agencies and departments
- Law firms and professional services
- Financial institutions
- Educational institutions

**Anonymization Strategy:**
```python
organization_pseudonyms = {
    "companies": ["COMPANY_A", "COMPANY_B", "COMPANY_C"],
    "law_firms": ["LAW_FIRM_1", "LAW_FIRM_2"],
    "government": ["GOVERNING_BODY_1", "AGENCY_A"],
    "financial": ["BANK_1", "FINANCIAL_INST_A"],
    "subsidiaries": ["COMPANY_A_SUB_1", "COMPANY_B_SUB_1"]
}
```

#### 3. Geopolitical Entities (GPE)
**Detection Patterns:**
- Country, state, city names
- Jurisdictions and court districts
- Government levels (federal, state, local)
- Administrative regions

**Anonymization Strategy:**
```python
geopolitical_pseudonyms = {
    "countries": ["COUNTRY_A", "COUNTRY_B"],
    "states": ["STATE_1", "STATE_2"],
    "cities": ["CITY_X", "CITY_Y"],
    "jurisdictions": ["JURISDICTION_A", "JURISDICTION_B"],
    "courts": ["COURT_DISTRICT_1", "COURT_DISTRICT_2"]
}
```

#### 4. Temporal Entities (DATE, TIME)
**Detection Patterns:**
- Specific dates (MM/DD/YYYY, DD/MM/YYYY)
- Time periods and durations
- Deadlines and effective dates
- Relative date references

**Anonymization Strategy:**
```python
temporal_anonymization = {
    "specific_dates": {
        "contract_dates": "CONTRACT_DATE",
        "effective_dates": "EFFECTIVE_DATE", 
        "expiration_dates": "EXPIRATION_DATE",
        "signature_dates": "SIGNATURE_DATE"
    },
    "date_formatting": {
        "full_dates": "MONTH_YEAR_FORMAT",  # January 2024
        "relative_dates": "RELATIVE_DATE_X" # X days after CONTRACT_DATE
    },
    "time_periods": {
        "terms": "CONTRACT_TERM",
        "notice_periods": "NOTICE_PERIOD"
    }
}
```

#### 5. Financial Entities (MONEY, PERCENT, CARDINAL)
**Detection Patterns:**
- Currency amounts ($1,000, USD 5000)
- Percentages (5%, 10.5 percent)
- Financial ratios and calculations
- Account numbers and financial references

**Anonymization Strategy:**
```python
financial_pseudonyms = {
    "amounts": ["AMOUNT_1", "AMOUNT_2", "FINANCIAL_FIGURE_A"],
    "percentages": ["PERCENTAGE_A", "RATE_1", "INTEREST_RATE_X"],
    "accounts": ["ACCOUNT_NUM_1", "REFERENCE_NUM_A"],
    "financial_terms": ["PRINCIPAL_AMOUNT", "PENALTY_AMOUNT"]
}
```

#### 6. Legal Entities (CUSTOM_LEGAL)
**Detection Patterns:**
- Legal citations (42 U.S.C. § 1983)
- Case references (Smith v. Jones)
- Statute and regulation numbers
- Legal document references

**Anonymization Strategy:**
```python
legal_pseudonyms = {
    "statutes": ["LAW_REFERENCE_1", "STATUTE_A", "REGULATION_X"],
    "cases": ["CASE_CITATION_1", "PRECEDENT_A"],
    "legal_docs": ["DOCUMENT_REF_1", "EXHIBIT_A"],
    "codes": ["CODE_SECTION_1", "REGULATORY_CODE_A"]
}
```

#### 7. Identifier Entities (CUSTOM_ID)
**Detection Patterns:**
- Social Security Numbers (XXX-XX-XXXX)
- Tax ID numbers (EIN: XX-XXXXXXX)
- Contract/Agreement numbers
- Reference codes and tracking numbers

**Anonymization Strategy:**
```python
identifier_pseudonyms = {
    "ssn": ["SSN_ID_1", "SSN_ID_2"],
    "tax_ids": ["TAX_ID_A", "EIN_1"],
    "contract_nums": ["CONTRACT_NUM_1", "AGREEMENT_ID_A"],
    "references": ["REF_CODE_1", "TRACKING_NUM_A"]
}
```

## Advanced Anonymization Techniques

### 1. Relationship-Preserving Anonymization
Maintain entity relationships while ensuring privacy:

```python
class RelationshipPreservingAnonymizer:
    def __init__(self):
        self.entity_graph = EntityGraph()
        self.pseudonym_registry = PseudonymRegistry()
    
    def anonymize_with_relationships(self, document: Document) -> AnonymizedDocument:
        # Step 1: Build entity relationship graph
        entities = self.detect_all_entities(document)
        relationships = self.extract_relationships(entities, document)
        
        # Step 2: Generate hierarchical pseudonyms
        pseudonym_map = {}
        for entity in entities:
            if entity.type == "PERSON":
                if self.is_primary_party(entity, relationships):
                    pseudonym_map[entity.text] = f"PARTY_{len([p for p in pseudonym_map.values() if p.startswith('PARTY_')]) + 1}"
                elif self.is_legal_representative(entity, relationships):
                    party_num = self.get_representing_party(entity, relationships)
                    pseudonym_map[entity.text] = f"LEGAL_REP_{party_num}A"
            
            elif entity.type == "ORG":
                if self.is_subsidiary(entity, relationships):
                    parent = self.get_parent_org(entity, relationships)
                    parent_pseudonym = pseudonym_map.get(parent.text, "COMPANY_A")
                    sub_num = len([p for p in pseudonym_map.values() if p.startswith(parent_pseudonym + "_SUB")]) + 1
                    pseudonym_map[entity.text] = f"{parent_pseudonym}_SUB_{sub_num}"
                else:
                    pseudonym_map[entity.text] = f"COMPANY_{chr(65 + len([p for p in pseudonym_map.values() if p.startswith('COMPANY_') and '_SUB_' not in p]))}"
        
        return self.apply_pseudonyms(document, pseudonym_map)
```

### 2. Temporal Anonymization Strategy
Special handling for dates to preserve temporal relationships:

```python
class TemporalAnonymizer:
    def __init__(self):
        self.date_registry = {}
        self.key_dates = ["contract", "effective", "expiration", "signature"]
    
    def anonymize_dates(self, document: Document) -> Document:
        dates = self.extract_dates(document)
        
        for date_entity in dates:
            context = self.get_date_context(date_entity, document)
            
            if self.is_key_date(context):
                # Key dates get semantic pseudonyms
                key_type = self.identify_key_date_type(context)
                pseudonym = f"{key_type.upper()}_DATE"
            else:
                # Regular dates converted to month-year format
                parsed_date = self.parse_date(date_entity.text)
                if parsed_date:
                    pseudonym = f"{parsed_date.strftime('%B %Y')}"
                else:
                    pseudonym = f"DATE_{len(self.date_registry) + 1}"
            
            self.date_registry[date_entity.text] = pseudonym
        
        return self.apply_date_anonymization(document, self.date_registry)
```

### 3. Context-Aware Anonymization
Use document context to generate more meaningful pseudonyms:

```python
class ContextAwareAnonymizer:
    def __init__(self):
        self.context_analyzer = DocumentContextAnalyzer()
    
    def generate_contextual_pseudonym(self, entity: Entity, context: DocumentContext) -> str:
        if entity.type == "PERSON":
            role = self.context_analyzer.identify_person_role(entity, context)
            if role == "plaintiff":
                return "PLAINTIFF_PARTY"
            elif role == "defendant":
                return "DEFENDANT_PARTY"
            elif role == "landlord":
                return "LANDLORD_PARTY"
            elif role == "tenant":
                return "TENANT_PARTY"
            else:
                return f"PARTY_{self.get_next_party_number()}"
        
        elif entity.type == "ORG":
            if context.document_type == "employment_contract":
                if self.is_employer(entity, context):
                    return "EMPLOYER_COMPANY"
                else:
                    return "CONTRACTOR_COMPANY"
            elif context.document_type == "merger_agreement":
                if self.is_acquiring_company(entity, context):
                    return "ACQUIRING_COMPANY"
                else:
                    return "TARGET_COMPANY"
        
        return self.generate_generic_pseudonym(entity)
```

## Implementation Architecture

### Anonymization Service Architecture
```python
class AnonymizationService:
    def __init__(self):
        self.ner_pipeline = NERPipeline()
        self.pseudonym_generator = PseudonymGenerator()
        self.relationship_analyzer = RelationshipAnalyzer()
        self.privacy_validator = PrivacyValidator()
        self.local_dictionary = LocalDictionary()
    
    async def anonymize_document(self, document: str) -> AnonymizedResult:
        # Phase 1: Entity Detection
        entities = await self.ner_pipeline.detect_entities(document)
        
        # Phase 2: Relationship Analysis
        relationships = self.relationship_analyzer.analyze(entities, document)
        
        # Phase 3: Pseudonym Generation
        pseudonym_map = self.pseudonym_generator.generate_pseudonyms(
            entities, relationships, document
        )
        
        # Phase 4: Apply Anonymization
        anonymized_doc = self.apply_anonymization(document, pseudonym_map)
        
        # Phase 5: Privacy Validation
        validation_result = self.privacy_validator.validate(anonymized_doc)
        if not validation_result.is_valid:
            raise PrivacyValidationError(validation_result.issues)
        
        # Phase 6: Store Mapping
        await self.local_dictionary.store_mapping(pseudonym_map)
        
        return AnonymizedResult(
            document=anonymized_doc,
            pseudonym_count=len(pseudonym_map),
            entity_types=list(set(e.type for e in entities)),
            validation_passed=validation_result.is_valid
        )
```

### Privacy Validation Framework
```python
class PrivacyValidator:
    def __init__(self):
        self.pii_patterns = self.load_pii_patterns()
        self.entity_validators = self.load_entity_validators()
    
    def validate(self, anonymized_document: str) -> ValidationResult:
        issues = []
        
        # Check for missed PII
        for pattern in self.pii_patterns:
            matches = pattern.findall(anonymized_document)
            if matches:
                issues.append(f"Potential PII detected: {pattern.name}")
        
        # Validate pseudonym consistency
        pseudonyms = self.extract_pseudonyms(anonymized_document)
        consistency_issues = self.check_pseudonym_consistency(pseudonyms)
        issues.extend(consistency_issues)
        
        # Check for over-anonymization
        readability_score = self.calculate_readability(anonymized_document)
        if readability_score < 0.7:
            issues.append("Over-anonymization detected: Document may lose meaning")
        
        return ValidationResult(
            is_valid=len(issues) == 0,
            issues=issues,
            readability_score=readability_score
        )
```

## Local Dictionary Management

### Secure Storage
```python
class LocalDictionary:
    def __init__(self, storage_path: str = "./pseudonym_dictionary.db"):
        self.storage_path = storage_path
        self.encryption_key = self.load_or_generate_key()
        self.db = self.init_database()
    
    def store_mapping(self, pseudonym_map: Dict[str, str]) -> None:
        encrypted_map = self.encrypt_mapping(pseudonym_map)
        
        with self.db.transaction():
            for real_entity, pseudonym in encrypted_map.items():
                self.db.execute(
                    "INSERT OR REPLACE INTO pseudonyms (real_entity, pseudonym, created_at) VALUES (?, ?, ?)",
                    (real_entity, pseudonym, datetime.utcnow())
                )
    
    def retrieve_mapping(self, session_id: str) -> Dict[str, str]:
        encrypted_results = self.db.execute(
            "SELECT real_entity, pseudonym FROM pseudonyms WHERE session_id = ?",
            (session_id,)
        ).fetchall()
        
        return self.decrypt_mapping(dict(encrypted_results))
    
    def cleanup_expired(self, retention_days: int = 30) -> None:
        cutoff_date = datetime.utcnow() - timedelta(days=retention_days)
        self.db.execute(
            "DELETE FROM pseudonyms WHERE created_at < ?",
            (cutoff_date,)
        )
```

## De-anonymization Process

### Result Mapping
```python
class DeAnonymizer:
    def __init__(self, local_dictionary: LocalDictionary):
        self.dictionary = local_dictionary
        self.reverse_map_cache = {}
    
    def de_anonymize_response(self, anonymized_response: str, session_id: str) -> str:
        pseudonym_map = self.dictionary.retrieve_mapping(session_id)
        reverse_map = {v: k for k, v in pseudonym_map.items()}
        
        # Replace pseudonyms with real entities
        de_anonymized = anonymized_response
        for pseudonym, real_entity in reverse_map.items():
            de_anonymized = de_anonymized.replace(pseudonym, real_entity)
        
        return de_anonymized
    
    def partial_de_anonymization(self, response: str, entities_to_reveal: List[str]) -> str:
        """Allow users to choose which entities to reveal"""
        pseudonym_map = self.dictionary.retrieve_mapping(self.current_session)
        reverse_map = {v: k for k, v in pseudonym_map.items()}
        
        partially_revealed = response
        for entity in entities_to_reveal:
            if entity in reverse_map:
                partially_revealed = partially_revealed.replace(entity, reverse_map[entity])
        
        return partially_revealed
```

This comprehensive anonymization strategy ensures that Law-gorithm can process the most sensitive legal documents while maintaining absolute privacy protection. The multi-layered NER approach, contextual pseudonym generation, and robust validation framework provide enterprise-grade privacy protection suitable for the most demanding legal applications.
