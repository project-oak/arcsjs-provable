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
      width: 100%-10px;
      border: 1px solid #ccc;
    }
  </style>
  <div id="self">
      <div id="name"></div>
      <div class="tab-panel">
        <div class="tab-row">
            <div id="tabs"></div>
            <button id="add-button">+</button>
            <button id="delete-button">-</button>
            <button id="download-button">&darr;</button>
        </div>
        <div id="files"></div>
        </div>
      </div>
  </div>`;

export class FilePane extends HTMLElement {

    constructor() {
        super();
        const shadowRoot = this.attachShadow({mode: 'open'});
        shadowRoot.innerHTML = template;

        this.self = shadowRoot.getElementById('self');
        this.name = shadowRoot.getElementById('name');
        this.tabs = shadowRoot.getElementById('tabs');
        this.files = shadowRoot.getElementById('files');
        this.addButton = shadowRoot.getElementById('add-button');
        this.deleteButton = shadowRoot.getElementById('delete-button');
        this.downloadButton = shadowRoot.getElementById('download-button');

        this.files.addEventListener('keypress', this.interceptCtrlEnter.bind(this));
        this.addButton.addEventListener('click', this.addFile.bind(this));
        this.deleteButton.addEventListener('click', this.deleteCurrent.bind(this));
        this.downloadButton.addEventListener('click', this.download.bind(this));
        this.fileBase = 'a'.charCodeAt(0);
        this.ext = '';
    }

    connectedCallback() {
    }

    addExecuteCallback(executeCallback) {
        this.executeCallback = executeCallback;
    }

    addTabSwitchCallback(tabSwitchCallback) {
        this.tabSwitchCallback = tabSwitchCallback;
    }

    interceptCtrlEnter(event) {
        if (event.key === 'Enter' && event.ctrlKey) {
            if (this.executeCallback) {
                this.executeCallback();
            }
            event.preventDefault();
        }
    }

    download() {
        const textInput = this.active.value;
        let filename = 'unknown';
        for (const tab of this.tabs.children) {
            if (tab.linkedFile === this.active) {
                filename = tab.textContent;
                break;
            }
        }
        var element = document.createElement('a');
        element.setAttribute('href','data:text/plain;charset=utf-8, ' + encodeURIComponent(textInput));
        element.setAttribute('download', filename);
        document.body.appendChild(element);
        element.click();
        document.body.removeChild(element);
    }

    static get observedAttributes() {
        return ['no-add-button', 'ext', 'name'];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'no-add-button') {
            if (newValue) {
                this.addButton.style.display = 'none';
            } else {
                this.addButton.style.display = '';
            }
            this.updateHiddenState();
        } else if (name === 'ext') {
            this.ext = newValue;
        } else if (name === 'name') {
            this.name.textContent = newValue;
        }
    }

    updateHiddenState() {
        if (this.addButton.style.display === 'none' && this.files.children.length === 0) {
            this.self.style.display = 'none';
        } else {
            this.self.style.display = '';
        }
    }

    addFile(event, content, filename) {
        const file = document.createElement('textarea');
        file.rows = 10;
        file.spellcheck = false;
        file.addEventListener('keypress', this.interceptCtrlEnter.bind(this))
        file.value = content || '';

        const tab = document.createElement('button');
        if (filename) {
            tab.textContent = filename;
        } else {
            tab.textContent = `${String.fromCharCode(this.fileBase++)}${this.ext || ''}`;
        }
        tab.linkedFile = file;
        tab.addEventListener('click', this.showFile.bind(this))

        this.tabs.appendChild(tab);
        this.files.appendChild(file);
        this.updateHiddenState();
        tab.click();

        if (this.fileBase > 'z'.charCodeAt(0)) {
            this.addButton.style.display = 'none';
        }
        return file;
    }

    deleteCurrent() {
        if (!this.active) {
            console.log('no active file');
            return;
        }
        for (const tab of this.tabs.children) {
            if (tab.linkedFile === this.active) {
                tab.remove();
            }
        }
        this.active.remove();
        this.updateHiddenState();
        const firstTab = this.tabs.children[0];
        if (firstTab) {
            this.showFile({target: firstTab});
        }
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
        this.active = event.target.linkedFile;
        if (this.tabSwitchCallback) {
            this.tabSwitchCallback();
        }
    }
}
