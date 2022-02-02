import { BaseHTMLElement, customElement, html } from 'dom-native';

/**
 * c-ico - svg icons waring the svg use
 * Note: Assume the symbol are local to the document
 */
@customElement('c-ico')
class Ico extends BaseHTMLElement {
  init() {
    const name = this.getAttribute("name")?.trim();

    const htmlContent = html`<svg class="symbol">
    	<use xlink:href="#${name}"></use>
    </svg>`;
    this.append(htmlContent);
  }
}