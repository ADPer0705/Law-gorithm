# Python RAG operations module
import chromadb
from sentence_transformers import SentenceTransformer

class RagProcessor:
    def __init__(self):
        self.model = SentenceTransformer('all-MiniLM-L6-v2')
        self.client = chromadb.Client()
        self.collection = self.client.create_collection("legal_docs")

    def add_chunks(self, chunks):
        embeddings = self.model.encode(chunks)
        self.collection.add(
            embeddings=embeddings.tolist(),
            documents=chunks,
            ids=[f"chunk_{i}" for i in range(len(chunks))]
        )

    def search(self, query, n_results=5):
        query_embedding = self.model.encode([query])
        results = self.collection.query(
            query_embeddings=query_embedding.tolist(),
            n_results=n_results
        )
        return results['documents'][0]
