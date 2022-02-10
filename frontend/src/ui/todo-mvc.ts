import { BaseHTMLElement, customElement, first, getChild, getChildren, html, OnEvent, onEvent, onHub } from 'dom-native';
import { Todo, todoMco } from '../model/todo-mco';

@customElement("todo-mvc")
class TodoMvc extends BaseHTMLElement { // extends HTMLElement
  #todoInputEl!: TodoInput;
  #todoListEl!: HTMLElement;

  init() {
    let htmlContent: DocumentFragment = html`
      <div class="box"></div>
      <h1>todos</h1>
      <todo-input></todo-input>
      <todo-list></todo-list>    
    `;
    [this.#todoInputEl, this.#todoListEl] =
      getChildren(htmlContent, 'todo-input', 'todo-list');

    this.append(htmlContent);
    this.refresh();
  }

  async refresh() {
    let todos: Todo[] = await todoMco.list();
    let htmlContent = document.createDocumentFragment();
    for (const todo of todos) {
      const el = document.createElement('todo-item');
      el.data = todo; // todo will be frozen
      htmlContent.append(el);
    }

    this.#todoListEl.innerHTML = '';
    this.#todoListEl.append(htmlContent);

  }

  // #region    --- UI Events
  @onEvent('pointerup', 'c-check')
  onCheckTodo(evt: PointerEvent & OnEvent) {
    const todoItem = evt.selectTarget.closest("todo-item")!;
    const status = todoItem.data.status == 'Open' ? 'Close' : 'Open';
    // update to server
    todoMco.update(todoItem.data.id, { status });
  }
  // #endregion --- UI Events

  // #region    --- Data Events
  @onHub('dataHub', 'Todo', 'update')
  onTodoUpdate(data: Todo) {
    // find the todo in the UI
    const todoItem = first(`todo-item.Todo-${data.id}`) as TodoItem | undefined;
    // if found, update it.
    if (todoItem) {
      todoItem.data = data; // data will be frozen
    }
  }

  @onHub('dataHub', 'Todo', 'create')
  onTodoCreate(data: Todo) {
    this.refresh();
  }
  // #endregion --- Data Events
}

@customElement("todo-input")
class TodoInput extends BaseHTMLElement { // extends HTMLElement
  #inputEl!: HTMLInputElement;

  init() {
    let htmlContent = html`
      <input type="text" placeholder="What needs to be done?">
    `;
    this.#inputEl = getChild(htmlContent, 'input');

    this.append(htmlContent);

  }

  // #region    --- UI Events
  @onEvent('keyup', 'input')
  onInputKeyUp(evt: KeyboardEvent) {
    if (evt.key == "Enter") {
      // get value from UI
      const title = this.#inputEl.value;
      // send create to server
      todoMco.create({ title });
      // don't wait, reset value input
      this.#inputEl.value = '';
    }
  }
  // #endregion --- UI Events
}
// todo-input tag
declare global {
  interface HTMLElementTagNameMap {
    'todo-input': TodoInput;
  }
}

@customElement('todo-item')
export class TodoItem extends BaseHTMLElement { // extends HTMLElement
  #titleEl!: HTMLElement;
  #data!: Todo;

  set data(data: Todo) {
    let oldData = this.#data;
    this.#data = Object.freeze(data);
    if (this.isConnected) {
      this.refresh(oldData);
    }
  }

  get data() { return this.#data }

  init() {
    let htmlContent = html`
			<c-check><c-ico name="ico-done"></c-ico></c-check>
			<div class="title">STATIC TITLE</div>
			<c-ico name="del"></c-ico>        
    `;
    this.#titleEl = getChild(htmlContent, 'div');

    this.append(htmlContent);
    this.refresh();
  }

  refresh(old?: Todo) {
    if (old != null) {
      this.classList.remove(`Todo-${old.id}`);
      this.classList.remove(old.status);
    }

    // render new data
    const todo = this.#data;
    this.classList.add(`Todo-${todo.id}`);
    this.classList.add(todo.status);
    this.#titleEl.textContent = todo.title;
  }
}
// todo-item type augmentation
declare global {
  interface HTMLElementTagNameMap {
    'todo-item': TodoItem;
  }
}
