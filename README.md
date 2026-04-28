# Rust Custom Handler for Azure Functions

A complete example of creating an Azure Functions custom handler using Rust with the Warp web framework.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Project Structure](#project-structure)
- [Installation Steps](#installation-steps)
- [Running the Function Locally](#running-the-function-locally)
- [Testing the Function](#testing-the-function)
- [How It Works](#how-it-works)
- [Troubleshooting](#troubleshooting)
- [Clean Up](#clean-up)

---

## Prerequisites

Before you begin, you need to install the following tools on your computer:

### 1. Rust Programming Language

**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and install from: https://rustup.rs

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify by opening a new terminal and running:
```bash
rustc --version
```

### 2. Azure Functions Core Tools

**macOS (using Homebrew):**
```bash
brew install azure-functions-core-tools@4
```

**Windows (using npm):**
```bash
npm install -g azure-functions-core-tools@4
```

**Linux (using npm):**
```bash
sudo npm install -g azure-functions-core-tools@4
```

Verify installation:
```bash
func --version
```

### 3. Visual Studio Code (Recommended)

Download from: https://code.visualstudio.com/

Install the Azure Functions extension:
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Azure Functions"
4. Click Install

---

## Project Structure

This project contains the following files:

```
rust-custom-handler/
├── Cargo.toml           # Rust project configuration
├── host.json            # Azure Functions host configuration (CRITICAL for custom handler)
├── local.settings.json  # Local development settings
├── httpTrigger/
│   └── function.json    # HTTP trigger function definition
├── src/
│   └── main.rs          # Rust custom handler code
└── handler              # Compiled executable (generated)
```

### Understanding host.json

The `host.json` file is the **most critical configuration** for Azure Functions Custom Handler. Here's what each part does:

```json
{
  "customHandler": {
    "description": {
      "defaultExecutablePath": "handler",  // Your compiled Rust binary
      "workingDirectory": "",
      "arguments": []
    },
    "enableForwardingHttpRequest": true    // CRITICAL: Forwards regular HTTP requests
  }
}
```

**Key Setting: `enableForwardingHttpRequest: true`**

- When set to `true`, Azure Functions forwards **regular HTTP requests** to your custom handler
- Without this, Azure sends a complex JSON payload that requires custom parsing
- This setting makes development much simpler - your handler receives standard HTTP requests

> ⚠️ **Important:** This setting must remain in host.json. If you modify host.json, ensure `enableForwardingHttpRequest: true` is preserved after every build or configuration change.

---

## Installation Steps

### Step 1: Clone the Repository

```bash
git clone <repository-url>
cd rust-custom-handler
```

### Step 2: Install Rust Dependencies

```bash
cargo build --release
```

This may take a few minutes on first run as Rust downloads and compiles dependencies.

> ⚠️ **Important:** After building, verify that `host.json` still contains `"enableForwardingHttpRequest": true`. This setting is essential for the custom handler to work correctly.

### Step 3: Copy the Executable

```bash
cp target/release/handler .
```

---

## Running the Function Locally

### Option 1: Using VS Code

1. Open the project folder in VS Code
2. Press F1 and type "Azure Functions: Start"
3. Select "Start"
4. The function will start at http://localhost:7071

### Option 2: Using Terminal

```bash
func start
```

You should see output similar to:

```
Azure Functions Core Tools
Core Tools Version: 4.x.x
Function Runtime Version: 4.x.x

Functions:

        httpTrigger: [GET, POST] http://localhost:7071/api/httpTrigger
```

---

## Testing the Function

### Test 1: GET Request with Query Parameter

Open your browser and visit:
```
http://localhost:7071/api/httpTrigger?name=World
```

**Expected Response:**
```
Hello, World. This HTTP triggered function executed successfully.
```

### Test 2: GET Request without Parameter

Visit:
```
http://localhost:7071/api/httpTrigger
```

**Expected Response:**
```
This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.
```

### Test 3: POST Request with JSON Body

Using cURL:
```bash
curl --location --request POST 'http://localhost:7071/api/httpTrigger' \
--header 'Content-Type: application/json' \
--data '{"name": "Amit"}'
```

**Expected Response:**
```
Hello, Amit. This HTTP triggered function executed successfully.
```

### Test 4: Using Postman

1. Open Postman
2. Create a new request
3. Set method to GET or POST
4. Enter URL: `http://localhost:7071/api/httpTrigger`
5. For GET: Add query parameter `name=YourName`
6. For POST: Select Body > JSON, enter `{"name": "YourName"}`
7. Click Send

---

## How It Works

### Azure Functions Custom Handler

Azure Functions Custom Handler allows you to run any executable as a function. Here's how it works:

1. **host.json** - Configures Azure Functions to use your custom handler executable
2. **FUNCTIONS_CUSTOMHANDLER_PORT** - Azure Functions tells your handler which port to listen on
3. **enableForwardingHttpRequest: true** - Forwards regular HTTP requests to your handler

### The host.json Configuration (Critical)

The `host.json` file is the key to making custom handlers work:

```json
{
  "customHandler": {
    "description": {
      "defaultExecutablePath": "handler"
    },
    "enableForwardingHttpRequest": true
  }
}
```

| Setting | Description |
|---------|-------------|
| `defaultExecutablePath` | Points to your compiled Rust binary (`handler`) |
| `enableForwardingHttpRequest` | When `true`, forwards HTTP requests directly to your handler instead of using the custom payload format |

**Why `enableForwardingHttpRequest: true` is important:**

- Without it: Azure sends a complex JSON payload that requires parsing
- With it: Your handler receives standard HTTP requests (like regular web servers)

> ⚠️ **Never remove or set `enableForwardingHttpRequest` to false** unless you want to handle the complex custom handler protocol manually.

### The Rust Code

The Rust code uses the **Warp** web framework to:

1. Listen on the port specified by Azure Functions (default: 8080)
2. Handle GET requests at `/api/httpTrigger`
3. Handle POST requests at `/api/httpTrigger`
4. Return personalized messages based on the "name" parameter

---

## Troubleshooting

### Issue: "405 Method Not Allowed"

**Cause:** The route path doesn't match the function name.

**Solution:** Ensure your Rust code uses `httpTrigger` (lowercase) matching the folder name.

### Issue: "500 Internal Server Error"

**Cause:** The handler executable is not running or crashed.

**Solution:**
1. Rebuild: `cargo build --release`
2. Copy: `cp target/release/handler .`
3. Restart: Stop func (Ctrl+C) and run `func start` again

### Issue: "Unable to create client for AzureWebJobsStorage"

**Cause:** Missing storage configuration.

**Solution:** This is just a warning. If it causes issues, update local.settings.json:

```json
{
  "IsEncrypted": false,
  "Values": {
    "AzureWebJobsStorage": "UseDevelopmentStorage=true",
    "FUNCTIONS_WORKER_RUNTIME": "custom"
  }
}
```

### Issue: "Port already in use"

**Cause:** Another process is using port 7071.

**Solution:** Find and stop the process, or change the port in local.settings.json.

---

## Clean Up

### Stop the Function

Press `Ctrl+C` in the terminal where func is running.

### Remove Build Files (Optional)

```bash
rm handler
cargo clean
```

---

## Next Steps

Now that you have a working example, you can:

- Add more routes to handle different endpoints
- Connect to Azure services (Blob Storage, Cosmos DB, etc.)
- Deploy to Azure
- Add authentication and authorization

## Acknowledgments & Tech Stack

This project leverages the following high-performance foundations:

* **[Warp](https://github.com/seanmonstar/warp)** - A super-easy, composable web server framework for warp speeds.
* **[Tokio](https://github.com/tokio-rs/tokio)** - The event-driven, non-blocking I/O platform for Rust.
* **[Azure Functions Custom Handlers](https://docs.microsoft.com/azure/azure-functions/functions-custom-handlers)** - The official Microsoft specification for custom runtimes.
* **[Azure Function Custom Hanlers rust](https://docs.microsoft.com/azure/azure-functions/create-first-function-vs-code-other)** - The official documentation specific to implementation in all supported languages.

---

## License

MIT
