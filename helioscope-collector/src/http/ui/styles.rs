// src/http/ui/styles.rs

/// Base CSS variables and reset (included on every page)
pub const BASE: &str = r#"
:root {
    --bg-gradient-start: #ff6b35;
    --bg-gradient-end: #f7931e;
    --card-bg: white;
    --text-primary: #2d3748;
    --text-secondary: #718096;
    --border-color: #e2e8f0;
    --accent-color: #ff6b35;
    --accent-hover: #e85a2a;
    --secondary-bg: #f7fafc;
    --secondary-hover: #edf2f7;
}

@media (prefers-color-scheme: dark) {
    :root {
        --bg-gradient-start: #1a202c;
        --bg-gradient-end: #2d3748;
        --card-bg: #2d3748;
        --text-primary: #f7fafc;
        --text-secondary: #cbd5e0;
        --border-color: #4a5568;
        --accent-color: #ff6b35;
        --accent-hover: #ff8555;
        --secondary-bg: #1a202c;
        --secondary-hover: #374151;
    }
}

* { margin: 0; padding: 0; box-sizing: border-box; }

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    line-height: 1.6;
    color: var(--text-primary);
    background: linear-gradient(135deg, var(--bg-gradient-start) 0%, var(--bg-gradient-end) 100%);
    min-height: 100vh;
    padding: 40px 20px;
}

.container { max-width: 1200px; margin: 0 auto; }

footer {
    margin-top: 40px;
    text-align: center;
    color: rgba(255, 255, 255, 0.9);
}
"#;

/// Button styles
pub const BUTTONS: &str = r#"
.btn {
    padding: 10px 16px;
    border-radius: 6px;
    text-decoration: none;
    font-weight: 500;
    font-size: 0.9em;
    transition: all 0.2s;
    display: inline-block;
}
.btn-primary { background: var(--accent-color); color: white; }
.btn-primary:hover { background: var(--accent-hover); }
.btn-secondary {
    background: var(--secondary-bg);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
}
.btn-secondary:hover { background: var(--secondary-hover); }
"#;

/// Header styles
pub const HEADER: &str = r#"
header {
    background: var(--card-bg);
    border-radius: 8px;
    padding: 15px 20px;
    margin-bottom: 20px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
.header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.header-left {
    display: flex;
    align-items: baseline;
    gap: 12px;
}
.header-right {
    display: flex;
    gap: 10px;
    align-items: center;
}
header h1 {
    color: var(--accent-color);
    margin: 0;
    font-size: 1.5em;
    font-weight: 600;
}
.subtitle {
    color: var(--text-secondary);
    font-size: 0.9em;
    margin: 0;
}
"#;

/// Stats cards (used on home page)
pub const STATS: &str = r#"
.stats { display: flex; gap: 10px; }
.stat-card {
    background: var(--secondary-bg);
    padding: 8px 16px;
    border-radius: 6px;
    border-left: 3px solid var(--accent-color);
    display: flex;
    align-items: center;
    gap: 8px;
}
.stat-value {
    font-size: 1.3em;
    font-weight: 700;
    color: var(--accent-color);
}
.stat-label {
    color: var(--text-secondary);
    font-size: 0.85em;
    white-space: nowrap;
}
"#;

/// Node card styles (used on home page)
pub const NODE_CARDS: &str = r#"
.nodes-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
    gap: 20px;
}
.node-card {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 25px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s, box-shadow 0.2s;
}
.node-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
}
.node-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding-bottom: 15px;
    border-bottom: 1px solid var(--border-color);
}
.node-header h3 { color: var(--text-primary); font-size: 1.3em; margin-bottom: 5px; }
.node-header .subtitle {
    margin: 0;
    padding: 0;
    font-size: 0.9em;
    color: var(--text-secondary);
    font-weight: normal;
}
.status-badge {
    padding: 5px 12px;
    border-radius: 20px;
    font-size: 0.85em;
    font-weight: 600;
    background: var(--accent-color);
    color: white;
}
.node-info { margin-bottom: 20px; }
.node-actions { display: flex; gap: 10px; flex-wrap: wrap; }
.no-data {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 40px;
    text-align: center;
    color: var(--text-secondary);
}
"#;

/// Info row styles (label: value pairs)
pub const INFO_ROWS: &str = r#"
.info-row {
    display: flex;
    justify-content: space-between;
    padding: 8px 0;
    border-bottom: 1px solid var(--border-color);
}
.info-label { color: var(--text-secondary); font-weight: 500; }
.info-value { color: var(--text-primary); font-weight: 600; }
"#;

/// Node dashboard specific styles
pub const NODE_DASHBOARD: &str = r#"
.breadcrumb {
    color: var(--text-secondary);
    font-size: 0.85em;
    margin-bottom: 12px;
}
.breadcrumb a { color: var(--accent-color); text-decoration: none; }
.breadcrumb a:hover { text-decoration: underline; }

.node-header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
}

.node-title-section h1 {
    margin: 0 0 4px 0;
    font-size: 1.4em;
    color: var(--text-primary);
}

.node-id-compact {
    font-family: "Courier New", monospace;
    font-size: 0.75em;
    color: var(--text-secondary);
    margin: 0;
}

.node-meta-compact {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
}

.meta-item {
    background: var(--secondary-bg);
    padding: 6px 12px;
    border-radius: 6px;
    border-left: 3px solid var(--accent-color);
    display: flex;
    align-items: center;
    gap: 6px;
}
.meta-label {
    color: var(--text-secondary);
    font-size: 0.75em;
    font-weight: 500;
}
.meta-value {
    color: var(--text-primary);
    font-weight: 600;
    font-size: 0.85em;
}

.charts-section { margin-top: 20px; }
.charts-section h2 {
    color: var(--text-primary);
    margin-bottom: 15px;
    font-size: 1.2em;
    font-weight: 600;
}
.charts-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 20px;
}

/* Two columns on larger screens (1600px+) */
@media (min-width: 1600px) {
    .charts-grid {
        grid-template-columns: repeat(2, 1fr);
    }
}

.chart-card {
    background: var(--card-bg);
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
.chart-card h3 {
    color: var(--text-primary);
    margin-bottom: 12px;
    font-size: 1em;
    font-weight: 600;
}
.chart-container {
    background: var(--secondary-bg);
    border-radius: 6px;
    padding: 8px;
    min-height: 400px;
}
.chart-container img { width: 100%; height: auto; display: block; }

.actions { margin-top: 30px; display: flex; gap: 10px; flex-wrap: wrap; }
"#;

/// Error page styles
pub const ERROR_PAGE: &str = r#"
.error-container {
    text-align: center;
    padding: 60px 40px;
    background: var(--card-bg);
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}
.error-container h1 {
    color: #dc3545;
    font-size: 2em;
    margin-bottom: 15px;
}
.error-container p {
    color: var(--text-secondary);
    margin-bottom: 25px;
}
"#;
