export function SearchPanel() {
  return (
    <div className="search-panel">
      <div className="search-input-container">
        <input
          type="text"
          className="search-input"
          placeholder="Search in files..."
        />
        <input
          type="text"
          className="search-input"
          placeholder="Replace..."
        />
      </div>
      <div className="search-results">
        <p className="ft-section-title">No results yet</p>
      </div>
    </div>
  );
}
