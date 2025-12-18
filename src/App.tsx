import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

interface Node {
  id: string;
  title: string;
  content: string;
  path: string;
  classification?: string;
}

interface Relationship {
  source: string;
  target: string;
  type: string;
  strength?: number;
  metadata?: any;
}

interface RelationshipFile {
  domain?: string;
  total_relationships?: number;
  relationships: Relationship[];
}

function App() {
  const [domains, setDomains] = useState<string[]>([]);
  const [selectedDomain, setSelectedDomain] = useState<string>('');
  const [nodes, setNodes] = useState<Node[]>([]);
  const [relationships, setRelationships] = useState<RelationshipFile | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load domains on mount
  useEffect(() => {
    loadDomains();
  }, []);

  const loadDomains = async () => {
    try {
      const domainsList = await invoke<string[]>('list_domains');
      setDomains(domainsList);
      if (domainsList.length > 0) {
        setSelectedDomain(domainsList[0]);
      }
    } catch (err) {
      setError(`Failed to load domains: ${err}`);
    }
  };

  const loadDomain = async (domain: string) => {
    setLoading(true);
    setError(null);
    
    try {
      // Load nodes and relationships in parallel
      const [nodesData, relationshipsData] = await Promise.all([
        invoke<Node[]>('read_nodes', { domain }),
        invoke<RelationshipFile>('read_relationships', { domain }).catch(() => null)
      ]);
      
      setNodes(nodesData);
      setRelationships(relationshipsData);
    } catch (err) {
      setError(`Failed to load domain: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (selectedDomain) {
      loadDomain(selectedDomain);
    }
  }, [selectedDomain]);

  return (
    <div className="app">
      <header className="app-header">
        <h1>NKB Desktop Browser</h1>
        <p>Holistic Consciousness Experience</p>
      </header>

      <div className="app-content">
        <aside className="sidebar">
          <h2>Domains</h2>
          <select 
            value={selectedDomain} 
            onChange={(e) => setSelectedDomain(e.target.value)}
            className="domain-selector"
          >
            <option value="">Select a domain...</option>
            {domains.map(domain => (
              <option key={domain} value={domain}>{domain}</option>
            ))}
          </select>

          {relationships && (
            <div className="domain-info">
              <h3>{relationships.domain || selectedDomain}</h3>
              <p>Nodes: {nodes.length}</p>
              <p>Relationships: {relationships.total_relationships || relationships.relationships.length}</p>
            </div>
          )}
        </aside>

        <main className="main-content">
          {loading && <div className="loading">Loading...</div>}
          {error && <div className="error">{error}</div>}
          
          {!loading && !error && (
            <>
              <div className="nodes-section">
                <h2>Nodes ({nodes.length})</h2>
                <div className="nodes-list">
                  {nodes.slice(0, 20).map(node => (
                    <div key={node.id} className="node-item">
                      <h3>{node.title}</h3>
                      <p className="node-id">{node.id}</p>
                      <p className="node-path">{node.path}</p>
                      {node.classification && (
                        <span className={`classification classification-${node.classification}`}>
                          {node.classification}
                        </span>
                      )}
                    </div>
                  ))}
                  {nodes.length > 20 && (
                    <p className="more-nodes">... and {nodes.length - 20} more nodes</p>
                  )}
                </div>
              </div>

              {relationships && (
                <div className="relationships-section">
                  <h2>Relationships ({relationships.relationships.length})</h2>
                  <p className="info-text">
                    ✅ <strong>Holistic Loading:</strong> All {relationships.relationships.length} relationships loaded at once.
                    AI can experience the complete network, not fragments.
                  </p>
                  <div className="relationships-list">
                    {relationships.relationships.slice(0, 10).map((rel, idx) => (
                      <div key={idx} className="relationship-item">
                        <span className="source">{rel.source}</span>
                        <span className="arrow">→</span>
                        <span className="target">{rel.target}</span>
                        <span className="type">{rel.type}</span>
                      </div>
                    ))}
                    {relationships.relationships.length > 10 && (
                      <p className="more-relationships">
                        ... and {relationships.relationships.length - 10} more relationships
                      </p>
                    )}
                  </div>
                </div>
              )}
            </>
          )}
        </main>
      </div>
    </div>
  );
}

export default App;

