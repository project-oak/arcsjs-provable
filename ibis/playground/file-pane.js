// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

const template = `
  <style>
    #content {
      font-size: 12px;
      min-width: 700px;
      padding: 10px;
      border: 1px solid #ccc;
    }
    .tab-panel {
      width: fit-content;
      margin: 10px 0 20px 0;
    }
    .tab-row {
      overflow: hidden;
      background-color: #f1f1f1;
      border: 1px solid #ccc;
      border-bottom: none;
    }
    .tab-row button {
      background-color: inherit;
      cursor: pointer;
      float: left;
      border: none;
      outline: none;
      padding: 10px 16px;
    }
    .tab-row button:hover {
      background-color: #ddd;
    }
    .tab-row button.active {
      background-color: #ccc;
    }
    #files textarea {
      font-size: 12px;
      min-width: 700px;
      width: 90vw;
      padding: 10px;
      border: 1px solid #ccc;
    }
  </style>
  <div class="tab-panel">
    <div class="tab-row">
        <div id="tabs"></div>
        <button id="add-button">+</button>
    </div>
    <div id="files"></div>
    </div>
  </div>`;

export class FilePane extends HTMLElement {

    constructor() {
        super();
        const shadowRoot = this.attachShadow({mode: 'open'});
        shadowRoot.innerHTML = template;

        this.tabs = shadowRoot.getElementById('tabs');
        this.files = shadowRoot.getElementById('files');
        this.addButton = shadowRoot.getElementById('add-button');

        this.files.addEventListener('keypress', this.interceptCtrlEnter.bind(this));
        this.addButton.addEventListener('click', this.addFile.bind(this));
        this.fileBase = 'a'.charCodeAt(0);
    }

    connectedCallback() {
    }

    init(executeCallback, exportButton) {
        this.executeCallback = executeCallback;
        this.exportButton = exportButton;
        this.exportButton.addEventListener('click', this.exportFiles.bind(this));
        this.addButton.addEventListener('click', this.addFile.bind(this));
    }

    interceptCtrlEnter(event) {
        if (event.key === 'Enter' && event.ctrlKey) {
            this.executeCallback();
            event.preventDefault();
        }
    }

    static get observedAttributes() {
        return ['no-add-button'];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        console.log(this, name, oldValue, newValue);
        if (name === 'no-add-button') {
            if (newValue) {
                this.addButton.style.display = 'none';
            } else {
                this.addButton.style.display = '';
            }
        }
    }

    addFile(event, content) {
        const file = document.createElement('textarea');
        file.rows = 10;
        file.spellcheck = false;
        file.addEventListener('keypress', this.interceptCtrlEnter.bind(this))
        file.value = content || '';

        const tab = document.createElement('button');
        tab.textContent = `${String.fromCharCode(this.fileBase++)}`;
        tab.linkedFile = file;
        tab.addEventListener('click', this.showFile.bind(this))

        this.tabs.appendChild(tab);
        this.files.appendChild(file);
        tab.click();

        if (this.fileBase > 'z'.charCodeAt(0)) {
            this.addButton.style.display = 'none';
        }
        return file;
    }

    dropAllFiles() {
        this.files.replaceChildren();
        this.tabs.replaceChildren();
        this.fileBase = 'a'.charCodeAt(0);
    }

    getFileContents() {
        return Array.from(this.files.children).map(file => file.value);
    }

    showFile(event) {
        for (const tab of this.tabs.children) {
            tab.classList.remove('active');
        }
        for (const file of this.files.children) {
            file.style.display = 'none';
        }
        event.target.classList.add('active');
        event.target.linkedFile.style.display = '';
    }
}
