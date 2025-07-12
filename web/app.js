// GigliOptix Playground Application
class GigliOptixPlayground {
  constructor() {
    this.editor = null;
    this.currentExample = "hello";
    this.examples = {};
    this.isRunning = false;

    this.init();
  }

  async init() {
    await this.loadExamples();
    this.setupEditor();
    this.setupEventListeners();
    this.loadExample("hello");
  }

  async loadExamples() {
    // Load example files
    const exampleFiles = [
      "hello",
      "counter",
      "todo",
      "calculator",
      "weather",
      "game",
    ];

    for (const example of exampleFiles) {
      try {
        const response = await fetch(`../examples/${example}.gx`);
        if (response.ok) {
          this.examples[example] = await response.text();
        } else {
          // Fallback to hardcoded examples if files not found
          this.examples[example] = this.getFallbackExample(example);
        }
      } catch (error) {
        console.warn(`Could not load ${example}.gx, using fallback`);
        this.examples[example] = this.getFallbackExample(example);
      }
    }
  }

  getFallbackExample(name) {
    const examples = {
      hello: `// Example: Hello World for the Web in GigliOptix
fn main() {
    dom::set_inner_html("app", "<h1>Hello, Web!</h1>");
}`,
      counter: `// Example: Counter with reactive UI in GigliOptix
view App {
  cell counter = 0

  on click: counter += 1

  style:
    background: "#000",
    color: if counter > 10 then "#f00" else "#0f0"

  render:
    text("Clicks: " + counter)
}`,
      todo: `// Example: Todo List with reactive UI in GigliOptix
view TodoApp {
  cell todos = []
  cell new_todo = ""

  fn add_todo() {
    if new_todo != "" {
      todos.push({
        id: Date::now(),
        text: new_todo,
        completed: false
      })
      new_todo = ""
    }
  }

  fn toggle_todo(id) {
    for todo in todos {
      if todo.id == id {
        todo.completed = !todo.completed
      }
    }
  }

  fn remove_todo(id) {
    todos = todos.filter(todo => todo.id != id)
  }

  style:
    font_family: "Arial, sans-serif",
    max_width: "500px",
    margin: "0 auto",
    padding: "20px"

  render:
    div {
      h1("Todo List")

      div {
        input(
          value: new_todo,
          on_input: new_todo = event.target.value,
          placeholder: "Add a new todo..."
        )
        button(on_click: add_todo)("Add")
      }

      ul {
        for todo in todos {
          li {
            input(
              type: "checkbox",
              checked: todo.completed,
              on_change: toggle_todo(todo.id)
            )
            span(
              style: if todo.completed then "text-decoration: line-through" else ""
            )(todo.text)
            button(on_click: remove_todo(todo.id))("Delete")
          }
        }
      }

      p("Total: " + todos.length + " items")
    }
}`,
      calculator: `// Example: Calculator with reactive UI in GigliOptix
view Calculator {
  cell display = "0"
  cell current_number = ""
  cell operation = ""
  cell previous_number = 0
  cell waiting_for_number = true

  fn append_digit(digit) {
    if waiting_for_number {
      display = digit
      waiting_for_number = false
    } else {
      display = display + digit
    }
    current_number = display
  }

  fn set_operation(op) {
    if !waiting_for_number {
      calculate()
    }
    operation = op
    previous_number = parse_float(current_number)
    waiting_for_number = true
  }

  fn calculate() {
    if operation != "" && !waiting_for_number {
      let current = parse_float(current_number)
      let result = match operation {
        "+" => previous_number + current,
        "-" => previous_number - current,
        "*" => previous_number * current,
        "/" => if current != 0 then previous_number / current else 0,
        _ => current
      }
      display = result.to_string()
      current_number = display
      operation = ""
      waiting_for_number = true
    }
  }

  fn clear() {
    display = "0"
    current_number = ""
    operation = ""
    previous_number = 0
    waiting_for_number = true
  }

  style:
    display: "grid",
    grid_template_columns: "repeat(4, 1fr)",
    gap: "5px",
    max_width: "300px",
    margin: "0 auto",
    padding: "20px",
    background: "#f0f0f0",
    border_radius: "10px"

  render:
    div {
      div(
        style: "grid-column: 1 / -1; background: white; padding: 10px; text-align: right; font-size: 24px; margin-bottom: 10px; border-radius: 5px"
      )(display)

      button(on_click: clear, style: "grid-column: 1 / 3; background: #ff6b6b; color: white")("Clear")
      button(on_click: set_operation("/"), style: "background: #4ecdc4")("/")
      button(on_click: set_operation("*"), style: "background: #4ecdc4")("*")

      button(on_click: append_digit("7"))("7")
      button(on_click: append_digit("8"))("8")
      button(on_click: append_digit("9"))("9")
      button(on_click: set_operation("-"), style: "background: #4ecdc4")("-")

      button(on_click: append_digit("4"))("4")
      button(on_click: append_digit("5"))("5")
      button(on_click: append_digit("6"))("6")
      button(on_click: set_operation("+"), style: "background: #4ecdc4")("+")

      button(on_click: append_digit("1"))("1")
      button(on_click: append_digit("2"))("2")
      button(on_click: append_digit("3"))("3")
      button(on_click: calculate, style: "grid-row: span 2; background: #45b7d1; color: white")("=")

      button(on_click: append_digit("0"), style: "grid-column: span 2")("0")
      button(on_click: append_digit("."))(".")
    }
}`,
      weather: `// Example: Weather App with API integration in GigliOptix
view WeatherApp {
  cell weather_data = null
  cell city = "London"
  cell loading = false
  cell error = ""

  fn fetch_weather() {
    loading = true
    error = ""

    // Simulate API call (in real implementation, this would call a weather API)
    setTimeout(() => {
      weather_data = {
        city: city,
        temperature: 22,
        condition: "Sunny",
        humidity: 65,
        wind_speed: 12,
        icon: "☀️"
      }
      loading = false
    }, 1000)
  }

  fn update_city(new_city) {
    city = new_city
    fetch_weather()
  }

  on_mount: fetch_weather()

  style:
    font_family: "Arial, sans-serif",
    max_width: "400px",
    margin: "0 auto",
    padding: "20px",
    background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    color: "white",
    border_radius: "15px",
    text_align: "center"

  render:
    div {
      h1("Weather App")

      div {
        input(
          value: city,
          on_input: city = event.target.value,
          placeholder: "Enter city name...",
          style: "padding: 10px; border: none; border-radius: 5px; margin-right: 10px; width: 200px"
        )
        button(
          on_click: update_city(city),
          style: "padding: 10px 20px; background: #4CAF50; color: white; border: none; border-radius: 5px; cursor: pointer"
        )("Search")
      }

      if loading {
        div("Loading weather data...")
      } else if error != "" {
        div(style: "color: #ff6b6b")(error)
      } else if weather_data != null {
        div {
          h2(weather_data.city)
          div(style: "font-size: 48px; margin: 20px 0")(weather_data.icon)
          div(style: "font-size: 36px; font-weight: bold")(weather_data.temperature + "°C")
          div(style: "font-size: 24px; margin: 10px 0")(weather_data.condition)

          div(style: "display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-top: 30px") {
            div {
              div("Humidity")
              div(style: "font-size: 20px; font-weight: bold")(weather_data.humidity + "%")
            }
            div {
              div("Wind Speed")
              div(style: "font-size: 20px; font-weight: bold")(weather_data.wind_speed + " km/h")
            }
          }
        }
      }
    }
}`,
      game: `// Example: Simple Snake Game in GigliOptix
view SnakeGame {
  cell snake = [{x: 10, y: 10}]
  cell food = {x: 15, y: 15}
  cell direction = "right"
  cell game_over = false
  cell score = 0
  cell game_speed = 200

  fn move_snake() {
    if game_over {
      return
    }

    let head = snake[0]
    let new_head = match direction {
      "up" => {x: head.x, y: head.y - 1},
      "down" => {x: head.x, y: head.y + 1},
      "left" => {x: head.x - 1, y: head.y},
      "right" => {x: head.x + 1, y: head.y}
    }

    // Check wall collision
    if new_head.x < 0 || new_head.x >= 20 || new_head.y < 0 || new_head.y >= 20 {
      game_over = true
      return
    }

    // Check self collision
    for segment in snake {
      if segment.x == new_head.x && segment.y == new_head.y {
        game_over = true
        return
      }
    }

    snake.insert(0, new_head)

    // Check food collision
    if new_head.x == food.x && new_head.y == food.y {
      score += 10
      generate_food()
    } else {
      snake.pop()
    }
  }

  fn generate_food() {
    food = {
      x: Math::random() * 20,
      y: Math::random() * 20
    }
  }

  fn change_direction(new_direction) {
    let opposites = {
      "up": "down",
      "down": "up",
      "left": "right",
      "right": "left"
    }

    if opposites[direction] != new_direction {
      direction = new_direction
    }
  }

  fn restart_game() {
    snake = [{x: 10, y: 10}]
    food = {x: 15, y: 15}
    direction = "right"
    game_over = false
    score = 0
  }

  on_mount: {
    setInterval(move_snake, game_speed)

    // Keyboard controls
    document.addEventListener("keydown", (event) => {
      match event.key {
        "ArrowUp" => change_direction("up"),
        "ArrowDown" => change_direction("down"),
        "ArrowLeft" => change_direction("left"),
        "ArrowRight" => change_direction("right"),
        "r" => restart_game(),
        _ => {}
      }
    })
  }

  style:
    font_family: "Arial, sans-serif",
    text_align: "center",
    max_width: "600px",
    margin: "0 auto",
    padding: "20px"

  render:
    div {
      h1("Snake Game")
      div(style: "font-size: 24px; margin: 10px 0")("Score: " + score)

      if game_over {
        div {
          h2(style: "color: #ff6b6b")("Game Over!")
          button(
            on_click: restart_game,
            style: "padding: 10px 20px; background: #4CAF50; color: white; border: none; border-radius: 5px; cursor: pointer"
          )("Restart")
        }
      }

      div(
        style: "display: grid; grid-template-columns: repeat(20, 20px); gap: 1px; background: #333; padding: 10px; margin: 20px auto; width: fit-content"
      ) {
        for y in 0..20 {
          for x in 0..20 {
            let is_snake = snake.some(segment => segment.x == x && segment.y == y)
            let is_food = food.x == x && food.y == y

            div(
              style: "width: 20px; height: 20px; background: " +
                     if is_snake then "#4CAF50" else
                     if is_food then "#ff6b6b" else "#222"
            )("")
          }
        }
      }

      div(style: "margin-top: 20px") {
        p("Use arrow keys to control the snake")
        p("Press 'R' to restart")
      }
    }
}`,
    };

    return examples[name] || examples.hello;
  }

  setupEditor() {
    this.editor = CodeMirror.fromTextArea(
      document.getElementById("code-editor"),
      {
        mode: "javascript",
        theme: "monokai",
        lineNumbers: true,
        autoCloseBrackets: true,
        matchBrackets: true,
        indentUnit: 2,
        tabSize: 2,
        lineWrapping: true,
        foldGutter: true,
        gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
        extraKeys: {
          "Ctrl-Space": "autocomplete",
          Tab: function (cm) {
            if (cm.somethingSelected()) {
              cm.indentSelection("add");
            } else {
              cm.replaceSelection("  ", "end");
            }
          },
        },
      }
    );

    // Set initial size
    this.editor.setSize("100%", "100%");
  }

  setupEventListeners() {
    // Example selector buttons
    document.querySelectorAll(".example-btn").forEach((btn) => {
      btn.addEventListener("click", (e) => {
        const example = e.target.dataset.example;
        this.loadExample(example);

        // Update active button
        document
          .querySelectorAll(".example-btn")
          .forEach((b) => b.classList.remove("active"));
        e.target.classList.add("active");
      });
    });

    // Run button
    document.getElementById("run-btn").addEventListener("click", () => {
      this.runCode();
    });

    // Keyboard shortcuts
    document.addEventListener("keydown", (e) => {
      if (e.ctrlKey && e.key === "Enter") {
        e.preventDefault();
        this.runCode();
      }
    });
  }

  loadExample(name) {
    this.currentExample = name;
    const code = this.examples[name];
    if (code) {
      this.editor.setValue(code);
      this.editor.refresh();
    }
  }

  async runCode() {
    if (this.isRunning) return;

    this.isRunning = true;
    const runBtn = document.getElementById("run-btn");
    const outputContent = document.getElementById("output-content");

    // Update UI
    runBtn.disabled = true;
    runBtn.textContent = "🔄 Running...";

    // Show loading
    outputContent.innerHTML = `
            <div class="loading">
                <div class="spinner"></div>
                <span>Compiling and running your GigliOptix code...</span>
            </div>
        `;

    try {
      const code = this.editor.getValue();

      // Simulate compilation and execution
      await this.simulateExecution(code);
    } catch (error) {
      this.showError("Execution failed: " + error.message);
    } finally {
      this.isRunning = false;
      runBtn.disabled = false;
      runBtn.textContent = "▶️ Run Code";
    }
  }

  async simulateExecution(code) {
    // Simulate compilation time
    await new Promise((resolve) =>
      setTimeout(resolve, 1000 + Math.random() * 2000)
    );

    const outputContent = document.getElementById("output-content");

    // Create a sandboxed iframe for output
    const iframe = document.createElement("iframe");
    iframe.className = "output-frame";
    iframe.sandbox = "allow-scripts allow-same-origin";

    // Generate HTML content based on the code
    const htmlContent = this.generateOutputHTML(code);

    outputContent.innerHTML = "";
    outputContent.appendChild(iframe);

    // Write content to iframe
    const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;
    iframeDoc.open();
    iframeDoc.write(htmlContent);
    iframeDoc.close();

    this.showSuccess("Code executed successfully!");
  }

  generateOutputHTML(code) {
    // Simple code analysis to generate appropriate output
    if (code.includes("dom::set_inner_html")) {
      return `
                <!DOCTYPE html>
                <html>
                <head>
                    <style>
                        body { font-family: Arial, sans-serif; padding: 20px; }
                    </style>
                </head>
                <body>
                    <div id="app"></div>
                    <script>
                        // Simulated GigliOptix runtime
                        const dom = {
                            set_inner_html: (id, html) => {
                                document.getElementById(id).innerHTML = html;
                            }
                        };

                        // Execute the main function
                        function main() {
                            dom.set_inner_html("app", "<h1>Hello, Web!</h1>");
                        }
                        main();
                    </script>
                </body>
                </html>
            `;
    } else if (code.includes("view")) {
      // For reactive views, create a more complex simulation
      return `
                <!DOCTYPE html>
                <html>
                <head>
                    <style>
                        body { font-family: Arial, sans-serif; padding: 20px; }
                        .counter { text-align: center; padding: 20px; }
                        .counter button {
                            padding: 10px 20px;
                            font-size: 18px;
                            margin: 10px;
                            border: none;
                            border-radius: 5px;
                            cursor: pointer;
                        }
                        .todo-app { max-width: 500px; margin: 0 auto; }
                        .todo-item { display: flex; align-items: center; margin: 10px 0; }
                        .calculator {
                            display: grid;
                            grid-template-columns: repeat(4, 1fr);
                            gap: 5px;
                            max-width: 300px;
                            margin: 0 auto;
                        }
                        .calculator button {
                            padding: 15px;
                            font-size: 18px;
                            border: none;
                            border-radius: 5px;
                            cursor: pointer;
                        }
                        .weather-app {
                            max-width: 400px;
                            margin: 0 auto;
                            text-align: center;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                            color: white;
                            padding: 20px;
                            border-radius: 15px;
                        }
                        .snake-game { text-align: center; }
                        .game-grid {
                            display: grid;
                            grid-template-columns: repeat(20, 20px);
                            gap: 1px;
                            background: #333;
                            padding: 10px;
                            margin: 20px auto;
                            width: fit-content;
                        }
                        .game-cell { width: 20px; height: 20px; }
                    </style>
                </head>
                <body>
                    <div id="app">
                        <div class="counter">
                            <h2>Counter Example</h2>
                            <div id="counter-display">Clicks: 0</div>
                            <button onclick="incrementCounter()" style="background: #4CAF50; color: white;">Click me!</button>
                        </div>

                        <div class="todo-app">
                            <h2>Todo List Example</h2>
                            <div>
                                <input type="text" id="todo-input" placeholder="Add a new todo..." style="padding: 10px; margin-right: 10px;">
                                <button onclick="addTodo()" style="background: #4CAF50; color: white; padding: 10px;">Add</button>
                            </div>
                            <ul id="todo-list"></ul>
                            <p id="todo-count">Total: 0 items</p>
                        </div>

                        <div class="calculator">
                            <h2>Calculator Example</h2>
                            <div style="grid-column: 1 / -1; background: white; padding: 10px; text-align: right; font-size: 24px; margin-bottom: 10px; border-radius: 5px; color: black;" id="calc-display">0</div>
                            <button onclick="calcClear()" style="grid-column: 1 / 3; background: #ff6b6b; color: white;">Clear</button>
                            <button onclick="calcOp('/')" style="background: #4ecdc4;">/</button>
                            <button onclick="calcOp('*')" style="background: #4ecdc4;">*</button>
                            <button onclick="calcDigit('7')">7</button>
                            <button onclick="calcDigit('8')">8</button>
                            <button onclick="calcDigit('9')">9</button>
                            <button onclick="calcOp('-')" style="background: #4ecdc4;">-</button>
                            <button onclick="calcDigit('4')">4</button>
                            <button onclick="calcDigit('5')">5</button>
                            <button onclick="calcDigit('6')">6</button>
                            <button onclick="calcOp('+')" style="background: #4ecdc4;">+</button>
                            <button onclick="calcDigit('1')">1</button>
                            <button onclick="calcDigit('2')">2</button>
                            <button onclick="calcDigit('3')">3</button>
                            <button onclick="calcEquals()" style="grid-row: span 2; background: #45b7d1; color: white;">=</button>
                            <button onclick="calcDigit('0')" style="grid-column: span 2">0</button>
                            <button onclick="calcDigit('.')">.</button>
                        </div>

                        <div class="weather-app">
                            <h2>Weather App Example</h2>
                            <div>
                                <input type="text" id="city-input" value="London" placeholder="Enter city name..." style="padding: 10px; border: none; border-radius: 5px; margin-right: 10px; width: 200px; color: black;">
                                <button onclick="fetchWeather()" style="padding: 10px 20px; background: #4CAF50; color: white; border: none; border-radius: 5px; cursor: pointer;">Search</button>
                            </div>
                            <div id="weather-display">
                                <h3>London</h3>
                                <div style="font-size: 48px; margin: 20px 0;">☀️</div>
                                <div style="font-size: 36px; font-weight: bold;">22°C</div>
                                <div style="font-size: 24px; margin: 10px 0;">Sunny</div>
                            </div>
                        </div>

                        <div class="snake-game">
                            <h2>Snake Game Example</h2>
                            <div style="font-size: 24px; margin: 10px 0;">Score: <span id="game-score">0</span></div>
                            <div class="game-grid" id="game-grid"></div>
                            <p>Use arrow keys to control the snake</p>
                            <p>Press 'R' to restart</p>
                        </div>
                    </div>

                    <script>
                        // Counter functionality
                        let counter = 0;
                        function incrementCounter() {
                            counter++;
                            document.getElementById('counter-display').textContent = 'Clicks: ' + counter;
                            document.getElementById('counter-display').style.color = counter > 10 ? '#f00' : '#0f0';
                        }

                        // Todo functionality
                        let todos = [];
                        function addTodo() {
                            const input = document.getElementById('todo-input');
                            if (input.value.trim()) {
                                todos.push({
                                    id: Date.now(),
                                    text: input.value,
                                    completed: false
                                });
                                input.value = '';
                                renderTodos();
                            }
                        }

                        function renderTodos() {
                            const list = document.getElementById('todo-list');
                            list.innerHTML = '';
                            todos.forEach(todo => {
                                const li = document.createElement('li');
                                li.className = 'todo-item';
                                li.innerHTML = \`
                                    <input type="checkbox" \${todo.completed ? 'checked' : ''} onchange="toggleTodo(\${todo.id})">
                                    <span style="\${todo.completed ? 'text-decoration: line-through' : ''}">\${todo.text}</span>
                                    <button onclick="removeTodo(\${todo.id})" style="margin-left: 10px; background: #ff6b6b; color: white; border: none; padding: 5px 10px; border-radius: 3px;">Delete</button>
                                \`;
                                list.appendChild(li);
                            });
                            document.getElementById('todo-count').textContent = 'Total: ' + todos.length + ' items';
                        }

                        function toggleTodo(id) {
                            const todo = todos.find(t => t.id === id);
                            if (todo) todo.completed = !todo.completed;
                            renderTodos();
                        }

                        function removeTodo(id) {
                            todos = todos.filter(t => t.id !== id);
                            renderTodos();
                        }

                        // Calculator functionality
                        let calcDisplay = '0';
                        let calcCurrentNumber = '';
                        let calcOperation = '';
                        let calcPreviousNumber = 0;
                        let calcWaitingForNumber = true;

                        function calcDigit(digit) {
                            if (calcWaitingForNumber) {
                                calcDisplay = digit;
                                calcWaitingForNumber = false;
                            } else {
                                calcDisplay += digit;
                            }
                            calcCurrentNumber = calcDisplay;
                            document.getElementById('calc-display').textContent = calcDisplay;
                        }

                        function calcOp(op) {
                            if (!calcWaitingForNumber) {
                                calcEquals();
                            }
                            calcOperation = op;
                            calcPreviousNumber = parseFloat(calcCurrentNumber);
                            calcWaitingForNumber = true;
                        }

                        function calcEquals() {
                            if (calcOperation && !calcWaitingForNumber) {
                                const current = parseFloat(calcCurrentNumber);
                                let result;
                                switch (calcOperation) {
                                    case '+': result = calcPreviousNumber + current; break;
                                    case '-': result = calcPreviousNumber - current; break;
                                    case '*': result = calcPreviousNumber * current; break;
                                    case '/': result = current !== 0 ? calcPreviousNumber / current : 0; break;
                                    default: result = current;
                                }
                                calcDisplay = result.toString();
                                calcCurrentNumber = calcDisplay;
                                calcOperation = '';
                                calcWaitingForNumber = true;
                                document.getElementById('calc-display').textContent = calcDisplay;
                            }
                        }

                        function calcClear() {
                            calcDisplay = '0';
                            calcCurrentNumber = '';
                            calcOperation = '';
                            calcPreviousNumber = 0;
                            calcWaitingForNumber = true;
                            document.getElementById('calc-display').textContent = calcDisplay;
                        }

                        // Weather functionality
                        function fetchWeather() {
                            const city = document.getElementById('city-input').value;
                            document.getElementById('weather-display').innerHTML = \`
                                <h3>\${city}</h3>
                                <div style="font-size: 48px; margin: 20px 0;">☀️</div>
                                <div style="font-size: 36px; font-weight: bold;">22°C</div>
                                <div style="font-size: 24px; margin: 10px 0;">Sunny</div>
                            \`;
                        }

                        // Snake game functionality
                        let snake = [{x: 10, y: 10}];
                        let food = {x: 15, y: 15};
                        let direction = 'right';
                        let gameOver = false;
                        let score = 0;

                        function initGame() {
                            const grid = document.getElementById('game-grid');
                            grid.innerHTML = '';
                            for (let y = 0; y < 20; y++) {
                                for (let x = 0; x < 20; x++) {
                                    const cell = document.createElement('div');
                                    cell.className = 'game-cell';
                                    const isSnake = snake.some(segment => segment.x === x && segment.y === y);
                                    const isFood = food.x === x && food.y === y;
                                    cell.style.background = isSnake ? '#4CAF50' : isFood ? '#ff6b6b' : '#222';
                                    grid.appendChild(cell);
                                }
                            }
                        }

                        function moveSnake() {
                            if (gameOver) return;

                            const head = snake[0];
                            let newHead;
                            switch (direction) {
                                case 'up': newHead = {x: head.x, y: head.y - 1}; break;
                                case 'down': newHead = {x: head.x, y: head.y + 1}; break;
                                case 'left': newHead = {x: head.x - 1, y: head.y}; break;
                                case 'right': newHead = {x: head.x + 1, y: head.y}; break;
                            }

                            if (newHead.x < 0 || newHead.x >= 20 || newHead.y < 0 || newHead.y >= 20) {
                                gameOver = true;
                                return;
                            }

                            if (snake.some(segment => segment.x === newHead.x && segment.y === newHead.y)) {
                                gameOver = true;
                                return;
                            }

                            snake.unshift(newHead);

                            if (newHead.x === food.x && newHead.y === food.y) {
                                score += 10;
                                document.getElementById('game-score').textContent = score;
                                food = {
                                    x: Math.floor(Math.random() * 20),
                                    y: Math.floor(Math.random() * 20)
                                };
                            } else {
                                snake.pop();
                            }

                            initGame();
                        }

                        document.addEventListener('keydown', (event) => {
                            switch (event.key) {
                                case 'ArrowUp': if (direction !== 'down') direction = 'up'; break;
                                case 'ArrowDown': if (direction !== 'up') direction = 'down'; break;
                                case 'ArrowLeft': if (direction !== 'right') direction = 'left'; break;
                                case 'ArrowRight': if (direction !== 'left') direction = 'right'; break;
                                case 'r':
                                    snake = [{x: 10, y: 10}];
                                    food = {x: 15, y: 15};
                                    direction = 'right';
                                    gameOver = false;
                                    score = 0;
                                    document.getElementById('game-score').textContent = score;
                                    break;
                            }
                        });

                        // Initialize game
                        initGame();
                        setInterval(moveSnake, 200);
                    </script>
                </body>
                </html>
            `;
    } else {
      return `
                <!DOCTYPE html>
                <html>
                <head>
                    <style>
                        body { font-family: Arial, sans-serif; padding: 20px; }
                        .output { background: #f8f9fa; padding: 20px; border-radius: 5px; }
                    </style>
                </head>
                <body>
                    <div class="output">
                        <h3>Code Output:</h3>
                        <pre>${this.escapeHtml(code)}</pre>
                        <p><em>This is a simulation of GigliOptix code execution. In a real implementation, this would be compiled to WebAssembly and run natively in the browser.</em></p>
                    </div>
                </body>
                </html>
            `;
    }
  }

  escapeHtml(text) {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }

  showError(message) {
    const outputContent = document.getElementById("output-content");
    outputContent.innerHTML = `
            <div class="error">
                <strong>Error:</strong> ${message}
            </div>
        `;
  }

  showSuccess(message) {
    const outputContent = document.getElementById("output-content");
    const successDiv = document.createElement("div");
    successDiv.className = "success";
    successDiv.innerHTML = `<strong>Success:</strong> ${message}`;
    outputContent.insertBefore(successDiv, outputContent.firstChild);

    // Remove success message after 3 seconds
    setTimeout(() => {
      if (successDiv.parentNode) {
        successDiv.remove();
      }
    }, 3000);
  }
}

// Initialize the playground when the page loads
document.addEventListener("DOMContentLoaded", () => {
  new GigliOptixPlayground();
});
