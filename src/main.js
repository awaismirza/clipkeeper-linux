// ClipKeeper Frontend Controller
const invoke = window.__TAURI__ ? window.__TAURI__.core.invoke : null;
const listen = window.__TAURI__ ? window.__TAURI__.event.listen : null;

// Application State
let historyItems = [];
let selectedIndex = 0;
let activeFilter = 'all';
let searchQuery = '';

// DOM Elements
const searchInput = document.getElementById('searchInput');
const clearSearchBtn = document.getElementById('clearSearchBtn');
const filterTabs = document.getElementById('filterTabs');
const historyList = document.getElementById('historyList');
const itemCountEl = document.getElementById('itemCount');
const clearAllBtn = document.getElementById('clearAllBtn');

// Initialize App
document.addEventListener('DOMContentLoaded', async () => {
  setupEventListeners();
  await loadHistory();
  setupTauriListeners();
  focusSearchInput();
});

window.addEventListener('focus', () => {
  focusSearchInput();
});

document.addEventListener('visibilitychange', () => {
  if (document.visibilityState === 'visible') {
    focusSearchInput();
  }
});

function focusSearchInput() {
  if (searchInput) {
    setTimeout(() => {
      searchInput.focus();
      searchInput.select();
    }, 50);
  }
}

// Setup Realtime Tauri Event Listeners
async function setupTauriListeners() {
  if (listen) {
    try {
      await listen('clipboard-updated', () => {
        loadHistory();
      });
      await listen('window-focused', () => {
        focusSearchInput();
      });
    } catch (err) {
      console.warn('Failed to register Tauri event listener:', err);
    }
  }
}

// Load Clipboard History from Backend IPC
async function loadHistory() {
  if (!invoke) {
    // Fallback mock data for browser testing
    historyItems = getMockHistory();
    renderList();
    return;
  }

  try {
    const items = await invoke('get_history', {
      search: searchQuery.trim() ? searchQuery.trim() : null,
      filterType: activeFilter !== 'all' ? activeFilter : null,
      limit: 200
    });

    historyItems = items || [];
    
    // Clamp selected index
    if (selectedIndex >= historyItems.length) {
      selectedIndex = Math.max(0, historyItems.length - 1);
    }
    
    renderList();
  } catch (err) {
    console.error('Failed to load history:', err);
    historyList.innerHTML = `<div class="empty-state">Error loading history: ${err}</div>`;
  }
}

// Render History List
function renderList() {
  itemCountEl.textContent = `${historyItems.length} item${historyItems.length === 1 ? '' : 's'}`;
  
  if (historyItems.length === 0) {
    historyList.innerHTML = `
      <div class="empty-state">
        <div class="empty-icon">📋</div>
        <div>No clipboard items found</div>
        <div style="font-size: 11px; opacity: 0.7;">Copied text or images will automatically appear here</div>
      </div>
    `;
    return;
  }

  historyList.innerHTML = '';

  historyItems.forEach((item, index) => {
    const isSelected = index === selectedIndex;
    const card = document.createElement('div');
    card.className = `item-card ${isSelected ? 'selected' : ''}`;
    card.dataset.id = item.id;
    card.dataset.index = index;

    const timeLabel = formatRelativeTime(item.timestamp);
    const badgeClass = `badge-${item.item_type || 'text'}`;

    let contentHtml = '';

    if (item.item_type === 'image') {
      const dimensions = (item.image_width && item.image_height) 
        ? `${item.image_width} × ${item.image_height} px` 
        : 'Image';

      contentHtml = `
        <div class="image-preview-container">
          <img src="${item.preview}" class="image-thumbnail" alt="Clipboard Image" loading="lazy" />
          <div class="image-meta">
            <span style="font-weight: 600; color: #f8fafc;">PNG Image</span>
            <span>${dimensions}</span>
          </div>
        </div>
      `;
    } else {
      const isCode = item.item_type === 'code';
      const escapeHtml = str => (str || '').replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
      contentHtml = `
        <div class="item-content-preview ${isCode ? 'code-font' : ''}">
          ${escapeHtml(item.preview)}
        </div>
      `;
    }

    card.innerHTML = `
      <div class="item-main">
        <div class="item-header">
          <span class="type-badge ${badgeClass}">${item.item_type}</span>
          ${item.pinned ? '<span class="pin-indicator" title="Pinned">📌</span>' : ''}
          <span class="item-time">${timeLabel}</span>
        </div>
        ${contentHtml}
      </div>

      <div class="item-actions">
        <button class="action-btn pin-btn ${item.pinned ? 'active' : ''}" title="${item.pinned ? 'Unpin' : 'Pin'}" data-action="pin">📌</button>
        <button class="action-btn delete-btn" title="Delete" data-action="delete">🗑️</button>
      </div>
    `;

    // Click selection and copy trigger
    card.addEventListener('click', (e) => {
      const action = e.target.closest('[data-action]')?.dataset.action;
      if (action === 'pin') {
        e.stopPropagation();
        togglePinItem(item.id);
      } else if (action === 'delete') {
        e.stopPropagation();
        deleteItem(item.id);
      } else {
        selectedIndex = index;
        updateSelectionHighlight();
        selectAndCopyItem(item.id);
      }
    });

    historyList.appendChild(card);
  });

  scrollSelectedIntoView();
}

// Update Highlight Class Without Re-rendering
function updateSelectionHighlight() {
  const cards = historyList.querySelectorAll('.item-card');
  cards.forEach((card, idx) => {
    if (idx === selectedIndex) {
      card.classList.add('selected');
    } else {
      card.classList.remove('selected');
    }
  });
  scrollSelectedIntoView();
}

function scrollSelectedIntoView() {
  const selectedCard = historyList.querySelector(`.item-card[data-index="${selectedIndex}"]`);
  if (selectedCard) {
    selectedCard.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
  }
}

// Select & Copy Item Back to System Clipboard
async function selectAndCopyItem(id) {
  if (!invoke) return;

  try {
    await invoke('copy_to_clipboard', { id });
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
  }
}

// Toggle Item Pin Status
async function togglePinItem(id) {
  if (!invoke) return;

  try {
    await invoke('toggle_pin', { id });
    await loadHistory();
  } catch (err) {
    console.error('Failed to toggle pin:', err);
  }
}

// Delete Item
async function deleteItem(id) {
  if (!invoke) return;

  try {
    await invoke('delete_item', { id });
    await loadHistory();
  } catch (err) {
    console.error('Failed to delete item:', err);
  }
}

// Setup Keyboard & Input Event Listeners
function setupEventListeners() {
  // Search Input Handler
  searchInput.addEventListener('input', (e) => {
    searchQuery = e.target.value;
    clearSearchBtn.style.display = searchQuery ? 'block' : 'none';
    selectedIndex = 0;
    loadHistory();
  });

  clearSearchBtn.addEventListener('click', () => {
    searchInput.value = '';
    searchQuery = '';
    clearSearchBtn.style.display = 'none';
    selectedIndex = 0;
    searchInput.focus();
    loadHistory();
  });

  // Filter Tabs Handler
  filterTabs.addEventListener('click', (e) => {
    const tab = e.target.closest('.filter-tab');
    if (!tab) return;

    filterTabs.querySelectorAll('.filter-tab').forEach(t => t.classList.remove('active'));
    tab.classList.add('active');

    activeFilter = tab.dataset.filter;
    selectedIndex = 0;
    loadHistory();
  });

  // Clear Unpinned Items
  clearAllBtn.addEventListener('click', async () => {
    if (confirm('Clear all unpinned clipboard items?')) {
      if (invoke) {
        await invoke('clear_history');
        await loadHistory();
      }
    }
  });

  // Keyboard Navigation Handler
  window.addEventListener('keydown', (e) => {
    if (historyItems.length === 0) {
      if (e.key === 'Escape') {
        if (invoke) invoke('hide_window');
      }
      return;
    }

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = (selectedIndex + 1) % historyItems.length;
      updateSelectionHighlight();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = (selectedIndex - 1 + historyItems.length) % historyItems.length;
      updateSelectionHighlight();
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const currentItem = historyItems[selectedIndex];
      if (currentItem) {
        selectAndCopyItem(currentItem.id);
      }
    } else if (e.key === 'Escape') {
      e.preventDefault();
      if (invoke) invoke('hide_window');
    } else if (e.key === 'Delete' || (e.ctrlKey && e.key.toLowerCase() === 'd')) {
      e.preventDefault();
      const currentItem = historyItems[selectedIndex];
      if (currentItem) {
        deleteItem(currentItem.id);
      }
    } else if (e.key.toLowerCase() === 'p' && !e.ctrlKey && !e.altKey && document.activeElement !== searchInput) {
      e.preventDefault();
      const currentItem = historyItems[selectedIndex];
      if (currentItem) {
        togglePinItem(currentItem.id);
      }
    }
  });
}

// Relative Time Helper
function formatRelativeTime(timestamp) {
  if (!timestamp) return 'Just now';
  const now = Date.now();
  const diffSec = Math.floor((now - timestamp) / 1000);

  if (diffSec < 30) return 'Just now';
  if (diffSec < 60) return `${diffSec}s ago`;

  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60) return `${diffMin}m ago`;

  const diffHours = Math.floor(diffMin / 60);
  if (diffHours < 24) return `${diffHours}h ago`;

  const diffDays = Math.floor(diffHours / 24);
  return `${diffDays}d ago`;
}

// Mock Data for Non-Tauri Browser Testing
function getMockHistory() {
  return [
    {
      id: 1,
      hash: 'h1',
      item_type: 'code',
      content: 'fn main() {\n  println!("Hello ClipKeeper!");\n}',
      preview: 'fn main() {\n  println!("Hello ClipKeeper!");\n}',
      timestamp: Date.now() - 1000 * 60 * 2,
      pinned: true
    },
    {
      id: 2,
      hash: 'h2',
      item_type: 'url',
      content: 'https://tauri.app/v2/',
      preview: 'https://tauri.app/v2/',
      timestamp: Date.now() - 1000 * 60 * 15,
      pinned: false
    },
    {
      id: 3,
      hash: 'h3',
      item_type: 'text',
      content: 'ClipKeeper is an ultra-lightweight system clipboard manager.',
      preview: 'ClipKeeper is an ultra-lightweight system clipboard manager.',
      timestamp: Date.now() - 1000 * 60 * 45,
      pinned: false
    }
  ];
}
