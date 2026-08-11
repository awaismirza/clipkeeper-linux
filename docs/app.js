// ClipKeeper Interactive Demo & Copy Helper
document.addEventListener('DOMContentLoaded', () => {
  const sampleClips = [
    { text: 'git commit -m "feat: restrict shortcut strictly to Alt+Shift+V"', type: 'code', time: 'Just now' },
    { text: 'https://github.com/awaismirza/clipkeeper-linux', type: 'text', time: '2 mins ago' },
    { text: 'wget https://github.com/awaismirza/clipkeeper-linux/releases/download/v0.1.1/ClipKeeper_0.1.1_arm64.deb', type: 'code', time: '5 mins ago' },
    { text: 'sudo dpkg -i ClipKeeper_0.1.1_arm64.deb', type: 'code', time: '10 mins ago' },
    { text: 'Alt + Shift + V (Global Linux Shortcut)', type: 'text', time: '1 hour ago' }
  ];

  const searchInput = document.getElementById('demo-search-input');
  const clipContainer = document.getElementById('demo-clip-container');
  const copyBtn = document.getElementById('copy-cmd-btn');
  const terminalCode = document.getElementById('terminal-code');

  function renderClips(filterText = '') {
    if (!clipContainer) return;
    
    const filtered = sampleClips.filter(c => 
      c.text.toLowerCase().includes(filterText.toLowerCase())
    );

    if (filtered.length === 0) {
      clipContainer.innerHTML = '<div style="color: #9ca3af; text-align: center; padding: 20px;">No matching items found</div>';
      return;
    }

    clipContainer.innerHTML = filtered.map((clip, index) => `
      <div class="clip-item ${index === 0 ? 'selected' : ''}" style="cursor: pointer;">
        <div class="clip-meta">
          <span class="clip-type ${clip.type}">${clip.type.toUpperCase()}</span>
          <span class="clip-time">${clip.time}</span>
        </div>
        <div class="clip-text">${escapeHtml(clip.text)}</div>
      </div>
    `).join('');
  }

  function escapeHtml(str) {
    return str.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
  }

  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      renderClips(e.target.value);
    });
    renderClips();
  }

  if (copyBtn && terminalCode) {
    copyBtn.addEventListener('click', () => {
      navigator.clipboard.writeText(terminalCode.innerText.trim()).then(() => {
        copyBtn.innerText = 'Copied!';
        setTimeout(() => {
          copyBtn.innerText = 'Copy Command';
        }, 2000);
      });
    });
  }
});
